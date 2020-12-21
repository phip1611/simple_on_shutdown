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

use simple_on_shutdown::on_shutdown_move;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::spawn;

/// Simple example that shows how you can move variables
/// into the callback.
fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let exit_work_loop = Arc::new(AtomicBool::new(false));
    let exit_work_loop_t = exit_work_loop.clone();
    let thread_h = spawn(move || {
        loop {
            if exit_work_loop_t.load(Ordering::Relaxed) {
                break;
            }
        }
        42
    });

    // on_shutdown!() would not work here because the closure needs "foobar"
    on_shutdown_move!({
        exit_work_loop.store(true, Ordering::Relaxed);
        let thread_res = thread_h.join().unwrap();
        println!("thread result={}", thread_res);
    });
    // or just:
    // on_shutdown_move!(foobar.store(true, Ordering::Relaxed));
}
