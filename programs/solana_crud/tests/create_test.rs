use anchor_lang::{system_program, InstructionData, Key};
use borsh::BorshDeserialize;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
};
use solana_transaction::Transaction;

mod utils;
use solana_crud;
use utils::{get_message_account_pda, setup_svm};

pub struct CreateInput {
    title: String,
    message: String,
}

#[derive(Debug, BorshDeserialize)]
pub struct MessageAccount {
    pub user: Pubkey,
    pub title: String,
    pub message: String,
    pub bump: u8,
}

#[test]
fn create_test() {
    let (mut svm, program_id, signer) = setup_svm();

    let inputs = CreateInput {
        title: "title1".to_string(),
        message: "message1".to_string(),
    };

    let (message_account_pda, bump) =
        get_message_account_pda(&signer.pubkey(), &inputs.title, &program_id);

    let create_ix = Instruction {
        program_id,
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
        &[&signer],
        svm.latest_blockhash(),
    );
    svm.send_transaction(create_tx);

    let message_account_raw = svm.get_account(&message_account_pda).unwrap();
    let message_account = MessageAccount::deserialize(&mut &message_account_raw.data[8..]).unwrap();

    assert_eq!(message_account.user.key(), signer.pubkey());
    assert_eq!(message_account.title, inputs.title);
    assert_eq!(message_account.message, inputs.message);
    assert_eq!(message_account.bump, bump);
}
