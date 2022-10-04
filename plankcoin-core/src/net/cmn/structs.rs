use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Peer {
    pub services: ServiceFlags,
    pub address: SocketAddr,
}
impl From<(ServiceFlags, SocketAddr)> for Peer {
    #[inline]
    fn from((services, address): (ServiceFlags, SocketAddr)) -> Self {
        Peer { services, address }
    }
}

impl Peer {
    #[inline]
    pub fn new(services: impl Into<ServiceFlags>, address: impl Into<SocketAddr>) -> Self {
        Peer {
            services: services.into(),
            address: address.into(),
        }
    }
}
