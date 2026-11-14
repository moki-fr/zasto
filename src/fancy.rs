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
    fancy_print_full(print_type, text, false);
}

pub fn fancy_print_full(print_type: PrintType, text: &str, replaceable: bool) {
    let replacement_suffix: String; // \n or not

    if replaceable {
        replacement_suffix = String::from("");
    } else {
        replacement_suffix = String::from("\n");
    }

    // \r is for replacing the line
    // \x1b[2K is for erasing the line in some case the previous line was longer that the newer and wont erase the last chars without this backslash pattern

    match print_type {
        PrintType::SuccessPrint => {
            print!(
                "\r\x1b[2K{}{}{} {}{}",
                "[".bold().white(),
                "+".bold().green(),
                "]".bold().white(),
                text.white(),
                replacement_suffix
            );
            io::stdout().flush().unwrap();
        }
        PrintType::InfoPrint => {
            print!(
                "\r\x1b[2K{}{}{} {}{}",
                "[".bold().white(),
                "i".bold().cyan(),
                "]".bold().white(),
                text.white(),
                replacement_suffix
            );
            io::stdout().flush().unwrap();
        }
        PrintType::WarningPrint => {
            print!(
                "\r\x1b[2K{}{}{} {}{}",
                "[".bold().white(),
                "!".bold().yellow(),
                "]".bold().white(),
                text.white(),
                replacement_suffix
            );
            io::stdout().flush().unwrap();
        }

        PrintType::ErrorPrint => {
            print!(
                "\r\x1b[2K{}{}{} {}{}",
                "[".bold().white(),
                "X".bold().red(),
                "]".bold().white(),
                text.white(),
                replacement_suffix
            );
            io::stdout().flush().unwrap();
        }
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
