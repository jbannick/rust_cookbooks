# Logging

This Cookbook shows one way to implement Logging in a Rust app.  

It uses the log4rs crate.  
it uses an optional logging-config.yaml file in its working directory.  
It defaults to: stdout, Info, Warn, Error (no Debug).  

It demonstrates deep logging.  

[See: https://docs.rs/log4rs/latest/log4rs/index.html](https://docs.rs/log4rs/latest/log4rs/index.html])  
[See: https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html](https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html])  

Usage:  
cargo run --example logging  
