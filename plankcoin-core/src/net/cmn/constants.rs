/// The IpAddr type represents either an IPv4 or an IPv6 address.
/// So it is 16 bytes long.
pub const IP_SIZE: usize = 16;
/// The port size is 2 bytes(16 bits).
pub const PORT_SIZE: usize = 2;
/// IP(16 bytes) + PORT(2 bytes)
pub const ADDRESS_SIZE: usize = IP_SIZE + PORT_SIZE;

// ------------------------------------------------

/// The domains that have the approved bank node list.
pub const DNS_SEEDS: &[&str] = &["plank.crabdance.com"];
/// Plankcoin port
pub const PORT: u16 = 6626;
