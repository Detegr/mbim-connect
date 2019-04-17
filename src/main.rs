#[macro_use]
extern crate nom;

mod args;
mod ip_configuration;
mod parser;
mod util;

use args::Args;
use failure::Error;
use structopt::StructOpt;
use util::*;

fn connect_to_network(args: &Args) -> Result<(), Error> {
    mbimcli_run_sequentally(
        &[
            "--query-subscriber-ready-status",
            "--query-registration-state",
            "--attach-packet-service",
            "--connect=apn='internet',ip-type=ipv4",
            "--query-ip-configuration",
        ],
        args,
    )
    .and_then(|conf| {
        run_str_as_root(
            &format!("ip addr flush dev {}", args.wwan_device),
            args.verbose,
        )?;

        run_str_as_root(
            &format!("ip link set {} up", args.wwan_device),
            args.verbose,
        )?;

        for ip in &conf.ipv4.ip {
            run_str_as_root(
                &format!(
                    "ip addr add {}/{} dev {}",
                    ip.addr, ip.mask, args.wwan_device
                ),
                args.verbose,
            )?;
        }

        run_str_as_root(
            &format!(
                "ip route add default via {} dev {} src {}",
                conf.ipv4.gw, args.wwan_device, conf.ipv4.ip[0].addr
            ),
            args.verbose,
        )
        .map(|_| conf)
    })
    .and_then(|conf| {
        println!("Connected to network. Ip address(es): {:?}", conf.ipv4.ip);
        if args.dns_to_stderr {
            for dns in conf.ipv4.dns {
                eprintln!("{}", dns);
            }
        }
        Ok(())
    })
}

fn main() {
    let args = Args::from_args();
    if let Err(e) = connect_to_network(&args) {
        println!("Could not connect to network.\n{}", e);
    }
}
