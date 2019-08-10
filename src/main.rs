use std::process::exit;

use structopt::StructOpt;
use termcolor::Color;

mod console;
mod rules;
mod thinbasic_script;
mod tokenizer;

#[derive(Debug, StructOpt)]
#[structopt(name = "thinClippy", about = "Tool for thinBasic code analysis.")]
struct CommandLineParams {
    script_file: String,

    #[structopt(short = "w", long = "wait")]
    wait: bool,
}

fn main() {
    let command_line_params = CommandLineParams::from_args();

    println!("{}", "-".repeat(80));
    println!("{}", command_line_params.script_file);
    println!("{}", "-".repeat(80));

    let mut code = match thinbasic_script::Code::new(&command_line_params.script_file) {
        Ok(c) => c,
        Err(e) => {
            console::print_color("input error: ", Color::Red);
            println!("{}", e);

            end_program(1, command_line_params)
        }
    };

    let issues = get_issues(&mut code);

    print_issues(&issues, &mut code);

    end_program((!issues.is_empty()) as i32, command_line_params);
}

fn get_issues(mut code: &mut thinbasic_script::Code) -> Vec<thinbasic_script::IssueSummary> {
    let mut issues: Vec<thinbasic_script::IssueSummary> = vec![];

    let mut compiled_issues = rules::compiled::section_definition(&mut code);
    issues.append(&mut compiled_issues);

    let mut alias_issues = rules::core::alias::check(&mut code);
    issues.append(&mut alias_issues);

    issues.sort_by(|a, b| a.line.cmp(&b.line));

    issues
}

fn print_issues(issues: &[thinbasic_script::IssueSummary], code: &mut thinbasic_script::Code) {
    for issue in issues {
        let mut lines = code.get_file_content().unwrap().lines();

        print!("Line {:>5} - ", issue.line);

        console::print_color(lines.nth((issue.line - 1) as usize).unwrap(), Color::White);
        println!();

        print!("{}", " ".repeat((issue.pos + 12) as usize));
        println!("^");
        print!("{}", " ".repeat((13) as usize));
        console::print_color(&issue.summary, Color::Red);

        println!("\n{}", "-".repeat(80));
    }
    println!();

    print!("{}", "-".repeat(80));
    print!("\nAnalysis finished: ");

    if issues.is_empty() {
        console::print_color(&format!("{}", issues.len()), Color::Red);
        console::print_color(" issue(s) found\n", Color::Red);
    } else {
        console::print_color("no issues found\n", Color::Green);
    }

    print!("{}", "-".repeat(80));
    println!();
}

fn end_program(exit_code: i32, command_line_params: CommandLineParams) -> ! {
    if command_line_params.wait {
        console::wait_enter();
    }

    exit(exit_code)
}
