use std::process::exit;
use termcolor::Color;

use structopt::StructOpt;

mod console;
mod rules;
mod thinbasic_script;
mod tokenizer;

#[derive(Debug, StructOpt)]
#[structopt(name = "thinClippy", about = "Tool for thinBasic code analysis.")]
struct Opt {
    script_file: String,

    #[structopt(short = "w", long = "wait")]
    wait: bool,
}

fn main() {
    let command_line_params = Opt::from_args();

    let main_file_name = command_line_params.script_file;

    println!("In {}:\n", main_file_name);

    let mut code = match thinbasic_script::Code::new(&main_file_name) {
        Ok(c) => c,
        Err(e) => {
            console::print_color("input error: ", Color::Red);
            println!("{}", e);

            if command_line_params.wait {
                console::wait_enter();
            }
            exit(1)
        }
    };

    let mut issues_found: i32 = 0;

    let mut issues: Vec<thinbasic_script::IssueSummary> = vec![];

    let mut compiled_issues = rules::compiled::section_definition(&mut code);
    issues.append(&mut compiled_issues);

    let mut alias_issues = rules::core::alias::check(&mut code);
    issues.append(&mut alias_issues);

    issues.sort_by(|a, b| a.line.cmp(&b.line));

    for v in issues {
        print!("\n{}", "-".repeat(80));
        println!();

        let lines = &mut code.get_file_content().unwrap().lines();

        issues_found += 1;

        print!("Line {:>5} - ", v.line);

        console::print_color(lines.nth((v.line - 1) as usize).unwrap(), Color::White);
        println!();

        print!("{}", " ".repeat((v.pos + 12) as usize));
        println!("^");
        print!("{}", " ".repeat((13) as usize));
        console::print_color(&v.summary, Color::Red);
    }
    println!();

    print!("\n{}", "-".repeat(80));
    print!("\nAnalysis finished: ");
    if issues_found > 0 {
        console::print_color(&format!("{}", issues_found), Color::Red);
        console::print_color(" issue(s) found\n", Color::Red);
    } else {
        console::print_color("no issues found\n", Color::Green);
    }
    print!("{}", "-".repeat(80));
    println!();
    if issues_found > 0 {
        if command_line_params.wait {
            console::wait_enter();
        }
        exit(2)
    }
}
