use std::env;
use std::process::exit;

mod compile;
mod thinbasic_script;
mod tokenizer;

fn main() {
    println!("thinClippy - thinBASIC code analyzer, v0.1");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("INPUT ERROR: Please supply thinBasic script file name as parameter");
        exit(1);
    }

    let main_file_name = (&args[1]).to_string();
    println!("Analyzing {}...\n", main_file_name);

    let mut code = match thinbasic_script::Code::new(main_file_name) {
        Ok(c) => c,
        Err(e) => {
            println!("INPUT ERROR: {}", e);
            exit(1)
        }
    };

    let mut issues_found = 0;

    if compile::analysis_available(&mut code) {
        match compile::pairs_match(&mut code) {
            Ok(()) => (),
            Err(v) => {
                issues_found += 1;
                println!("{}", v)
            }
        };
    } else {
        println!("No #compile section, skipping part of analysis...")
    }

    println!("\nAnalysis finished, {} issue(s) found.", issues_found);
    if issues_found > 0 {
        exit(2)
    }
}
