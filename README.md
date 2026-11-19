<img width="3440" height="1147" alt="Zašto" src="https://github.com/user-attachments/assets/06187736-b303-4e4e-a18b-f0b0bb9e187c" />

# ATTENTION
Since this project has been rebuilt in Rust, some features might not work or are not implemented **yet**

---

Zasto is an open-source all-in-one disk cleaning tool, built in Rust, powered by AI to analyze your disks easily for you ! Works on Linux and Windows, and is 100% human made, no AI is used while coding this project !

---

**Useful MDs**

[Get your OpenRouter API key for free](MDs/HOWDOIGETMYAPIKEY.md)

[View code changelog](MDs/CHANGELOG.md)

---



# Prerequisites

- Rust (cargo, rustc...)
- A Openrouter API Key (it's free and easy to get)

> [Click here to get your free API key](HOWDOIGETMYAPIKEY.md)

# How to use

You can either:
- Execute the binaries on the release page
- Compile zasto from its source with `cargo build`

Once you've got your zasto binary, you can use it:

```
zasto [command]
```

If no command is set, it will bring you to the help page.

*hint: OR means Openrouter*

| Command | Argument | Description |
|-----------|-----------|-----------|
| `--version`   | None    | Shows you the version    |
| `--key`   | OR API key    | Sets a OpenRouter API key, only for this command    |
| `--storekey` | OR API key | Sets a OpenRouter API key, and stores it in config files |
| `--model` | OR AI model | Sets a AI model in config files (default is google/gemma-4-26b-a4b-it) |
| `--ignorelist` | File path | Sets a list where the scanning script won't check, only for this command |
| `--path` | Directory path | Sets a path where you want the script to check, only for this command |
| `--scanoutputlenght` | Number (int) | Sets the number of file that will be given to the AI |
| `--json` | Output file path | Outputs the worth-deleting files in a JSON file instead of showing them in the TUI selector |
| `--force` | None | Forces unsafe options (like overriding a JSON output file...) |
| `--yes` | None | Skips y/n prompt |
| `--scan` | None | Starts scanning |


If you're too lazy to understand all this, you can just run these commands: 

```
zasto --storekey <API_KEY_HERE> # Only once to store the key in config file
zasto --scan
```
or just
```
zasto --storekey <API_KEY_HERE> --scan
```
