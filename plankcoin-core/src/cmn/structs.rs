use super::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct VaultTime;

impl VaultTime {
    pub const SIZE: usize = u64::BITS as usize;

    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
    #[inline]
    pub fn now() -> u64 {
        Utc::now().timestamp_millis() as u64
    }
    #[inline]
    pub fn into_now(self) -> u64 {
        Self::now()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Nonce<T>(PhantomData<T>);

impl Nonce<u64> {
    pub const SIZE: usize = u64::BITS as usize;
}

impl<T> Nonce<T>
where
    Standard: Distribution<T>,
{
    #[inline]
    pub fn new() -> Self {
        Nonce(PhantomData)
    }
    #[inline]
    pub fn gen(&self) -> T {
        rand::random()
    }
    #[inline]
    pub fn into_gen(self) -> T {
        rand::random()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Height(pub u64);

impl Height {
    pub const SIZE: usize = u64::BITS as usize;
}

impl Deref for Height {
    type Target = u64;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for Height {
    #[inline]
    fn from(height: u64) -> Self {
        Height(height)
    }
}
