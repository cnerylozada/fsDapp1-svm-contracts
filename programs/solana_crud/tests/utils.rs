use anchor_lang::Key;
use litesvm::LiteSVM;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

pub fn setup_svm() -> (LiteSVM, Pubkey, Keypair) {
    let mut svm = LiteSVM::new();
    let program_id = solana_crud::id();
    let program_bytes = include_bytes!("../../../target/deploy/solana_crud.so");
    svm.add_program(program_id, program_bytes);

    let signer = Keypair::new();
    svm.airdrop(&signer.pubkey(), 10_000_000_000).unwrap();

    (svm, program_id, signer)
}

pub fn get_message_account_pda(
    signer: &Pubkey,
    title: &String,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"message_account", signer.key().as_ref(), title.as_ref()],
        program_id,
    )
}
