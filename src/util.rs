use crate::args::Args;
use crate::ip_configuration::IpConfiguration;
use crate::parser;
use failure::{format_err, Error};
use std::ffi::OsStr;
use std::io;
use std::os::unix::process::CommandExt;
use std::process;
use std::process::Command;

pub fn run_as_root<I: IntoIterator<Item = S> + Clone, S: AsRef<OsStr>>(
    cmd: &str,
    args: I,
    verbose: bool,
) -> Result<process::Output, Error> {
    if verbose {
        let args_copy = args.clone();
        print!("{} ", cmd);
        println!(
            "{}",
            args_copy
                .into_iter()
                .map(|s| s.as_ref().to_string_lossy().to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
    Command::new(cmd)
        .uid(0)
        .gid(0)
        .args(args)
        .output()
        .map_err(|e| {
            if e.kind() == io::ErrorKind::PermissionDenied {
                format_err!("Permission denied, please run this program as root.")
            } else {
                e.into()
            }
        })
}

pub fn run_str_as_root(command_line: &str, verbose: bool) -> Result<process::Output, Error> {
    let mut words = command_line.split(" ");
    let cmd = words.nth(0);
    if let Some(cmd) = cmd {
        let output = run_as_root(cmd, words, verbose)?;
        if !output.status.success() {
            return Err(format_err!("{}", String::from_utf8_lossy(&output.stderr)));
        }
        if verbose {
            print_stdout_and_stderr(&output);
        }
        Ok(output)
    } else {
        Err(format_err!("Empty command line"))
    }
}

pub fn print_stdout_and_stderr(output: &process::Output) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.len() > 0 {
        println!("{}", stdout);
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.len() > 0 {
        println!("{}", stderr);
    }
}

pub fn mbimcli_run_sequentally<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(
    args: I,
    process_args: &Args,
) -> Result<IpConfiguration, Error> {
    #[derive(Debug)]
    enum Parsed {
        IpConfiguration(IpConfiguration),
        TRID(u64),
    }
    args.into_iter()
        .fold(Ok(Parsed::TRID(0)), |acc, arg| {
            let output = match acc? {
                Parsed::TRID(0) => {
                    let mbimcli_args = &[
                        arg.as_ref(),
                        OsStr::new("--no-close"),
                        OsStr::new("-d"),
                        OsStr::new(&process_args.mbim_device),
                    ];
                    run_as_root("mbimcli", mbimcli_args, process_args.verbose)?
                }
                Parsed::TRID(trid) => {
                    let no_open = format!("--no-open={}", trid);
                    let mbimcli_args = &[
                        arg.as_ref(),
                        OsStr::new(&no_open),
                        OsStr::new("--no-close"),
                        OsStr::new("-d"),
                        OsStr::new(&process_args.mbim_device),
                    ];
                    run_as_root("mbimcli", mbimcli_args, process_args.verbose)?
                }
                conf => return Ok(conf),
            };
            if !output.status.success() {
                return Err(format_err!("{}", String::from_utf8_lossy(&output.stderr)));
            }
            if process_args.verbose {
                print_stdout_and_stderr(&output);
            }
            match parser::parse_ip_configuration(&output.stdout[..]) {
                Ok(conf) => Ok(Parsed::IpConfiguration(conf)),
                Err(_) => {
                    let trid = parser::parse_trid(&output.stdout[..])?;
                    Ok(Parsed::TRID(trid))
                }
            }
        })
        .and_then(|parsed| {
            if let Parsed::IpConfiguration(conf) = parsed {
                Ok(conf)
            } else {
                Err(format_err!("Could not parse IP configuration"))
            }
        })
}
