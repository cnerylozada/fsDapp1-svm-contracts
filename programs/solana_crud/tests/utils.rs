use anchor_lang::Key;
use anchor_lang::{system_program, InstructionData};
use borsh::BorshDeserialize;
use litesvm::{types::TransactionResult, LiteSVM};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use solana_transaction::Transaction;

pub struct CreateInput {
    pub title: String,
    pub message: String,
}

#[derive(Debug, BorshDeserialize)]
pub struct MessageAccount {
    pub user: Pubkey,
    pub title: String,
    pub message: String,
    pub bump: u8,
}

pub fn setup_svm() -> (LiteSVM, Keypair) {
    let mut svm = LiteSVM::new();
    let program_id = solana_crud::ID;
    let program_bytes = include_bytes!("../../../target/deploy/solana_crud.so");
    svm.add_program(program_id, program_bytes);

    let signer = Keypair::new();
    svm.airdrop(&signer.pubkey(), 10_000_000_000).unwrap();

    (svm, signer)
}

pub fn get_message_account_pda(user: Pubkey, title: &String) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"message_account", user.key().as_ref(), title.as_bytes()],
        &solana_crud::ID,
    )
}

pub fn create_tx(
    svm: &mut LiteSVM,
    signer: &Keypair,
    message_account_pda: Pubkey,
    inputs: &CreateInput,
) -> TransactionResult {
    let create_ix = Instruction {
        program_id: solana_crud::ID,
        accounts: vec![
            AccountMeta::new(message_account_pda, false),
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: solana_crud::instruction::Create {
            _title: inputs.title.clone(),
            _message: inputs.message.clone(),
        }
        .data(),
    };
    let create_tx = Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&signer.pubkey()),
        &[signer],
        svm.latest_blockhash(),
    );

    svm.send_transaction(create_tx)
}
