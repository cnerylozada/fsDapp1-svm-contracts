use anchor_lang::InstructionData;
use borsh::BorshDeserialize;
use litesvm::{types::TransactionResult, LiteSVM};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use solana_transaction::Transaction;

mod utils;
use solana_crud;
use utils::{create_tx, get_message_account_pda, setup_svm, CreateInput, MessageAccount};

struct UpdateInput {
    title: String,
    message: String,
}

fn update_tx(
    svm: &mut LiteSVM,
    signer: &Keypair,
    message_account_pda: Pubkey,
    inputs: &UpdateInput,
) -> TransactionResult {
    let update_ix = Instruction {
        program_id: solana_crud::ID,
        accounts: vec![
            AccountMeta::new(message_account_pda, false),
            AccountMeta::new(signer.pubkey(), true),
        ],
        data: solana_crud::instruction::Update {
            _title: inputs.title.clone(),
            _message: inputs.message.clone(),
        }
        .data(),
    };
    let update_tx = Transaction::new_signed_with_payer(
        &[update_ix],
        Some(&signer.pubkey()),
        &[&signer],
        svm.latest_blockhash(),
    );
    svm.send_transaction(update_tx)
}

#[test]
fn update_test() {
    let (mut svm, signer) = setup_svm();

    let create_tx_inputs = CreateInput {
        title: "my title".to_string(),
        message: "my message".to_string(),
    };

    let (message_account_pda, _) =
        get_message_account_pda(signer.pubkey(), &create_tx_inputs.title);
    let create_tx_result = create_tx(&mut svm, &signer, message_account_pda, &create_tx_inputs);
    assert_eq!(create_tx_result.is_ok(), true);

    let update_tx_inputs = UpdateInput {
        title: create_tx_inputs.title.clone(),
        message: "my new message".to_string(),
    };

    let update_tx_result = update_tx(&mut svm, &signer, message_account_pda, &update_tx_inputs);
    assert_eq!(update_tx_result.is_ok(), true);

    let message_account_raw = svm.get_account(&message_account_pda).unwrap();
    let message_account = MessageAccount::deserialize(&mut &message_account_raw.data[8..]).unwrap();

    assert_eq!(message_account.user, signer.pubkey());
    assert_eq!(message_account.title, update_tx_inputs.title);
    assert_eq!(message_account.message, update_tx_inputs.message);
}
