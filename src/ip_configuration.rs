use std::iter;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

macro_rules! impl_debug_for_ipaddr_with_mask(($iptype:ty) => {
    impl ::std::fmt::Debug for $iptype {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "{}/{}", self.addr, self.mask)
        }
    }
});

#[derive(PartialEq)]
pub struct IpAddrWithMask {
    pub addr: IpAddr,
    pub mask: u8,
}

#[derive(PartialEq)]
pub struct Ipv4AddrWithMask {
    pub addr: Ipv4Addr,
    pub mask: u8,
}

#[derive(PartialEq)]
pub struct Ipv6AddrWithMask {
    pub addr: Ipv6Addr,
    pub mask: u8,
}

impl_debug_for_ipaddr_with_mask!(IpAddrWithMask);
impl_debug_for_ipaddr_with_mask!(Ipv4AddrWithMask);
impl_debug_for_ipaddr_with_mask!(Ipv6AddrWithMask);

#[derive(Debug)]
pub struct Ipv4Configuration {
    pub ip: Vec<Ipv4AddrWithMask>,
    pub gw: Ipv4Addr,
    pub dns: Vec<Ipv4Addr>,
}

#[derive(Debug)]
pub struct Ipv6Configuration {
    pub ip: Vec<Ipv6AddrWithMask>,
    pub gw: Ipv6Addr,
    pub dns: Vec<Ipv6Addr>,
}

#[derive(Debug)]
pub struct IpConfiguration {
    pub ipv4: Ipv4Configuration,
    pub ipv6: Ipv6Configuration,
    pub trid: u64,
}

#[derive(Debug, PartialEq)]
pub enum ConfigEntry {
    Ip(IpAddrWithMask),
    Gw(IpAddr),
    Dns(IpAddr),
    TRID(u64),
    NoValue,
}

impl iter::FromIterator<ConfigEntry> for IpConfiguration {
    fn from_iter<T: IntoIterator<Item = ConfigEntry>>(iter: T) -> Self {
        let mut ip4 = vec![];
        let mut gw4 = Ipv4Addr::from(0);
        let mut dns4 = vec![];
        let mut ip6 = vec![];
        let mut gw6 = Ipv6Addr::from(0);
        let mut dns6 = vec![];
        let mut trid = 0;

        for item in iter {
            match item {
                ConfigEntry::Ip(IpAddrWithMask {
                    addr: IpAddr::V4(addr),
                    mask,
                }) => {
                    ip4.push(Ipv4AddrWithMask { addr, mask });
                }
                ConfigEntry::Ip(IpAddrWithMask {
                    addr: IpAddr::V6(addr),
                    mask,
                }) => {
                    ip6.push(Ipv6AddrWithMask { addr, mask });
                }
                ConfigEntry::Gw(IpAddr::V4(addr)) => {
                    gw4 = addr;
                }
                ConfigEntry::Gw(IpAddr::V6(addr)) => {
                    gw6 = addr;
                }
                ConfigEntry::Dns(IpAddr::V4(addr)) => {
                    dns4.push(addr);
                }
                ConfigEntry::Dns(IpAddr::V6(addr)) => {
                    dns6.push(addr);
                }
                ConfigEntry::TRID(parsed_trid) => {
                    trid = parsed_trid;
                }
                ConfigEntry::NoValue => continue,
            }
        }

        IpConfiguration {
            ipv4: Ipv4Configuration {
                ip: ip4,
                gw: gw4,
                dns: dns4,
            },
            ipv6: Ipv6Configuration {
                ip: ip6,
                gw: gw6,
                dns: dns6,
            },
            trid,
        }
    }
}
