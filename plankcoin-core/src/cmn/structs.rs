use super::*;

pub struct VaultTime;

impl VaultTime {
    #[inline]
    pub fn now() -> u64 {
        Utc::now().timestamp_millis() as u64
    }
}
