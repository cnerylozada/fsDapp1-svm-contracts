use anchor_lang::InstructionData;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::Signer,
};
use solana_transaction::Transaction;

mod utils;
use solana_crud;
use utils::{create_tx, get_message_account_pda, setup_svm, CreateInput};

#[test]
fn delete_test() {
    let (mut svm, signer) = setup_svm();

    let inputs = CreateInput {
        title: "my title".to_string(),
        message: "a message".to_string(),
    };

    let (message_account_pda, _) = get_message_account_pda(signer.pubkey(), &inputs.title);
    let create_tx_result = create_tx(&mut svm, &signer, message_account_pda, &inputs);
    assert_eq!(create_tx_result.is_ok(), true);

    let delete_ix = Instruction {
        program_id: solana_crud::ID,
        accounts: vec![
            AccountMeta::new(message_account_pda, false),
            AccountMeta::new(signer.pubkey(), true),
        ],
        data: solana_crud::instruction::Delete {
            _title: inputs.title,
        }
        .data(),
    };
    let delete_tx = Transaction::new_signed_with_payer(
        &[delete_ix],
        Some(&signer.pubkey()),
        &[&signer],
        svm.latest_blockhash(),
    );

    let delete_tx_result = svm.send_transaction(delete_tx);
    assert_eq!(delete_tx_result.is_ok(), true);

    let message_account_raw = svm.get_account(&message_account_pda);
    assert_eq!(message_account_raw.is_none(), true);
}
