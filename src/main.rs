// Fancy custom prints
mod fancy;

use fancy::PrintType::*;
use fancy::*;

// Args parsing
mod cli;

// Path checking for config
mod config;

// Requesting Openrouter API
mod ia;
use ia::*;

// Exiting
use std::process::*;

mod scanner;

use std::path::Path;

fn main() {
    // Args
    let args = cli::parse_args();
    let args_number = cli::get_args_number();

    if !config::check_config() {
        fancy_print(InfoPrint, "~/.zasto/ directory not found, creating it...");
    }

    let zasto_config = config::get_toml_values();

    // Vars used by the scanning function
    let mut key: String;
    let model: String;
    let mut ignore_list: String;
    let path: String;
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
                config::change_toml_value(String::from("api_key"), arg_key);
                fancy_print(SuccessPrint, "Stored key to config files");
                fancy_print(
                    WarningPrint,
                    "This key is less than 16 characters long and could be invalid",
                );
            }
        } else {
            config::change_toml_value(String::from("api_key"), arg_key.clone());
            fancy_print(
                SuccessPrint,
                &format!("Stored key {}*****", &arg_key[0..15]),
            );
        }
    }

    // Making sure that we get the api key from the config before the --key flag so this flag can override the stored key
    key = zasto_config.api_key;

    // Key flag

    // This string is the key but cut off and hidden so it can be shown in the logs safely

    let mut key_string: String;

    if key.len() >= 15 {
        key_string = String::from(&format!("{}*****", &key[0..15]));
    } else {
        key_string = String::from(&key);
    }

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
        config::change_toml_value(String::from("model"), arg_model.clone());
        fancy_print(SuccessPrint, &format!("Changed model to {}", arg_model));
        fancy_print(
            InfoPrint,
            "If you want to set it back to default, set it to google/gemma-4-26b-a4b-it",
        );
    }

    model = zasto_config.model;

    ignore_list = String::from("");

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
    json = String::from("");

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
        println!("Ignorelist: {}", ignore_list);
        println!("Focused path: {}", path);
        println!("JSON: {}", json_string);

        if 1 == 2 {
            println!("{}{}{}", ignore_list, json, path);
        }

        let scan_confirmation: String = // Y/n user input
            fancy_input("Do you want to scan with these options ? (Y/n): ");

        if scan_confirmation.to_lowercase() != "y" && scan_confirmation != "" {
            fancy_print(InfoPrint, "Aborting");
            return;
        }
        fancy_print(InfoPrint, "Beginning scan now");

        fancy_print(InfoPrint, "Checking API key");

        match check_api_key(&key) {
            // Check API Key
            Ok(key_state) => {
                if !key_state {
                    // if key is wrong
                    fancy_print(ErrorPrint, "Key is invalid");
                    exit(1);
                }
            }
            Err(error) => {
                // error with internet or Openrouter
                fancy_print(ErrorPrint, "Communication error");
                fancy_print(
                    ErrorPrint,
                    &format!("Here is the Rust exact error:\n{}", error),
                );
                fancy_print(InfoPrint, "Consider checking your internet settings");
                fancy_print(
                    InfoPrint,
                    "If this still persists, you can open an issue here: https://github.com/moki-fr/zasto",
                );
                exit(1);
            }
        }

        fancy_print(InfoPrint, "Starting scanner");
        let scan_output = scanner::scanner(Path::new("/"));

        println!("{:?}", scan_output);
    }
}
