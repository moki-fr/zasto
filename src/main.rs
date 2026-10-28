mod fancy;
use fancy::PrintType::{ErrorPrint, InfoPrint, SuccessPrint, WarningPrint};
use fancy::*;

mod cli;

fn main() {
    let args = cli::parse_args();

    print_zasto_logo();
    fancy_print(SuccessPrint, "YOO waddup la gang");
    fancy_print(InfoPrint, "Thats an info");
    fancy_print(WarningPrint, "Thats a warning");
    fancy_print(ErrorPrint, "Oh shii");

    if args.yes {
        println!("You used the --yes flag");
    }
}
