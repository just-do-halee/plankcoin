use super::*;

pub struct Netconfig {
    pub network: Network,
    pub version: ProtocolVersion,
    pub me: Peer,
}

impl Netconfig {
    #[inline]
    pub fn new(
        network: impl Into<Network>,
        version: impl Into<ProtocolVersion>,
        me: impl Into<Peer>,
    ) -> Self {
        Netconfig {
            network: network.into(),
            version: version.into(),
            me: me.into(),
        }
    }
}
