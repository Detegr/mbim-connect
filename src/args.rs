use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "mbim-connect",
    about = "Helper tool for connecting to LTE network with mbimcli"
)]
pub struct Args {
    #[structopt(
        short = "d",
        long = "device",
        help = "Device to use for MBIM connection",
        default_value = "/dev/cdc-wdm0"
    )]
    pub mbim_device: String,
    #[structopt(
        short = "w",
        long = "wwan_device",
        help = "Device to assign the IP address to",
        default_value = "wwp0s20f0u6"
    )]
    pub wwan_device: String,
    #[structopt(
        short = "v",
        long = "verbose",
        help = "Print commands before running them"
    )]
    pub verbose: bool,
    #[structopt(
        short = "n",
        long = "dns-to-stderr",
        help = "Output DNS servers to stderr to allow processing them in an easy way by piping stderr to a script"
    )]
    pub dns_to_stderr: bool,
}
