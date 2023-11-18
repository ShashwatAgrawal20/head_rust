use std::env;
use std::process;

use head_rust::ArgsVal;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_val = ArgsVal::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = head_rust::run(arg_val) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
