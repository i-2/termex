#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate termex_api;
extern crate docopt;
extern crate base64;
extern crate env_logger;

use docopt::Docopt;
use std::io;
use std::process::exit;
use base64::{encode, decode};
use termex_api::key::{Key, Encryptor};
use termex_api::SERVICE_NAME;
use termex_api::vault::Vault;
use termex_api::endpoint::TermexClient;
use std::process::Command;

fn whoami() -> String {
   Command::new("whoami").output()
   .and_then(|out| String::from_utf8(out.stdout).map_err(|e| io::Error::from(io::ErrorKind::Other)))
   .unwrap_or("root".to_string())
}

const HELP : &'static str = "
Termex Sync

Usage:
  termex_sync <command>

Options:
  -h --help     show help
";

#[derive(Debug, Deserialize)]
struct Args{
    arg_command: Option<String>
}

fn main() {
    env_logger::init();
    // main args
    let args: Args = Docopt::new(HELP).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());
    if let Some(command) = args.arg_command {
        //sync new command
        let user = whoami();
        let vault = Vault::new(&user);
        if !vault.exists() {
            println!("Please try 'termex_cli login'");
            exit(1);
        }
        let token = vault.get_token().expect("Please do 'termex_cli login' before syncing command");
        if let Ok(key_string) = vault.get() {
            let key_decode = decode(&key_string).unwrap();
            let key = Key::from_pem_string(key_decode).expect("Invalid Key");
            let msg_bytes = key.encrypt(command.into_bytes()).expect("Unable to encrypt");
            let msg_string = String::from_utf8(msg_bytes).expect("Invalid blob");
            let client = TermexClient::new(token);
            match client.new_blob(msg_string) {
                Ok(_) => exit(0),
                Err(_) => exit(1)
            };
        } else {
            println!("No Secure Key present, do a 'termex_cli login'");
            exit(1);
        }
    }
}
