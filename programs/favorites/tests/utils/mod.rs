use borsh::BorshDeserialize;
use litesvm::LiteSVM;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

#[derive(Debug, BorshDeserialize)]
pub struct Favorites {
    pub number: u64,
    pub color: String,
    pub hobbies: Vec<String>,
}

pub fn setup_svm() -> (LiteSVM, Pubkey, Keypair) {
    let mut svm = LiteSVM::new();
    let program_id = Pubkey::from(favorites::ID);
    let program_bytes = include_bytes!("../../../../target/deploy/favorites.so");
    svm.add_program(program_id, program_bytes);

    let signer = Keypair::new();
    svm.airdrop(&signer.pubkey(), 10_000_000_000).unwrap();

    (svm, program_id, signer)
}

pub fn get_favorites_pda(signer: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"favorites", signer.as_ref()], program_id)
}
