use super::*;

pub mod cmn;
pub mod msg;
pub mod netconfig;

pub use netconfig::*;

use cmn::*;
use msg::*;

pub struct Net(pub Netconfig);

impl Net {
    #[inline]
    pub fn getaddrs_from_seeds() -> Vec<SocketAddr> {
        DNS_SEEDS
            .iter()
            .flat_map(|&domain| {
                dns_lookup::lookup_host(domain)
                    .unwrap()
                    .into_iter()
                    .map(|ip| (ip, PORT).into())
            })
            .collect()
    }

    #[inline]
    pub fn try_handshake(
        &self,
        vault_height: impl Into<Height>,
        to: impl Into<SocketAddr>,
    ) -> io::Result<()> {
        let vault_height = vault_height.into();
        let to = to.into();

        let magic: Magic = self.0.network.into();
        let version = self.0.version;
        let services = self.0.me.services;
        let timestamp = VaultTime::new();
        let addr_recv = to;
        let addr_from = self.0.me.address;
        let nonce = Nonce::new();

        debug!("try_handshake: to={}", to);
        self.send_msg(
            MsgVersion(
                magic,
                version,
                services,
                timestamp,
                addr_recv,
                addr_from,
                nonce,
                vault_height,
            ),
            Peer::new(ServiceFlags::NONE, to),
        )?;

        todo!("Rest of handshake")
    }

    pub fn send_msg(&self, msg: impl Into<Msg>, to: impl Into<Peer>) -> io::Result<()> {
        let to = to.into();
        let msg = msg.into();
        let mut stream = TcpStream::connect(to.address)?;

        match msg {
            Msg::Version(_) => {
                let mut buf = [0u8; MsgVersion::SIZE];

                msg.into_byte_iter().enumerate().for_each(|(i, byte)| {
                    buf[i] = byte;
                });

                stream.write_all(&buf)
            }
        }
    }
}
