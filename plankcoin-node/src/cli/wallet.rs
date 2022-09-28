use super::*;

#[derive(Debug, Error)]
enum Error {
    #[error("The root wallet file does not exist")]
    RootWalletDoesNotExist,
}

pub fn read(wallet_path: &Path) -> AnyResult<()> {
    info!("Finding root wallet...");

    let wallet_path_display = wallet_path.display();

    // check if wallet exists
    if !wallet_path.exists() {
        info!(
            "The wallet file {} not found, do you want to create it? [y/n]",
            wallet_path_display
        );

        match stdin_get_char(is_y_or_n)? {
            'y' => {
                info!("Creating wallet file {}", wallet_path_display);
                let mut file = fs::File::create(&wallet_path)?;
                file.write_all(b"")?;
                info!("Wallet file created successfully");
            }
            'n' => {
                let reason = Error::RootWalletDoesNotExist;
                error!("{reason}");
                bail!(reason);
            }
            _ => unreachable!(),
        }
    }

    info!("Wallet file {} found", wallet_path_display);
    Ok(())
}
