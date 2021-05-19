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
