use std::env;
use std::io::Write;
use std::process::exit;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod rules;
mod thinbasic_script;
mod tokenizer;

fn print_color(text: &str, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)).set_intense(true))
        .unwrap();
    write!(&mut stdout, "{}", text).unwrap();
    stdout
        .set_color(
            ColorSpec::new()
                .set_fg(Some(Color::White))
                .set_intense(false),
        )
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

    let mut code = match thinbasic_script::Code::new(&main_file_name) {
        Ok(c) => c,
        Err(e) => {
            print_color("input error: ", Color::Red);
            println!("{}", e);
            exit(1)
        }
    };

    println!("In {}:\n", main_file_name);

    let mut issues_found: i32 = 0;

    if rules::compiled::analysis_available(&mut code) {
        let issues = rules::compiled::section_definition(&mut code);

        for v in issues {
            print!("\n{}", "-".repeat(80));
            println!();

            let lines = &mut code.get_file_content().unwrap().lines();

            issues_found += 1;

            print!("Line {:>5} - ", v.line);

            print_color(lines.nth((v.line - 1) as usize).unwrap(), Color::White);
            println!();

            print!("{}", " ".repeat((v.pos + 12) as usize));
            println!("^");
            print!("{}", " ".repeat((13) as usize));
            print_color(&v.summary, Color::Red);
        }
        println!();
    } else {
        print_color("[i] ", Color::Green);
        println!("No violations against #compile")
    }

    print!("\n{}", "-".repeat(80));
    print!("\nAnalysis finished: ");
    if issues_found > 0 {
        print_color(&format!("{}", issues_found), Color::Red);
        print_color(" issue(s) found\n", Color::Red);
    } else {
        print_color("no issues found\n", Color::Green);
    }
    print!("{}", "-".repeat(80));
    println!();
    if issues_found > 0 {
        exit(2)
    }
}
