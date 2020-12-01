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

//! This crate consists of a convenient macro to specify on shutdown callbacks.
//! This is useful for all runtimes you do not have control over.
//!
//! The generated main() function of an "actix" web server is a good example. With the
//! exported macro [`on_shutdown`] you can easily specify code, that should run during program
//! termination.
//!
//! IMPORTANT: Use this on the top level of your main() or whatever your current runtimes main
//! function is! The code gets executed when the context it lives in gets dropped.
//! This can be called multiple times (at least with stable Rust 1.48.0) without problem.

use std::time::Instant;

#[macro_use]
extern crate log;

/// Simple type that holds an closure (callback). The closure gets invoked during `drop()`.
/// This works also fine with applications that do gracefully shutdown via signals, like SIGTERM.
///
/// Create this type via `on_shutdown!(println!("foobar"))` or `on_shutdown!({e1; e2; e3; println!("foobar")})`.
/// See [`on_shutdown`] for more info.
///
/// IMPORTANT: Use this on the top level of your main() or whatever your current runtimes main
/// function is! The code gets executed when the context it lives in gets dropped.
/// This can be called multiple times (at least with stable Rust 1.48.0) without problem.
pub struct ShutdownCallbackDummy(Box<dyn FnMut()>);
impl ShutdownCallbackDummy {
    /// Constructor. Better use macro [`on_shutdown`].
    pub fn new(inner: Box<dyn FnMut()>) -> Self {
        Self(inner)
    }
}
impl Drop for ShutdownCallbackDummy {
    /// Executes the specified callback.
    fn drop(&mut self) {
        debug!("on shutdown callback:");
        let now = Instant::now();
        (self.0)();
        let duration_usecs = now.elapsed().as_micros();
        let duration_secs = duration_usecs as f64 / 1_000_000_f64;
        debug!("on shutdown callback finished: took {}s ({}µs)", duration_secs, duration_usecs);
    }
}

/// Convenient constructor macro for [`ShutdownCallbackDummy`]. Pass in an expression
/// or a block of code you want to be executed during shutdown.
/// IMPORTANT: Use this on the top level of your main() or whatever your current runtimes main
/// function is! The code gets executed when the context it lives in gets dropped.
/// This can be called multiple times (at least with stable Rust 1.48.0) without problem.
#[macro_export]
macro_rules! on_shutdown {
    ($cb:block) => {
        // Some unique name that the programmer will never use inside his application.
        // It's okay if this var exists multiple times if the programmer uses the macro
        // multiple times. Because two values may have the same identifier in rustlang
        // but internally they are two different values.
        let _aafuhaifhabfa252axc3xvcxwqdagteafeerqav = $crate::ShutdownCallbackDummy::new(
            // put closure on heap
            Box::new(
                // closure has zero parameters
                || {
                    // the actual code
                    $cb
                }
            )
        );
    };
    // recursive mapping to block
    ($cb:expr) => { on_shutdown!({$cb}) };
}


/// A test works if after executing it you can see the shutdown action in the output.
#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_simple() {
        on_shutdown!(println!("shut down with success"));
        println!("registered on_shutdown");
        sleep(Duration::from_secs(1));
        println!("waited 1 second");
    }
    #[test]
    fn test_block() {
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
