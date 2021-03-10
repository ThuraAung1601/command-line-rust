extern crate tail;
use std::process;

fn main() {
    let config = match tail::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = tail::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
