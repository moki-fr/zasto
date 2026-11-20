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
use std::{fs, process::*};

mod scanner;

use inquire::*; // select menus
use std::path::Path;

fn main() {
    // Args

    let args = cli::parse_args();
    let args_number = cli::get_args_number();

    if !config::check_config() {
        fancy_print(InfoPrint, "~/.zasto/ directory not found, creating it...");
    }

    let zasto_config: config::Config;

    match config::get_toml_values() {
        Ok(c) => zasto_config = c,
        Err(_) => {
            fancy_print(
                ErrorPrint,
                "Encontered and error while reading throught your config file, try deleting ~/.zasto/ directory to reset it",
            );
            return;
        }
    }

    // Vars used by the scanning function
    let mut key: String;
    let model: String;
    let mut ignore_list: String;
    let path: String;
    let mut json: String;
    let yes: bool;

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

    // Yes flag
    if args.yes {
        yes = true;
    } else {
        yes = false;
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
        None => {
            #[cfg(target_os = "windows")]
            {
                path = String::from(r"C:\");
            }

            #[cfg(not(target_os = "windows"))]
            {
                path = String::from("/");
            }
        }
    }

    // Json flag

    let mut json_string = "No";
    json = String::from("");

    if let Some(arg_json) = args.json {
        json = arg_json.clone();

        json_string = "Yes";

        if Path::new(&json).exists() {
            // if json path already exists
            if args.force {
                fancy_print(
                    WarningPrint,
                    "Output JSON file already exists, overriding it",
                );
            } else {
                fancy_print(
                    ErrorPrint,
                    "The selected output JSON path already exists, use --force to bypass",
                );
            }
        }
        // try creating the complete path for the JSON output
        match fs::create_dir_all(Path::new(&json).parent().unwrap()) {
            Err(e) => {
                fancy_print(
                    ErrorPrint,
                    "Couln't create path through the JSON output file",
                );
                fancy_print(InfoPrint, &format!("Rust error: {}", e));
                exit(1);
            }
            Ok(_) => {
                fancy_print(SuccessPrint, &format!("Outputting JSON at {}", arg_json));
            }
        }

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

        if !yes {
            let scan_confirmation: String = // Y/n user input
                fancy_input("Do you want to scan with these options ? (Y/n): ");

            if scan_confirmation.to_lowercase() != "y" && scan_confirmation != "" {
                fancy_print(InfoPrint, "Aborting");
                return;
            }
        }

        fancy_print(InfoPrint, "Beginning scan now");

        fancy_print_full(ProgressPrint, "Checking API key", true);

        match check_api_key(&key) {
            // Check API Key
            Ok(key_state) => {
                if !key_state {
                    // if key is wrong
                    fancy_print(ErrorPrint, "Key is invalid");
                    exit(1);
                } else {
                    fancy_print(SuccessPrint, "Key is valid");
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

        fancy_print_full(ProgressPrint, "Starting scanner", true);

        if !Path::new(&path).exists() {
            // if focused path is invalid
            fancy_print(
                ErrorPrint,
                "Your focused path doesn't exist, please check again",
            );
            exit(1);
        }

        //let scan_output = scanner::scanner(Path::new(&path));
        let scan_output = scanner::scan(Path::new(&path));

        fancy_print(SuccessPrint, "Scan complete");

        fancy_print_full(ProgressPrint, "Sorting results", true);

        let sorted_scan_output = scanner::sort_results(&scan_output, args.scanoutputlenght);

        fancy_print(SuccessPrint, "Sort complete");
        fancy_print_full(ProgressPrint, "Contacting AI...", true);

        let ai_response = ai(&key, &model, &format!("{:?}", sorted_scan_output));

        let path_list: String;

        match ai_response {
            Err(e) => {
                fancy_print(ErrorPrint, "AI contacting Error");
                println!("Rust error: {}", e);
                fancy_print(
                    InfoPrint,
                    "This error can happen sometimes, wait a little then restart zasto",
                );
                exit(1);
            }
            Ok(s) => {
                if s.to_lowercase() == "ai_thinking_error" {
                    // catch if the ai fails to think
                    fancy_print(ErrorPrint, "AI thinking error");
                    fancy_print(
                        InfoPrint,
                        "This usually happens when the ai fails to understand something in the prompt, therefore this is a pretty uncommon error so you should retry scanning, if it fails multiple time, first check your selected path, if you think it is correct, you should open an issue here: https://github.com/moki-fr/zasto",
                    );
                    exit(1);
                }
                fancy_print(SuccessPrint, "AI response received");
                path_list = s;
            }
        }

        let mut displayed_path: Vec<String> = Vec::new(); // paths to be displayed in the selection menu

        for line in path_list.lines() {
            let parts: Vec<&str> = line.split('|').collect();

            displayed_path.push(format!("{} | {} ({})", parts[0], parts[1], parts[2]));
        }

        print!("\x1B[2J\x1B[1;1H"); // clears term

        let selected_paths: Vec<String>;

        match MultiSelect::new("Select paths", displayed_path).prompt() {
            Err(_) => {
                fancy_print(ErrorPrint, "Error with selection menu");
                exit(1);
            }
            Ok(v) => {
                selected_paths = v;
            }
        }

        if selected_paths.is_empty() {
            fancy_print(SuccessPrint, "Nothing to delete");
        }

        let user_confirmation = fancy_input(&format!(
            "Are you sure to delete these {} files ? (y/N): ",
            selected_paths.iter().len()
        ));

        if user_confirmation.to_lowercase() != "y" {
            fancy_print(InfoPrint, "Aborting");
            exit(0);
        }

        let mut failed_deletions: Vec<String> = Vec::new();

        for line in selected_paths {
            let file_to_delete: String = line
                .split(" | ") // get the first part aka path
                .next()
                .unwrap()
                .trim()
                .trim_matches('"') // remove quotes
                .to_string();

            fancy_print_full(ProgressPrint, &format!("Deleting {}", file_to_delete), true);

            match fs::remove_file(&file_to_delete) {
                Err(_) => {
                    fancy_print(ErrorPrint, &format!("Failed to delete {}", file_to_delete));
                    failed_deletions.push(file_to_delete);
                }
                Ok(_) => {
                    fancy_print(SuccessPrint, &format!("Deleted {}", file_to_delete));
                }
            }
        }
        if !failed_deletions.is_empty() {
            fancy_print(
                InfoPrint,
                "\nSome files couldn't be deleted, you can use this command to delete them all as root (on Linux for now)\n",
            );
            println!("```");
            print!("sudo rm -f ");

            for file in &failed_deletions {
                print!("\"{}\" ", file);
            }
            print!("\n```");
        }
    }
}
