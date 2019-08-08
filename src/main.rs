use std::env;
use std::io::Write;
use std::process::exit;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod compile;
mod thinbasic_script;
mod tokenizer;

fn print_color(text: &str, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)).set_intense(true))
        .unwrap();
    write!(&mut stdout, "{}", text).unwrap();
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::White)).set_intense(false))
        .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_color("input error: ", Color::Red);
        println!("Please supply thinBasic script file name as parameter");
        exit(1);
    }

    let main_file_name = (&args[1]).to_string();
    println!("In {}:\n", main_file_name);

    let mut code = match thinbasic_script::Code::new(main_file_name) {
        Ok(c) => c,
        Err(e) => {
            print_color("error: ", Color::Red);
            println!("{}", e);
            exit(1)
        }
    };

    let mut issues_found: i32 = 0;

    if compile::analysis_available(&mut code) {
        match compile::pairs_match(&mut code) {
            Ok(()) => (),
            Err(v) => {
                issues_found += 1;
                print!("Line {:>2}:{:>2} - ", v.line, v.pos);
                print_color(&v.summary, Color::Red);
                println!();
            }
        };
    } else {
        println!("No #compile section, skipping part of analysis...")
    }

    println!("\n----------------------------------------");
    print!("Analysis finished: ");
    if issues_found > 0 {
        print_color(&format!("{}", issues_found), Color::Red);
        print_color(" issue(s) found\n", Color::Red);
    } else {
        print_color("no issues found\n", Color::Green);
    }
    println!("----------------------------------------");

    if issues_found > 0 {
        exit(2)
    }
}
