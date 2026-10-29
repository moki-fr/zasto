mod fancy;

use std::fs::FileType;

use fancy::PrintType::*;
use fancy::*;

mod cli;

fn main() {
    let args = cli::parse_args();
    let args_number = cli::get_args_number();

    // Vars used by the scanning function
    let key: String;
    let model: String;
    let ignore_list: String;
    let path: String;
    let json: String;

    // If no args
    if args_number == 0 {
        print_zasto_logo();
        fancy_print(InfoPrint, "Entering help menu");
        return;
    }

    // Force flag
    if args.force {
        fancy_print(WarningPrint, "Force option is used");
    }

    // Storekey flag
    if let Some(arg_key) = args.storekey {
        if arg_key.len() <= 15 {
            if !args.force {
                fancy_print(ErrorPrint, "Cannot store invalid key");
                fancy_print(InfoPrint, "Use --force to bypass");
            } else {
                fancy_print(SuccessPrint, "Stored key to config files");
                fancy_print(
                    WarningPrint,
                    "This key is less than 16 characters long and could be invalid",
                );
            }
        } else {
            fancy_print(
                SuccessPrint,
                &format!("Stored key {}*****", &arg_key[0..15]),
            );
        }
    }

    // Key flag
    if let Some(arg_key) = args.key {
        let key_string: String;
        if arg_key.len() <= 15 {
            if !args.force {
                fancy_print(ErrorPrint, "Cannot use invalid key");
                fancy_print(InfoPrint, "Use --force to bypass");
            } else {
                fancy_print(SuccessPrint, "Using key");
                fancy_print(
                    WarningPrint,
                    "This key is less than 16 characters long and could be invalid",
                );
                key = arg_key
            }
        } else {
            key = arg_key.clone();
            fancy_print(SuccessPrint, &format!("Using key {}*****", &arg_key[0..15]));
        }
        if !args.scan {
            fancy_print(
                WarningPrint,
                "The key flag is used without the scan flag, this key will not be stored in the config files, use --storekey instead ",
            );
        }
    }

    if let Some(arg_model) = args.model {
        fancy_print(SuccessPrint, &format!("Changed model to {}", arg_model));
        fancy_print(
            InfoPrint,
            "If you want to set it back to default, set it to google/gemma-4-26b-a4b-it",
        );
    }

    if let Some(arg_ignorelist) = args.ignorelist {
        ignore_list = arg_ignorelist.clone();
        fancy_print(SuccessPrint, &format!("Set ignorelist {}", arg_ignorelist));

        if !args.scan {
            fancy_print(
                WarningPrint,
                "The ignorelist flag is used without the scan flag, the ignorelist will not be stored in the config files",
            );
        }
    }

    match args.path {
        Some(arg_path) => {
            path = arg_path.clone();
            fancy_print(SuccessPrint, &format!("Set focused path to {}", arg_path));
        }
        None => path = "/".to_string(),
    }

    if let Some(arg_json) = args.json {
        json = arg_json.clone();
        fancy_print(SuccessPrint, &format!("Set ignorelist {}", arg_json));

        if !args.scan {
            fancy_print(
                WarningPrint,
                "The JSON flag is used without the scan flag, JSON outputting will not be stored in the config files",
            );
        }
    }

    // Scan

    if args.scan {
        print_zasto_logo();
        println!("Key: ");
        println!("Model: ");
        println!("Ignorelist: ");
        println!("Focused path: ");
        println!("JSON: ");
    }
}
