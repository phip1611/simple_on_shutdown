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

//! This crate consists of a convenient macro to specify on shutdown callbacks called [`on_shutdown`].
//! It takes code that should be executed when your program exits (gracefully).
//!
//! Internally it creates a closure that gets executed when the context gets dropped, i.e. when
//! `main()` exits. There is also [`on_shutdown_move`] available in case the closure needs to capture
//! vars, like an `Arc<>`.
//!
//! In theory this macro can be used everywhere where the context gets dropped. But it has a nice
//! expressive name so that one exactly knows what it should achieve in code. A good example
//! is the `main()` function in an `actix-web`-Server. For example you want to log to a file
//! when the server was shut down.
//!
//! There is no guarantee that this gets executed during "non-regular" shutdown scenarios,
//! like when receiving `CTRL+C / SIGINT / SIGTERM`. This depends on whether your application
//! properly handles signals and if the operating system gives your application time before it gets
//! totally killed/stopped.
//!
//! **IMPORTANT**: Use this on the top level of your main() or whatever your current runtimes main
//! function is! The code gets executed when the context it lives in gets dropped.
//! This macro can be called multiple times (at least with stable Rust 1.48.0) without problems.
//!
//! This crate uses the `log` crate on the `debug` level.

use log::debug;
use std::time::Instant;

/// Simple type that holds a closure (callback). The closure gets invoked during `drop()`.
/// This works also fine with applications that do gracefully shutdown via signals, like SIGTERM.
///
/// Create this type via `on_shutdown!(println!("foobar"))` or `on_shutdown!({e1; e2; e3; println!("foobar")})`.
/// See [`on_shutdown`] and [`on_shutdown_move`] for more info.
///
/// This crate uses the `log` crate on the `debug` level.
///
/// IMPORTANT: Use this on the top level of your main() or whatever your current runtimes main
/// function is! The code gets executed when the context it lives in gets dropped.
/// This can be called multiple times (at least with stable Rust 1.48.0) without problem.
///
/// ## Example:
/// ```
/// use simple_on_shutdown::on_shutdown;
///
/// fn main() {
///     // some code ...
///
///     // Important that the returned value of the macro lives through
///     // the whole lifetime of main(). It gets dropped in the end.
///     on_shutdown!(println!("shut down with success"));
///
///     // can be used multiple times
///     on_shutdown!(println!("shut down with success"));
///     on_shutdown!({println!("blocks also work")});
///     // some code ...
/// }
/// ```
pub struct OnShutdownCallback(Box<dyn FnMut()>);

impl OnShutdownCallback {
    /// Constructor. Used by [`on_shutdown`] and [`on_shutdown_move`].
    // THIS MUST BE PUBLIC, OTHERWISE THE MACROS DO NOT WORK!
    pub fn new(inner: Box<dyn FnMut()>) -> Self {
        Self(inner)
    }
}
impl Drop for OnShutdownCallback {
    /// Executes the specified callback.
    fn drop(&mut self) {
        debug!("on shutdown callback:");
        let now = Instant::now();
        (self.0)();
        let duration_usecs = now.elapsed().as_micros();
        let duration_secs = duration_usecs as f64 / 1_000_000_f64;
        debug!(
            "on shutdown callback finished: took {}s ({}µs)",
            duration_secs, duration_usecs
        );
    }
}

/// This crate consists of a convenient macro to specify on shutdown callbacks called [`on_shutdown`].
/// It takes code that should be executed when your program exits (gracefully).
///
/// Internally it creates a closure that gets executed when the context gets dropped, i.e. when
/// `main()` exits. There is also [`on_shutdown_move`] available in case the closure needs to capture
/// vars, like an `Arc<>`.
///
/// In theory this macro can be used everywhere where the context gets dropped. But it has a nice
/// expressive name so that one exactly knows what it should achieve in code. A good example
/// is the `main()` function in an `actix-web`-Server. For example you want to log to a file
/// when the server was shut down.
///
/// There is no guarantee that this gets executed during "non-regular" shutdown scenarios,
/// like when receiving `CTRL+C / SIGINT / SIGTERM`. This depends on whether your application
/// properly handles signals and if the operating system gives your application time before it gets
/// totally killed/stopped..
///
/// **IMPORTANT**: Use this on the top level of your main() or whatever your current runtimes main
/// function is! The code gets executed when the context it lives in gets dropped.
/// This macro can be called multiple times (at least with stable Rust 1.48.0) without problems.
///
/// This crate uses the `log` crate on the `debug` level.
///
/// Also see [`on_shutdown_move`].
///
/// ## Example
/// ```
/// use simple_on_shutdown::on_shutdown;
///
/// fn main() {
///     // some code ...
///
///     // Important that the returned value of the macro lives through
///     // the whole lifetime of main(). It gets dropped in the end.
///     on_shutdown!(println!("shut down with success"));
///
///     // can be used multiple times
///     on_shutdown!(println!("shut down with success"));
///     on_shutdown!({println!("blocks also work")});
///     // some code ...
/// }
/// ```
#[macro_export]
macro_rules! on_shutdown {
    ($cb:block) => {
        // Some unique name that the programmer will never use inside his application.
        // It's okay if this var exists multiple times if the programmer uses the macro
        // multiple times. Because two values may have the same identifier in rustlang
        // but internally they are two different values.
        let _on_shutdown_callback_1337deadbeeffoobaraffecoffee = $crate::OnShutdownCallback::new(
            // put closure on heap
            Box::new(
                // closure has zero parameters
                || {
                    // the actual code
                    $cb
                },
            ),
        );
    };
    // recursive mapping to block
    ($cb:expr) => {
        on_shutdown!({ $cb })
    };
}

/// Like [`on_shutdown`] but moves all variables into the created closure.
/// ## Example
/// ```
/// use std::sync::atomic::AtomicBool;
/// use std::sync::Arc;
/// use simple_on_shutdown::on_shutdown_move;
///
/// fn main() {
///     // some code ...
///     let foobar = Arc::new(AtomicBool::new(false));
///     // on_shutdown!() would not work here because the closure needs "foobar"
///     on_shutdown_move!({
///         foobar.store(true, Ordering::Relaxed);
///         println!("foobar={}", foobar.load(Ordering::Relaxed));
///     });
///     // or just:
///     // on_shutdown_move!(foobar.store(true, Ordering::Relaxed));
/// }
/// ```
#[macro_export]
macro_rules! on_shutdown_move {
    ($cb:block) => {
        // Some unique name that the programmer will never use inside his application.
        // It's okay if this var exists multiple times if the programmer uses the macro
        // multiple times. Because two values may have the same identifier in rustlang
        // but internally they are two different values.
        let _on_shutdown_callback_1337deadbeeffoobaraffecoffee = $crate::OnShutdownCallback::new(
            // put closure on heap
            Box::new(
                // closure has zero parameters; moves variables into closure
                move || {
                    // the actual code
                    $cb
                },
            ),
        );
    };
    // recursive mapping to block
    ($cb:expr) => {
        on_shutdown_move!({ $cb })
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
        on_shutdown_move!({
            foobar_c.store(true, Ordering::Relaxed);
            println!("foobar={}", foobar_c.load(Ordering::Relaxed));
        });
        // or just:
        on_shutdown_move!(foobar.store(true, Ordering::Relaxed));
    }
}
