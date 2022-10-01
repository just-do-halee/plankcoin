use super::*;

mod cmn;

use cmn::*;

#[derive(Debug)]
pub struct Wallet {
    pub path: PathBuf,
    pub root: Account,
}

impl Wallet {
    /// Create a new wallet with a random key
    #[inline]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            root: Default::default(),
        }
    }

    /// Read wallet from a file
    ///
    /// If the file does not exist, create a new wallet
    /// and save it to the file.
    ///
    pub fn read_from_terminal(mut term: &Term, path: &Path) -> AnyResult<Wallet> {
        info!("Finding wallet...");

        let path_display = path.display();

        // check if wallet exists
        if !path.exists() {
            writeln!(
                term,
                "The wallet file {} not found, do you want to create it? [y/n]",
                path_display
            )?;

            match term_get_char(term, is_y_or_n)? {
                Y => {
                    writeln!(term, "Creating wallet file {}", path_display)?;

                    let passphrase = term_read_wallet_passphrase(term)?;

                    Wallet::new(path).try_write(passphrase, WriteMode::CreateNew)?;

                    info!("Wallet file created successfully");
                }
                N => {
                    log_bail!("The root wallet does not exist");
                }
                _ => unreachable!(),
            }
        }
        info!("Wallet file {} found", path_display);

        let passphrase = term_read_wallet_passphrase(term)?;

        info!("Reading wallet file...");
        let wallet = Wallet::try_read(path, passphrase)?;

        Ok(wallet)
    }

    /// Create a new wallet reading from the given path and passphrase
    pub fn try_read(
        path: impl Into<PathBuf>,
        passphrase: [u8; AES_KEY_SIZE],
    ) -> Result<Self, Error> {
        let path = path.into();
        let bytes = Self::try_read_encrypted_file(&path)?;
        Self::decrypt(&bytes, passphrase, path)
    }
    /// Create a new wallet file at the given passhprase and write mode
    pub fn try_write(&self, passphrase: [u8; AES_KEY_SIZE], mode: WriteMode) -> Result<(), Error> {
        if mode == WriteMode::CreateNew && self.path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "The wallet file already exists",
            )
            .into());
        }

        let encrypted_bytes = self.encrypt(passphrase)?;

        debug!("Opening wallet file for writing: {}", self.path.display());
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;

        debug!("Writing wallet file...");

        debug!("Writing wallet check bytes {}", WALLET_CHECK_BYTES.to_hex());
        file.write_all(&WALLET_CHECK_BYTES)?;

        debug!(
            "Writing wallet encrypted bytes {}",
            encrypted_bytes.to_hex()
        );
        file.write_all(&encrypted_bytes)?;
        Ok(())
    }
    /// Encrypt the wallet with the given passphrase
    pub fn encrypt(&self, passphrase: [u8; AES_KEY_SIZE]) -> Result<Vec<u8>, Error> {
        debug!("Creating Aes256Gcm cipher key...");
        let cipher = Aes256Gcm::new(passphrase.as_ref().into());

        debug!("Generating random nonce...");
        // generate a random nonce
        let mut nonce_bytes = [0u8; AES_NONCE_SIZE];
        OsRng.fill_bytes(&mut nonce_bytes);

        let nonce = Nonce::from_slice(nonce_bytes.as_ref());

        debug!("Serializing the account...");
        let ciphertext = bincode::serialize(&self.root)?;

        debug!("Encrypting the account...");
        debug!("Nonce: {}", nonce.to_hex());
        debug!("Ciphertext: {}", ciphertext.to_hex());
        let encrypted_bytes = cipher
            .encrypt(nonce, ciphertext.as_ref())
            .map_err(Error::Encrypt)?;

        let mut result = nonce.to_vec();
        result.extend(encrypted_bytes);
        Ok(result)
    }
    /// Decrypt the bytes with the given passphrase
    pub fn decrypt(
        bytes: &[u8],
        passphrase: [u8; AES_KEY_SIZE],
        path: impl Into<PathBuf>,
    ) -> Result<Self, Error> {
        let path = path.into();

        debug!(
            "Checking byte length... {} < nonce({})",
            bytes.len(),
            AES_NONCE_SIZE
        );
        if bytes.len() < AES_NONCE_SIZE {
            return Err(Error::InvalidRootWallet);
        }

        debug!("Creating Aes256Gcm cipher key...");
        let cipher = Aes256Gcm::new(passphrase.as_ref().into());

        debug!("Taking the nonce bytes...");
        let nonce = Nonce::from_slice(&bytes[..AES_NONCE_SIZE]);
        let ciphertext = &bytes[AES_NONCE_SIZE..];

        debug!("Decrypting the account...");
        debug!("Nonce: {}", nonce.to_hex());
        debug!("Ciphertext: {}", ciphertext.to_hex());
        let decrypted_bytes = cipher.decrypt(nonce, ciphertext).map_err(Error::Decrypt)?;

        debug!("Deserializing the account...");
        let root = bincode::deserialize(decrypted_bytes.as_slice())?;

        Ok(Self { path, root })
    }

    // ---------------------------------------------------------------------------------------------

    /// Returns the rest of the bytes without the wallet check bytes
    #[inline]
    fn try_read_encrypted_file(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
        Self::_try_read_encrypted_file(path.as_ref())
    }
    fn _try_read_encrypted_file(path: &Path) -> Result<Vec<u8>, Error> {
        let mut file = fs::File::open(path)?;

        let mut wallet_check_bytes = [0u8; WALLET_CHECK_BYTES.len()];

        debug!("Reading wallet check bytes");
        // read the first 2 bytes to check if the file is a valid wallet file
        {
            file.read_exact(&mut wallet_check_bytes).map_err(|e| {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    Error::InvalidRootWallet
                } else {
                    Error::Io(e)
                }
            })?;
            if wallet_check_bytes[..WALLET_CHECK_BYTES.len()] != WALLET_CHECK_BYTES {
                return Err(Error::InvalidRootWallet);
            }
        }

        debug!("Reading encrypted bytes");
        // read the rest of the file
        let mut rest = Vec::new();
        file.read_to_end(&mut rest)?;

        // check if the file is a encrypted wallet file
        if rest.len() < AES_NONCE_SIZE {
            return Err(Error::InvalidRootWallet);
        }

        debug!("The encrypted bytes: {}", rest.to_hex());
        // return the rest of the file without the check bytes
        Ok(rest)
    }
}
