#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate termex_api;
extern crate docopt;
extern crate rpassword;
extern crate base64;
extern crate env_logger;

mod login;
mod signup;

use std::io;
use std::io::Write;
use std::process::Command;
use std::process::exit;
use docopt::Docopt;
use base64::{encode, decode};
use rpassword::read_password;
use termex_api::vault::Vault;
use termex_api::key::Key;

pub enum UserLookUpError {
    UserError,
    InvalidName
}

pub fn whoami() -> Result<String, UserLookUpError> {
    Command::new("whoami")
    .output()
    .map_err(|_| UserLookUpError::UserError)
    .and_then(|out| String::from_utf8(out.stdout)
    .map_err(|_| UserLookUpError::InvalidName))
}

pub fn username() -> String {
    match whoami() {
        Ok(s) => s,
        Err(_) => "root".to_owned()
    }
}

const HELP : &'static str = "
Termex Cli

Usage:
  termex_cli login
  termex_cli signup

Options:
  --version     version
  -h --help     show help
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_login: bool,
    cmd_signup: bool
}

fn main() {
    env_logger::init();
    let system_user: String = username();
    let input_stream: io::Stdin = io::stdin();
    let mut output_stream : io::Stdout = io::stdout();
    let vault : Vault = Vault::new(&system_user[..]);

    // parse the arguments
    let args: Args = Docopt::new(HELP)
                     .and_then(|d| d.deserialize())
                     .unwrap_or_else(|e| e.exit());
    // check if it what command.
    if(args.cmd_login) {
        println!("Enter your termex username: ");
        let mut username = String::new();
        input_stream.read_line(&mut username);
        username = username.trim().to_owned();
        println!("Enter your termex password: ");
        let mut password = read_password().unwrap();
        output_stream.flush();
        let token = match login::login(username, password) {
            Ok(st) => st,
            Err(_) => {
                println!("Invalid Password");
                exit(1);
            }
        };
        vault.set_token(token);
        if(!vault.exists()){
            let private_key = Key::generate(256);
            let result = match private_key {
                Ok(key) => key.to_pem_string(),
                Err(_) => {
                    println!("Cannot genrate secure keypair..");
                    exit(1);
                } 
            };
            match result {
                Ok(key_str) => vault.set(encode(key_str.as_bytes())),
                Err(_) => {
                    println!("Unable to save the key");
                    exit(1);
                }
            };
        }
        exit(0);
    }

    if (args.cmd_signup) {
        // command for signup..
        println!("Enter you termex username: ");
        let mut username = String::new();
        input_stream.read_line(&mut username);
        username = username.trim().to_owned();
        println!("Enter your termex password: ");
        let mut password = read_password().unwrap();
        output_stream.flush();
        let res_sign = signup::signup(username, password);
        match res_sign {
            Ok(_) => println!("Signup Succeded"),
            Err(e) => {
                debug!("{:?}", e);
                println!("Signup Failed!");
                exit(1)
            } 
        }
        exit(0)
    }
}
