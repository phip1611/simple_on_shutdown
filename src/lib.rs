/*
MIT License

Copyright (c) 2021 Philipp Schuster

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
//! This crate consists of a convenient macro to specify on shutdown callbacks called [`on_shutdown`].
//! It takes code that should be executed when your program exits (gracefully).
//!
//! Internally it creates a `FnOnce`-closure that gets executed when the context gets dropped.
//! This macro can be called multiple times (at least with stable Rust 1.48.0) without problems.
//!
//! In theory this macro can be used everywhere where the context gets dropped. But it has a nice
//! expressive name so that one exactly knows what it should achieve in code. A good example
//! is the `main()` function in an `actix-web`-Server or a `tokio` runtime. For example you want
//! to log to a file when the server was shut down.
//!
//! There is no guarantee that this gets executed during "non-regular" shutdown scenarios,
//! like when receiving `CTRL+C / SIGINT / SIGTERM`. This depends on whether your application
//! properly handles signals and if the operating system gives the application time before it gets
//! totally killed/stopped.

#![cfg_attr(not(test), no_std)]

#[cfg(not(test))]
extern crate alloc;
#[cfg(not(test))]
use alloc::boxed::Box;

/// PRIVATE! Use [`on_shutdown`].
///
/// Simple type that holds a `FnOnce`-closure (callback). The `FnOnce`-closure gets invoked during `drop()`.
/// This works also fine with applications that do gracefully shutdown via signals, like SIGTERM.
pub struct OnShutdownCallback(Option<Box<dyn FnOnce()>>);

impl OnShutdownCallback {
    /// Constructor. Used by [`on_shutdown`].
    ///
    /// ## Parameters
    /// * `cb` boxed(heap) callback function
    ///
    // THIS MUST BE PUBLIC, OTHERWISE THE MACROS DO NOT WORK!
    pub fn new(cb: Box<dyn FnOnce()>) -> Self {
        Self(Some(cb))
    }
}

impl Drop for OnShutdownCallback {
    /// Executes the specified callback.
    fn drop(&mut self) {
        // take(): because I use a FnOnce here, I need to own the value
        // in order for it to get executed.
        (self.0.take().unwrap())();
    }
}

/// This crate consists of a convenient macro to specify on shutdown callbacks called [`on_shutdown`].
/// It takes code that should be executed when your program exits (gracefully).
///
/// Internally it creates a `FnOnce`-closure that gets executed when the context gets dropped.
/// This macro can be called multiple times (at least with stable Rust 1.48.0) without problems.
///
/// In theory this macro can be used everywhere where the context gets dropped. But it has a nice
/// expressive name so that one exactly knows what it should achieve in code. A good example
/// is the `main()` function in an `actix-web`-Server or a `tokio` runtime. For example you want
/// to log to a file when the server was shut down.
///
/// There is no guarantee that this gets executed during "non-regular" shutdown scenarios,
/// like when receiving `CTRL+C / SIGINT / SIGTERM`. This depends on whether your application
/// properly handles signals and if the operating system gives the application time before it gets
/// totally killed/stopped.
///
/// ## Example
/// ```
/// use simple_on_shutdown::on_shutdown;
///
/// fn main() {
///     // direct expression
///     on_shutdown!(println!("shut down with success"));
///     // closure expression
///     on_shutdown!(|| println!("shut down with success"));
///     // move closure expression
///     on_shutdown!(move || println!("shut down with success"));
///     // block
///     on_shutdown!({println!("shut down with success")});
///     // identifier
///     let identifier = || println!("shut down with success");
///     on_shutdown!(identifier);
/// }
/// ```
#[macro_export]
macro_rules! on_shutdown {
    // a identifier that must point to a valid closure
    ($closure:ident) => {
        // Some unique name that a programmer will never use inside their application.
        // It's okay if this var exists multiple times if the programmer uses the macro
        // multiple times. Because two values may have the same identifier in rustlang
        // but internally they are two different values (you can see this in debugger).
        let _on_shutdown_callback_1337deadbeeffoobaraffecoffee =
            $crate::OnShutdownCallback::new(Box::new($closure));
    };
    // move closure expression
    (move || $cb:expr) => {
        let closure = move || $cb;
        on_shutdown!(closure);
    };
    // closure expression
    (|| $cb:expr) => {
        let closure = || $cb;
        on_shutdown!(closure);
    };
    ($cb:expr) => {
        let closure = || $cb;
        on_shutdown!(closure);
    };
    ($cb:block) => {
        let closure = || $cb;
        on_shutdown!(closure);
    };
}

/// A test works if after executing it you can see the shutdown action in the output.
#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_macro_compilation() {
        // direct expression
        on_shutdown!(println!("shut down with success"));
        // closure expression
        on_shutdown!(|| println!("shut down with success"));
        // move closure expression
        on_shutdown!(move || println!("shut down with success"));
        // block
        on_shutdown!({ println!("shut down with success") });
        // identifier
        let identifier = || println!("shut down with success");
        on_shutdown!(identifier);
    }

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

    #[test]
    fn test_move() {
        let foobar = Arc::new(AtomicBool::new(false));
        let foobar_c = foobar.clone();
        //on_shutdown!() would not work here because the closure needs "foobar"
        on_shutdown!(move || {
            foobar_c.store(true, Ordering::Relaxed);
            println!("foobar={}", foobar_c.load(Ordering::Relaxed));
        });
    }
}
