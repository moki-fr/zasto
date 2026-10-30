use owo_colors::OwoColorize;
use std::io::{self, Write};

const ZASTO_LOGO: &str = r#"
$$$$$$$$\                      $$\
\____$$  |                     $$ |
    $$  / $$$$$$\   $$$$$$$\ $$$$$$\    $$$$$$\
   $$  /  \____$$\ $$  _____|\_$$  _|  $$  __$$\
  $$  /   $$$$$$$ |\$$$$$$\    $$ |    $$ /  $$ |
 $$  /   $$  __$$ | \____$$\   $$ |$$\ $$ |  $$ |
$$$$$$$$\\$$$$$$$ |$$$$$$$  |  \$$$$  |\$$$$$$  |
\________|\_______|\_______/    \____/  \______/
"#;

pub enum PrintType {
    SuccessPrint,
    InfoPrint,
    WarningPrint,
    ErrorPrint,
}

pub fn fancy_print(print_type: PrintType, text: &str) {
    match print_type {
        PrintType::SuccessPrint => println!(
            "{}{}{} {}",
            "[".bold().white(),
            "+".bold().green(),
            "]".bold().white(),
            text.white()
        ),
        PrintType::InfoPrint => println!(
            "{}{}{} {}",
            "[".bold().white(),
            "i".bold().cyan(),
            "]".bold().white(),
            text.white()
        ),
        PrintType::WarningPrint => println!(
            "{}{}{} {}",
            "[".bold().white(),
            "!".bold().yellow(),
            "]".bold().white(),
            text.white()
        ),
        PrintType::ErrorPrint => println!(
            "{}{}{} {}",
            "[".bold().white(),
            "X".bold().red(),
            "]".bold().white(),
            text.white()
        ),
    }
}

pub fn fancy_input(text: &str) -> String {
    print!(
        "{}{}{} {}",
        "[".bold().white(),
        ">".bold().purple(),
        "]".bold().white(),
        text.white()
    );
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

pub fn print_zasto_logo() {
    println!("{}", ZASTO_LOGO);
}
