use std::env;
use std::process::exit;

mod fb_code;
mod thinbasic_script;

fn main() {
    println!("thinClippy - thinBASIC code analyzer");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please supply thinBasic script file name as parameter");
        exit(1);
    }

    let main_file_name = (&args[1]).to_string();
    println!("Analyzing {}...\n", main_file_name);

    let mut code = thinbasic_script::Code::new(main_file_name);

    let mut issues_found = 0;

    if fb_code::analysis_available(&mut code) {
        match fb_code::pairs_match(&mut code) {
            Ok(()) => (),
            Err(v) => {
                issues_found += 1;
                println!("{}", v)
            }
        };
    } else {
        println!("No #fbCode section, skipping part of analysis...")
    }

    println!("\nAnalysis finished, {} issue(s) found.", issues_found);
    if issues_found > 0 {
        exit(2)
    }
}
