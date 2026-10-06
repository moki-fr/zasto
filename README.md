<img width="3440" height="1147" alt="Zašto" src="https://github.com/user-attachments/assets/06187736-b303-4e4e-a18b-f0b0bb9e187c" />

--- 

[![Download Windows](https://img.shields.io/badge/Download-Windows-blue?style=for-the-badge&logo=windows11&logoColor=white)](https://github.com/moki-fr/zasto/releases/latest/download/zasto.exe)
[![Download Linux](https://img.shields.io/badge/Download-Linux-brightgreen?style=for-the-badge&logo=linux&logoColor=white)](https://github.com/moki-fr/zasto/releases/latest/download/zasto)

v1.2

Made in France 💙🤍❤️

---

Zašto is an open source AI powered tool to help you analyze your disks easily ! 
Zašto is made to work on both Linux and Windows ! 
Zašto is 100% human made, no AI is used in the code.

---

**Useful MDs**

[Get your OpenRouter API key for free](MDs/HOWDOIGETMYAPIKEY.md)

[View code changelog](MDs/CHANGELOG.md)

---

# Infos and prerequisites

The project is built with python 3.1x+ and C++ 

|      Language      |    Why         |
|--------------------|----------------|
| Python             | Easier to use for our cli-tui base and for Openrouter's API |
| C++ (and not rust) | As Zasto uses a scanner to gather information on your disk, C++ would make this process faster especially on HDDs |  
 


## Prerequisites

- Python 3.10 or above
- *Optional:* a C++ compiler
- A Openrouter API Key (it's free and easy to get)

> [Click here to get your free API key](HOWDOIGETMYAPIKEY.md)

# How to use

You can either:
- Execute the binaries on the release page (not updated for now)
- Run the main script zasto.py as well as making sure you have all the depedencies (`pip install -r requirements.txt`)

*If you want to run the C++ scanner, you'll also have to compile it first and run it with python*

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
| `--json` | Output file path | Outputs the worth-deleting files in a JSON file instead of showing them in the TUI selector |
| `--force` | None | Forces some options (works with: `--json`) |
| `--yes` | None | Skips y/n prompt |
| `--scan` | None | Starts scanning |


If you're too lazy to understand all this, you can just run these commands: 

```
python ./zasto.py --storekey <API_KEY_HERE> # Only once to store the key in config file
python ./zasto.py --scan
```
or just
```
python ./zasto.py --storekey <API_KEY_HERE> --scan
```

## How to build

The releases are built with nuitka since Zasto 1.2, since it provides faster code execution and lighter file output size.
You can still package the project with pyinstaller, but the output will be slower and heavier.
 
### 1. Create venv (recommended)

```
python -m venv venv
```

Then activate the venv, command changes depending on your OS and shell, here's an example for Linux on Fish shell

```
source /venv/bin/activate.fish
```

### 2. Install requirements & nuitka

```
pip install -r requirements.txt
```

```
pip install nuitka
```

### 3. Build with Nuitka

```
python -m nuitka zasto.py --onefile
```

This can take about 3-10 min depending on your hardware
Nuitka's log should show you where your compiled file was created, it's often in Zasto's folder


## To do

|  Task   |   State   |
|---------|-----------|
| Make the C++ scanner compatible with Python | In work (ExpensiveCoal) |
| Create a GUI version | Not started |
| Speed up execution speed organizing depedencies | Not started |
