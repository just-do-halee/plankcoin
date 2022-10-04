use super::*;

pub mod msg_version;

pub use msg_version::*;

pub enum Msg {
    Version(MsgVersion),
}

impl Msg {
    pub fn into_byte_iter(self) -> impl Iterator<Item = u8> {
        match self {
            Msg::Version(MsgVersion(
                magic,
                version,
                services,
                timestamp,
                addr_recv,
                addr_from,
                nonce,
                height,
            )) => magic.to_bytes().into_iter().chain(
                version.bits().to_bytes().into_iter().chain(
                    services.bits().to_bytes().into_iter().chain(
                        timestamp.into_now().to_bytes().into_iter().chain(
                            addr_recv.to_bytes().into_iter().chain(
                                addr_from.to_bytes().into_iter().chain(
                                    nonce
                                        .into_gen()
                                        .to_bytes()
                                        .into_iter()
                                        .chain(height.to_bytes()),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        }
    }
}
