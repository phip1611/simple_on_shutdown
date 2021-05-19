# simple_on_shutdown

This crate consists of a convenient macro to specify on shutdown callbacks called `on_shutdown!`. It takes code that
should be executed when your program exits (gracefully). See https://docs.rs/simple_on_shutdown for more info.

Useful with *"runtimes you do not have control over"*, like for example actix-web framework doesn't let you specify 
shutdown callbacks by yourself. In such cases my macro may be a better option.

## Usage

#### Recommended

```rust
use simple_on_shutdown::on_shutdown;

fn main() {
    // macro can take: direct expression
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

```

## Examples
See ["examples/"-dir in repository!](https://github.com/phip1611/simple_on_shutdown/examples).


### ⚠ Restrictions ⚠

- There is no guarantee that this gets executed in "non-regular" shutdown scenarios, like
  `CTRL+C / SIGINT / SIGTERM`
- your application must handle `SIGINT/SIGTERM` (and other signals) properly to allow a gracefully "non-regular"
  shutdown (Actix web framework does this for example)
    - i.e. if you don't handle signals `CTRL+C` will immediately shut down your app
- Even in that case: there is no guarantee in every case that the operating system gives your application more time
  after it has been (forcefully) killed
- this behaviour differs a little between Windows and UNIX. See `example/src/bin/simple_example_ctrl_c_signal` for
  more details.