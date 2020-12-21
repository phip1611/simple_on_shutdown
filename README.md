# simple_on_shutdown

This crate consists of a convenient macro to specify on shutdown callbacks called `on_shutdown!`. It takes code that
should be executed when your program exits (gracefully). 

Internally it creates a closure that gets executed when the context gets dropped, i.e. when
`main()` exits. There is also `on_shutdown_move!` available in case the closure needs to capture vars, like an `Arc<>`.

In theory this macro can be used everywhere where the context gets dropped. But it has a nice expressive name so that
one exactly knows what it should achieve in code. A good example is the `main()` function in an `actix-web`-Server. For
example you want to log to a file when the server was shut down.

There is no guarantee that this gets executed during "non-regular" shutdown scenarios, like when
receiving `CTRL+C / SIGINT / SIGTERM`. This depends on whether your application properly handles signals and if the
operating system gives your application time before it gets totally killed/stopped.

**IMPORTANT**: Use this on the top level of your main() or whatever your current runtimes main function is! The code
gets executed when the context it lives in gets dropped. This macro can be called multiple times (at least with stable
Rust 1.48.0) without problems.

This crate uses the `log` crate on the `debug` level.

*With "runtimes you do not have control over" I mean that for example actix doesn't let you specify shutdown callbacks
by itself. In such cases my macro may be a better option.*

## Usage

#### Recommended

```rust
use simple_on_shutdown::{on_shutdown, on_shutdown_move};

// ...
fn main() {
    on_shutdown!(println!("shutted down"));

    // If you need to move a variable into the closure
    let foobar = Arc::new(AtomicBool::new(false));
    on_shutdown_move!({
        foobar.store(true, Ordering::Relaxed);
        println!("foobar={}", foobar.load(Ordering::Relaxed));
  });
}
```

#### Not recommended, old way

```rust
// Not recommended, old way
#[macro_use]
extern crate simple_on_shutdown;

// ...
fn main() {
    on_shutdown!(println!("shutted down"));
}
```

## Actix Web Server

*See also "example/"-dir in repository!*

```rust
use actix_web::{get, HttpServer, App, HttpResponse, Responder};
use simple_on_shutdown::on_shutdown;

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Important that the returned value of the macro lives through
    // the whole lifetime of main(). It gets dropped in the end.
    on_shutdown!({
                // the actual code
                println!("This gets executed during shutdown. Don't do expensive operations here.");
    });

    HttpServer::new(|| {
        App::new()
            .service(get_index)
    })
        .bind(format!("localhost:{}", 8080))?
        .run()
        .await
}
```

### More examples
See `example/`-directory.

### ⚠ Restrictions ⚠

- There is no guarantee that this gets executed in "non-regular" shutdown scenarios, like
  `CTRL+C / SIGINT / SIGTERM`
- your application must handle `SIGINT/SIGTERM` (and other signals) in a proper way to allow a gracefully "non-regular"
  shutdown (Actix web framework does this for example)
    - i.e. if you don't handle signals `CTRL+C` will immediately shut down your app
- but even in that case: there is no guarantee in every case that the operating system gives your application more time
  after it has been (forcefully) killed
- this behaviour differs a little bit between Windows and UNIX. See `example/src/bin/simple_example_ctrl_c_signal` for
  more details.