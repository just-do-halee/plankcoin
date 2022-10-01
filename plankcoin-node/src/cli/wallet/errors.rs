use super::*;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Bincode error")]
    Bincode(#[from] bincode::Error),
    #[error("Invalid passphrase")]
    Encrypt(aes_gcm::Error),
    #[error("Invalid passphrase")]
    Decrypt(aes_gcm::Error),
    #[error("The root wallet file is not a valid wallet file")]
    InvalidRootWallet,
}
