#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate docopt;
extern crate termex_api;

pub mod select;

use docopt::Docopt;
use std::io;
use std::process::{exit, Command};
use termex_api::endpoint::TermexClient;
use termex_api::vault::Vault;

const HELP: &'static str = "
Termex Import

Usage:
  termex_import past <num>

Options:
  --version     version
  -h --help     show help
";

fn whoami() -> String {
    Command::new("whoami")
        .output()
        .and_then(|out| {
            String::from_utf8(out.stdout)
                .map_err(|_e| io::Error::from(io::ErrorKind::Other))
        })
        .unwrap_or("root".to_string())
}

#[derive(Debug, Deserialize)]
struct Args {
    cmd_past: bool,
    arg_num: u32,
}

fn main() {
    let args: Args = Docopt::new(HELP)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    let user = whoami();
    let vault = Vault::new(&user);
    let token = vault
        .get_token()
        .expect("Please do 'termex_cli login' before syncing command");
    if !vault.exists() {
        println!("Please try 'termex_cli login'");
        exit(1);
    }
    let client = TermexClient::new(token);
    let range = select::PastDays::past(args.arg_num);
    let output = client.dump(range.0, range.1);
}
