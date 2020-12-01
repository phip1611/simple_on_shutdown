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