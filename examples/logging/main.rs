//! This Cookbook shows one way to implement Logging in a Rust app.
//!
//! It uses the log4rs crate.
//! it uses an optional logging-config.yaml file in its working directory.
//! It defaults to: stdout, Info, Warn, Error (no Debug).
//!
//! It demonstrates deep logging.
//!
//! See: https://docs.rs/log4rs/latest/log4rs/index.html
//! See: https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
//!
//! Usage:
//!   cargo run --example logging
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
use std::{io};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    Config,
};
use log::LevelFilter;
use log::{debug, error, info, trace, warn};

fn configure_logging() {
    const LOGGING_CONGFIG_FILE: &str = "logging-config.yaml";
    log4rs::init_file(
        LOGGING_CONGFIG_FILE,
        log4rs::config::Deserializers::default(),
    )
    .unwrap_or_else(|e| {
        match e.downcast_ref::<io::Error>() {
            Some(os_err) if os_err.kind() == io::ErrorKind::NotFound => {
                println!("No {} file found. Defaulting to: stdput, Info, Warn, Error.", 
                    LOGGING_CONGFIG_FILE);
                let stdout = ConsoleAppender::builder().build();
                let conf = Config::builder()
                    .appender(Appender::builder().build("stdout", Box::new(stdout)))
                    .build(Root::builder().appender("stdout").build(LevelFilter::Info))
                    .unwrap();
                log4rs::init_config(conf).unwrap();
            }
            other_error => panic!("Logging config problem {other_error:?}"),
        }
    });
}

#[derive(Debug)]
struct Structure(i32);
  
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("Logging Cookbook");

    configure_logging();
    
    info!("Structure: {:?}", Structure(3).0);
    info!("Deep: {:?}", Deep(Structure(0)).0);
    debug!("Deep: {:?}", Deep(Structure(7)));
    trace!("Deep: {:?}", Deep(Structure(77)));
    warn!("Deep: {:?}", Deep(Structure(999)));    
    error!("Deep: {:?}", Deep(Structure(666)));
}
