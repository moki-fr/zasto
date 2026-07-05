<img width="3440" height="1147" alt="Zašto" src="https://github.com/user-attachments/assets/06187736-b303-4e4e-a18b-f0b0bb9e187c" />

v1.0 Alpha

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

# How to use

The main script is zasto.py, so to run it you have to run

```bash
python zasto.py [command]
```

If no command is set, it will bring you to the help page.

- --version: Shows you the version
- --key **API_KEY**: Sets a temporary API key to the command
- --key **API_KEY**: Sets a API key and stores it so you don't have to specify it again
- --model **MODEL**: Sets a model (default is openai/gpt-4o, that one is free)
- --ignorelist **FILE**: makes Zašto ignore part(s) of your disk (must link to a file which contains all pathes you don't want Zasto to look)
- --path **PATH**: Exclusively scans a specific directory
- --scan: start scanning

