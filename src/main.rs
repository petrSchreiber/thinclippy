use std::env;
use std::process::exit;

fn main() {
    println!("thinClippy - thinBASIC code analyzer");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please supply thinBasic script file name as parameter");
        exit(1);
    }
}
