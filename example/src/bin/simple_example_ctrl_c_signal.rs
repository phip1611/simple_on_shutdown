/*
MIT License

Copyright (c) 2020 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
//! This example shows you how you can use [`simple_on_shutdown::on_shutdown`] to work
//! with SIGNALS, like when pressing CTRL+C.

use simple_on_shutdown::on_shutdown;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// This example shows you how you can use [`simple_on_shutdown::on_shutdown`] to work
/// with SIGNALS, like when pressing CTRL+C.
fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let do_work = Arc::new(AtomicBool::new(true));
    let do_work_handler = do_work.clone();

    ctrlc::set_handler(move || {
        println!("Received CTRL+C");
        do_work_handler.store(false, Ordering::Relaxed);
    })
    .unwrap();
    on_shutdown!({
        println!("THIS IS REALTED TO \"ctrlc\" crate and has nothing to do with `simple_on_shutdown`:");
        println!("- On UNIX this gets executed when SIGINT/SIGTERM is received.");
        println!("- On Windows this gets executed when SIGINT-equivalent is received but probably not when the application gets killed the hard way.");
    });
    println!("Stop me with CTRL+C or kill me with another method");

    // Start work loop
    loop {
        if !do_work.load(Ordering::Relaxed) {
            println!("Exiting work loop");
            break;
        }
    }
}
