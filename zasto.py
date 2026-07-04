# MAIN Script to execute

#############
## IMPORTS ## 
#############

import argparse # Parse command args
import sys # Detect OS and exit
import os # Set home directory 
from pathlib import Path # Create config files

############
## CONSTS ## 
############

DEBUG = True

def debugPrint(text): # Debugs if needed
    if DEBUG: print(text)


# Logo to load only if Zasto is executed without any args
LOGO = r""" 
$$$$$$$$\                      $$\               
\____$$  |                     $$ |              
    $$  / $$$$$$\   $$$$$$$\ $$$$$$\    $$$$$$\  
   $$  /  \____$$\ $$  _____|\_$$  _|  $$  __$$\ 
  $$  /   $$$$$$$ |\$$$$$$\    $$ |    $$ /  $$ |
 $$  /   $$  __$$ | \____$$\   $$ |$$\ $$ |  $$ |
$$$$$$$$\\$$$$$$$ |$$$$$$$  |  \$$$$  |\$$$$$$  |
\________|\_______|\_______/    \____/  \______/ """

# Help page to load on --help
HELP_PAGE = r"""Help page

> zasto.py <command> <option>

Commands:
 • help 
 • key <OpenRouter API key>
 • model <AI Model> (tip: you can reset the model with "reset" option, which sets it to "openai/gpt-4o")
 • scan <dontask *optional*> (tip: scan command asks to whether to start or not the scan, you can specify if you don't want confirmation)
"""

VERISON = "v1.0 Alpha"
HOME_DIR =os.path.expanduser("~").replace("\\", "/") # Simplify \ to /
ZASTO_DIR = Path(HOME_DIR) / ".zasto"

CLEAR_COMMAND = {"win": "cls", "lnx": "clear"}

##################
## OS DETECTION ## 
##################

if sys.platform == "win32":
    OS = "win"
elif sys.platform == "linux":
    OS = "lnx"
else:
    print("OS not supported, sorry :/")

print(os.path.expanduser("~"))



##########################
## FILES INITIALIZATION ## 
##########################

# Creates ~/.zasto/ dir
ZASTO_DIR.mkdir(parents=True, exist_ok=True)

# Sets API key and model files 
KEY_FILE = ZASTO_DIR / "key"
MODEL_FILE = ZASTO_DIR / "model"

# Create files if needed
if not KEY_FILE.exists():
    KEY_FILE.touch()

if not MODEL_FILE.exists():
    MODEL_FILE.touch()
    with open(f"{ZASTO_DIR}/key", "w") as f:
        f.write("openai/gpt-4o")

KEY = None

##################
## ARGS PARSING ## 
##################

parser = argparse.ArgumentParser(description="Zašto? - An intelligent disk analyzer")

parser.add_argument("--version", action="store_true", help=f"Shows you that the version is {VERISON} ;)")
parser.add_argument("--key", metavar="API_KEY", help="Sets an OpenRouter API key")
parser.add_argument("--storekey", metavar="API_KEY", help="Sets AND stores an OpenRouter API key (at ~/.Zasto/key)")
parser.add_argument("--model", default="openai/gpt-4o", help="Sets a model to use and stores it (e.g. openai/gpt-4o) and stores it at ~/.Zasto/model")
parser.add_argument("--ignorelist", metavar="FILE", help="Path to the file that contains every paths that should not be verified")
parser.add_argument("--path", default="/", help="Recursively scans only one directory (default is root)")
parser.add_argument("--scan", action="store_true")

args = parser.parse_args()

# If no args are provided then show the help menu and da beautiful logo
if len(sys.argv) == 1:
    print(LOGO)
    print(VERISON)
    parser.print_help()
    sys.exit(0)

# Da version
if args.version:
    print(f"Version: {VERISON}")

# If storekey is provided
if args.storekey != None:

    if args.storekey == "reset":
        key = ""
        print("Reset key in config")

    else:
        key = args.storekey
        print(f"Stored key {args.storekey[0:10]}*****")

    with open(f"{ZASTO_DIR}/key", "w") as f:
        f.write(f"{key}")

# Reads key in config BEFORE getting the key from the command so it doesn't overwrite the key in the command
key = open(f"{ZASTO_DIR}/key", "r").read()

# Gets key from command
if args.key != None:
   key = args.key

# Gets model
if args.model != None:
    with open(f"{ZASTO_DIR}/model", "w") as f:
        f.write(f"{args.model}")
    print(f"Set model {args.model} to config")
    print("If you want set it back to the default, set it to openai/gpt-4o (free)")
# Gets model from config file
model = open(f"{ZASTO_DIR}/model", "r").read()

# Gets ignorelist
if args.ignorelist != None:
    print("Feature not supported yet")

# Sets path to scan
if args.path != None:
    print("Feature not supported yet")

# Start scanning
if args.scan != None:
    os.system(CLEAR_COMMAND[OS]) # Clears the shell whether the machine is on Win or Lnx
    print(LOGO)

    print(" ")

    print("Now beginning scan")
    print(" ")
    print("Quick overview:")
    print(f"- API Key: {key}")
    print(f"- Model: {model}")
    print(f"- Ignorelist: unavaible")
    print(f"- Path: unavaible")
    
    print(" ")

    # Asks before scanning
    if input("Are you sure to process scan with all these options ? (y/N)").lower() != "y": 
        print("Aborted.")
        sys.exit(0)

    
