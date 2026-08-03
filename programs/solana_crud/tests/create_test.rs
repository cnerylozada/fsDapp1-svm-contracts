use anchor_lang::Key;
use borsh::BorshDeserialize;
use solana_sdk::signature::Signer;

mod utils;
use utils::{create_tx, get_message_account_pda, setup_svm, CreateInput, MessageAccount};

#[test]
fn create_test() {
    let (mut svm, signer) = setup_svm();

    let inputs = CreateInput {
        title: "title1".to_string(),
        message: "message1".to_string(),
    };

    let (message_account_pda, bump) = get_message_account_pda(signer.pubkey(), &inputs.title);
    let create_tx_result = create_tx(&mut svm, &signer, message_account_pda, &inputs);
    assert_eq!(create_tx_result.is_ok(), true);

    let message_account_raw = svm.get_account(&message_account_pda).unwrap();
    let message_account = MessageAccount::deserialize(&mut &message_account_raw.data[8..]).unwrap();

    assert_eq!(message_account.user.key(), signer.pubkey());
    assert_eq!(message_account.title, inputs.title);
    assert_eq!(message_account.message, inputs.message);
    assert_eq!(message_account.bump, bump);
}

#[test]
fn create_failed_test() {
    let (mut svm, signer) = setup_svm();

    let inputs = CreateInput {
        title: "".to_string(),
        message: "".to_string(),
    };

    let (message_account_pda, _) = get_message_account_pda(signer.pubkey(), &inputs.title);
    let create_tx_result = create_tx(&mut svm, &signer, message_account_pda, &inputs);
    assert_eq!(create_tx_result.is_err(), true);

    let error = create_tx_result.unwrap_err();
    assert_eq!(
        error
            .meta
            .logs
            .iter()
            .any(|item| item.contains("Title is required")),
        true
    );
}
