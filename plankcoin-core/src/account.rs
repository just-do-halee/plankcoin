use super::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub keypair: Keypair,
}

impl Default for Account {
    #[inline]
    fn default() -> Self {
        Self {
            keypair: Keypair::generate(&mut OsRng),
        }
    }
}
