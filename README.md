<img width="3440" height="1147" alt="Zašto" src="https://github.com/user-attachments/assets/06187736-b303-4e4e-a18b-f0b0bb9e187c" />

[![Download for Windows](https://img.shields.io/badge/Download-Latest%20Release-blue?style=for-the-badge&logo=github)](https://github.com/moki-fr/zasto/releases/latest/download/zasto.exe)
[![Download for Linux](https://img.shields.io/badge/Download-Latest%20Release-blue?style=for-the-badge&logo=github)](https://github.com/moki-fr/zasto/releases/latest/download/zasto)


v1.0 

---

Zašto is an open source AI powered tool to help you analyze your disks easily ! 

---


# Infos and prerequisites

The project is build with python 3.1x+ and probably later Rust, C++ or Go later to make browsing through disks faster
The python part is used to communicate with the Openrouter API and for simplicity to make TUI stuff.
Zašto is made to work on both Linux and Windows ! 
And Zašto means "why" btw

## Prerequisites

- Python 3.10 or above
- A Openrouter API Key (it's free and easy to get)

> [Click here to get your free API key](HOWDOIGETMYAPIKEY.md)

# How to use

The main script is zasto.py, so to run it you have to run

```
python zasto.py [command]
```

If no command is set, it will bring you to the help page.

| Command | Argument | Description |
|-----------|-----------|-----------|
| `--version`   | None    | Shows you the version    |
| `--key`   | OR API key    | Sets a OpenRouter API key, only for this command    |
| `--storekey` | OR API key | Sets a OpenRouter API key, and stores it in config files |
| `--model` | OR AI model | Sets a AI model in config files (default is google/gemma-4-26b-a4b-it) |
| `--ignorelist` | File path | Sets a list where the scanning script won't check, only for this command |
| `--path` | Directory path | Sets a path where you want the script to check, only for this command |
| `--filelist` | Number (int) | Sets the number of file that will be given to the AI |
| `--scan` | None | Starts scanning |


If you're too lazy to understand all this, you can just run this command:

```
python ./zasto.py --storekey <API_KEY_HERE> --model google/gemma-4-26b-a4b-it --scan
```
