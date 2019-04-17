use crate::ip_configuration::*;
use failure::{format_err, Error};
use std::io::{BufRead, BufReader, Read};
use std::net::IpAddr;

named!(contents<&str, &str>, do_parse!(
    take_until!("\'") >>
    c: delimited!(char!('\''), is_not!("\'"), char!('\'')) >>
    (c)
));

named!(name<&str, &str>, map!(take_while!(|c| c != ':'), |s| s.trim().split(" ").nth(0).unwrap()));

fn config_from_contents(name: &str, contents: &str) -> Result<ConfigEntry, Error> {
    match name {
        "IP" => {
            let mask_separator = contents.find('/').unwrap_or(0);
            let mask = contents[mask_separator + 1..].parse::<u8>()?;
            let addr = contents[..mask_separator].parse::<IpAddr>()?;
            Ok(ConfigEntry::Ip(IpAddrWithMask { addr, mask })
        }
        "Gateway" => {
            let addr = contents.parse::<IpAddr>()?;
            Ok(ConfigEntry::Gw(addr))
        }
        "DNS" => {
            let addr = contents.parse::<IpAddr>()?;
            Ok(ConfigEntry::Dns(addr))
        }
        "TRID" => Ok(ConfigEntry::TRID(contents.parse::<u64>()?)),
        _ => Ok(ConfigEntry::NoValue),
    }
}
named!(config_entry<&str, ConfigEntry>,
       map_res!(
           do_parse!(
               name: name >>
               contents: contents >>
               (name, contents)
            ), |(name, contents)| config_from_contents(name, contents)
        )
);

pub fn parse_ip_configuration<T: Read>(input: T) -> Result<IpConfiguration, Error> {
    let bufread = BufReader::new(input);
    let lines = bufread
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>();

    let entries = lines
        .iter()
        .filter_map(|line| {
            let entry = config_entry(line).ok().map(|(_, entry)| entry);
            if let Some(ConfigEntry::NoValue) = entry {
                None
            } else {
                entry
            }
        })
        .collect::<Vec<ConfigEntry>>();
    if entries.len() <= 1 {
        Err(format_err!("Ip configuration entries not found"))
    } else {
        Ok(entries.into_iter().collect::<IpConfiguration>())
    }
}

pub fn parse_trid<T: Read>(input: T) -> Result<u64, Error> {
    let bufread = BufReader::new(input);
    let lines = bufread
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>();

    lines
        .iter()
        .filter_map(|line| {
            let entry = config_entry(line).ok().map(|(_, entry)| entry);
            if let Some(ConfigEntry::TRID(trid)) = entry {
                Some(trid)
            } else {
                None
            }
        })
        .nth(0)
        .ok_or(format_err!("Could not parse TRID"))
}

#[test]
fn parser() {
    use std::net::{Ipv4Addr, Ipv6Addr};

    let input = r#"[/dev/cdc-wdm0] IPv4 configuration available: 'address, gateway, dns'
     IP [0]: '111.22.3.100/16'
    Gateway: '111.22.3.1'
    DNS [0]: '8.8.8.8'
    DNS [1]: '8.8.6.6'

[/dev/cdc-wdm0] IPv6 configuration available: 'address, gateway, dns'
     IP [0]: 'fe80::1234:4321:1234:4321/120'
    Gateway: 'fe80::1234:4321:1234:1'
    DNS [0]: '4321:4321:1234::1'
    DNS [1]: '4321:4321:1234::2'
[/dev/cdc-wdm0] Session not closed:
	    TRID: '8'"#;

    let trid = parse_trid(input.clone().as_bytes()).unwrap();
    assert_eq!(trid, 8);

    let config = parse_ip_configuration(input.as_bytes()).unwrap();
    assert_eq!(config.ipv4.ip[0].addr, Ipv4Addr::new(111, 22, 3, 100));
    assert_eq!(config.ipv4.ip[0].mask, 16);
    assert_eq!(config.ipv4.gw, Ipv4Addr::new(111, 22, 3, 1));
    assert_eq!(config.ipv4.dns[0], Ipv4Addr::new(8, 8, 8, 8));
    assert_eq!(config.ipv4.dns[1], Ipv4Addr::new(8, 8, 6, 6));
    assert_eq!(config.ipv6.ip[0].addr, Ipv6Addr::new(0xfe80, 0x0, 0x0, 0x0, 0x1234, 0x4321, 0x1234, 0x4321));
    assert_eq!(config.ipv6.ip[0].mask, 120);
    assert_eq!(config.ipv6.gw, Ipv6Addr::new(0xfe80, 0x0, 0x0, 0x0, 0x1234, 0x4321, 0x1234, 0x1));
    assert_eq!(config.ipv6.dns[0], Ipv6Addr::new(0x4321, 0x4321, 0x1234, 0x0, 0x0, 0x0, 0x0, 0x1));
    assert_eq!(config.ipv6.dns[1], Ipv6Addr::new(0x4321, 0x4321, 0x1234, 0x0, 0x0, 0x0, 0x0, 0x2));
    assert_eq!(config.trid, 8);
}
