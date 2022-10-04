use super::*;

pub struct MsgVersion(
    pub Magic,
    pub ProtocolVersion,
    pub ServiceFlags,
    pub VaultTime,
    pub SocketAddr,
    pub SocketAddr,
    pub Nonce<u64>,
    pub Height,
);
impl MsgVersion {
    pub const SIZE: usize = Magic::SIZE
        + ProtocolVersion::SIZE
        + ServiceFlags::SIZE
        + VaultTime::SIZE
        + ADDRESS_SIZE * 2
        + Nonce::<u64>::SIZE
        + Height::SIZE;
}

impl From<MsgVersion> for Msg {
    #[inline]
    fn from(msg: MsgVersion) -> Self {
        Msg::Version(msg)
    }
}
