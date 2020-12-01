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

use simple_on_shutdown::on_shutdown;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    println!("================ test binary ================");
    // on_shutdown!(println!("shut down with success"));
    on_shutdown!({
            println!("shut");
            println!("down");
            println!("with");
            println!("success");
        });
    println!("registered on_shutdown");
    sleep(Duration::from_secs(1));
    println!("waited 1 second");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        println!("================ test simple ================");
        on_shutdown!(println!("shut down with success"));
        println!("registered on_shutdown");
        sleep(Duration::from_secs(1));
        println!("waited 1 second");
    }
    #[test]
    fn test_block() {
        println!("================ test block ================");
        on_shutdown!({
            println!("shut");
            println!("down");
            println!("with");
            println!("success");
        });
        println!("registered on_shutdown");
        sleep(Duration::from_secs(1));
        println!("waited 1 second");
    }
}