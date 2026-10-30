mod fancy;

use std::collections::hash_map::{IntoKeys, Keys};
use std::fs::FileType;

use fancy::PrintType::*;
use fancy::*;

mod cli;

mod ia;
use ia::*;

use std::process::*;

fn main() {
    let args = cli::parse_args();
    let args_number = cli::get_args_number();

    // Vars used by the scanning function
    let mut key: String = String::from("API_KEY_HERE");
    let mut model: String = String::from("google/gemma-4-26b-a4b-it");
    let mut ignore_list: String;
    let mut path: String;
    let mut json: String;

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

    let mut key_string: String = String::from("");

    if let Some(arg_key) = args.key {
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
                key = arg_key.clone();
                key_string = arg_key.clone();
            }
        } else {
            key = arg_key.clone();
            key_string = String::from(&format!("{}*****", &arg_key[0..15]));
            fancy_print(
                SuccessPrint,
                &format!("Using key {}*****", key_string.clone()),
            );
        }
        if !args.scan {
            fancy_print(
                WarningPrint,
                "The key flag is used without the scan flag, this key will not be stored in the config files, use --storekey instead ",
            );
        }
    }

    // model flag
    if let Some(arg_model) = args.model {
        fancy_print(SuccessPrint, &format!("Changed model to {}", arg_model));
        fancy_print(
            InfoPrint,
            "If you want to set it back to default, set it to google/gemma-4-26b-a4b-it",
        );
    }

    // ignorelist flag
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

    // path flag
    match args.path {
        Some(arg_path) => {
            path = arg_path.clone();
            fancy_print(SuccessPrint, &format!("Set focused path to {}", arg_path));
        }
        None => path = "/".to_string(),
    }

    // Json flag

    let mut json_string = "No";

    if let Some(arg_json) = args.json {
        json = arg_json.clone();
        fancy_print(SuccessPrint, &format!("Outputting JSON at {}", arg_json));

        json_string = "Yes";

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
        println!("Key: {}", key_string);
        println!("Model: {}", model);
        println!("Ignorelist: ");
        println!("Focused path: ");
        println!("JSON: {}", json_string);

        let scan_confirmation: String = // Y/n user input
            fancy_input("Do you want to scan with these options ? (Y/n): ");

        if scan_confirmation.to_lowercase() != "y" {
            fancy_print(InfoPrint, "Aborting");
            return;
        }
        fancy_print(InfoPrint, "Beginning scan now");

        fancy_print(InfoPrint, "Checking API key");

        match check_api_key(&key) {
            Ok(key_state) => {
                if !key_state {
                    fancy_print(ErrorPrint, "Key is invalid");
                    exit(1);
                }
            }
            Err(error) => {
                fancy_print(ErrorPrint, "Communication error");
                fancy_print(ErrorPrint, &error.to_string());
                fancy_print(InfoPrint, "Consider checking your internet settings");
                fancy_print(
                    InfoPrint,
                    "If this still persists, you can open an issue here: https://github.com/moki-fr/zasto",
                );
                exit(1);
            }
        }
    }
}
