# simple_on_shutdown
This Rust library consists of a convenient macro to specify on shutdown callbacks.
This is useful for all runtimes you do not have control over.

The generated main() function of an "actix" web server is a good example. With the 
exported macro `on_shutdown!()` you can easily specify code, that should run during program
termination/shutdown.

**IMPORTANT**: Use this on the top level of your main() or whatever your current runtimes main
function is! The code gets executed when the context it lives in gets dropped.
This can be called multiple times (at least with stable Rust 1.48.0) without problem.

## Usage
#### Recommended
```rust
use simple_on_shutdown::on_shutdown;
// ...
on_shutdown!(println!("shutted down"));
```
#### Not recommended, old way
```rust
// Not recommended, old way
#[macro_use]
extern crate simple_on_shutdown;
// ...
on_shutdown!(println!("shutted down"));
```


## Simple Example
*See also "example/"-dir in repository!*
```rust
use simple_on_shutdown::on_shutdown;

fn main() {
    // some code ...
    
    // Important that the returned value of the macro lives through
    // the whole lifetime of main(). It gets dropped in the end.
    on_shutdown!(println!("shut down with success"));
    // some code ...
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
                println!("This gets executed during shutdown. Don't to expensive operations here.");
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