use std::env;
use std::process::exit;

use ansi_term;
use colored::*;

mod compile;
mod thinbasic_script;
mod tokenizer;

fn main() {
    if cfg!(windows) && ansi_term::enable_ansi_support().is_err() {
        colored::control::set_override(false);
    }

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "{} Please supply thinBasic script file name as parameter",
            "input error:".red().bold()
        );
        exit(1);
    }

    let main_file_name = (&args[1]).to_string();
    println!("In {}:\n", main_file_name);

    let mut code = match thinbasic_script::Code::new(main_file_name) {
        Ok(c) => c,
        Err(e) => {
            println!("{}: {}", "error:".red().bold(), e);
            exit(1)
        }
    };

    let mut issues_found: i32 = 0;

    if compile::analysis_available(&mut code) {
        match compile::pairs_match(&mut code) {
            Ok(()) => (),
            Err(v) => {
                issues_found += 1;
                println!("Line {:>2}:{:>2} {}", v.line, v.pos, v.summary.red().bold())
            }
        };
    } else {
        println!("No #compile section, skipping part of analysis...")
    }

    println!("\n----------------------------------------");
    print!("{}", "Analysis finished, ".white().bold());
    if issues_found > 0 {
        println!(
            "{} {}",
            issues_found.to_string().red().bold(),
            "issue(s) found".red().bold()
        );
    } else {
        println!("{}", "no issues found".green().bold());
    }
    println!("----------------------------------------");

    if issues_found > 0 {
        exit(2)
    }
}
