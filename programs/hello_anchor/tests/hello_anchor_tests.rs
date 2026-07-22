use anchor_lang::{system_program, InstructionData};
use borsh::BorshDeserialize;
use hello_anchor;
use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use solana_transaction::Transaction;

#[derive(Debug, BorshDeserialize)]
struct NewAccount {
    pub name: String,
    pub age: u64,
    pub likes: u64,
}

#[test]
fn initialize_account() {
    // Initialize the test environment
    let mut svm = LiteSVM::new();

    // Deploy your program to the test environment
    let program_id = Pubkey::from(hello_anchor::ID);
    let program_bytes = include_bytes!("../../../target/deploy/hello_anchor.so");
    svm.add_program(program_id, program_bytes);

    let new_account = Keypair::new();

    let signer = Keypair::new();
    svm.airdrop(&signer.pubkey(), 10_000_000_000).unwrap();

    let _name = String::from("cristh");
    let _age = 33;

    let initialize_account_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(new_account.pubkey(), true),
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: hello_anchor::instruction::InitializeAccount {
            _name: _name.clone(),
            _age,
        }
        .data(),
    };
    let initialize_account_tx = Transaction::new_signed_with_payer(
        &[initialize_account_ix],
        Some(&signer.pubkey()),
        &[&signer, &new_account],
        svm.latest_blockhash(),
    );
    svm.send_transaction(initialize_account_tx).unwrap();

    let account = svm.get_account(&new_account.pubkey()).unwrap();
    let new_account = NewAccount::deserialize(&mut &account.data[8..]).unwrap();

    assert_eq!(new_account.name, _name);
    assert_eq!(new_account.age, _age);
}
