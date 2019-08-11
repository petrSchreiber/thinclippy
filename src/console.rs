use std::io;
use std::io::prelude::*;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn wait_enter() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    print_color("\nPress ENTER to quit...", Color::White);
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn print_color(text: &str, color: Color) {
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
