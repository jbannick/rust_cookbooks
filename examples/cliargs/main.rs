//! This Cookbook shows one way to implement command line arguments in a Rust app.
//!
//! It demonstrates the use of enums obtained from the command line args.
//!
//! It uses the structopt crate for command line parsing.
//! Which is built upon the clap crate.
//!
//! It also uses the strum crate for enum serialization.
//!
//! See: https://crates.io/crates/structopt
//! See: https://crates.io/crates/strum
//! See: https://doc.rust-lang.org/rust-by-example/std_misc/arg.html
//!
//! Usage:
//!   cargo run --example cliargs
//!   cargo run --example cliargs -- --servertype [pub | sib] 
//!   cargo run --example cliargs -- --help 
//!
//! Note: the extra "--" string is required when running from Cargo.
//!
//! MIT License
//!
//! Copyright (c) 2024 John Bannick
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
//!
use std::fmt;
use structopt::StructOpt;
use strum::VariantNames;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(long, possible_values = ServerType::VARIANTS)]
    servertype: ServerType,
}

#[derive(Debug, strum::EnumString, strum::EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
enum ServerType {
    Pub,
    Sub,
}

impl fmt::Display for ServerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerType::Pub => write!(f, "pub"),
            ServerType::Sub => write!(f, "sub"),
        }
    }
}

fn main() {
    println!("Command Line Arguments Cookbook");
  
    let args = Args::from_args();
    println!("args: {:?}", args);
    
    let arg = args.servertype;
    println!("arg: {:?}", arg);
    
    let server_type = arg.to_string();   
    println!("server_type: {:?}", server_type);

}
