use owo_colors::OwoColorize;

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

pub fn print_zasto_logo() {
    println!("{}", ZASTO_LOGO);
}
