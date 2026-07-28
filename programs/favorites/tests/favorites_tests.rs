use anchor_lang::{system_program, InstructionData};
use borsh::BorshDeserialize;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::signature::Signer;
use solana_transaction::Transaction;
mod utils;
use utils::{get_favorites_pda, setup_svm, Favorites};

struct SetFavoritesInput {
    pub number: u64,
    pub color: String,
    pub hobbies: Vec<String>,
}

#[test]
fn set_favorites() {
    let (mut svm, program_id, signer) = setup_svm();

    let (favorites_pda, _) = get_favorites_pda(&signer.pubkey(), &program_id);

    let inputs = SetFavoritesInput {
        number: 7,
        color: "red".to_string(),
        hobbies: vec!["coding".to_string()],
    };

    let set_favorites_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(favorites_pda, false),
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: favorites::instruction::SetFavorites {
            _number: inputs.number,
            _color: inputs.color.clone(),
            _hobbies: inputs.hobbies.clone(),
        }
        .data(),
    };
    let set_favorites_tx = Transaction::new_signed_with_payer(
        &[set_favorites_ix],
        Some(&signer.pubkey()),
        &[&signer],
        svm.latest_blockhash(),
    );
    svm.send_transaction(set_favorites_tx);

    let favorites_account_raw = svm.get_account(&favorites_pda).unwrap();
    let favorites_account = Favorites::deserialize(&mut &favorites_account_raw.data[8..]).unwrap();

    assert_eq!(favorites_account.number, inputs.number);
    assert_eq!(favorites_account.color, inputs.color);
}
