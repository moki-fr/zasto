# MAIN Script to execute

#############
## IMPORTS ##
#############

import argparse # Parse command args
import sys # Detect OS and exit
import os # Set home directory
from pathlib import Path # Create config file
import toml # Config file
from utils.fancy import * # Fancy prints

############
## CONSTS ##
############

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



# https://texteditor.com/multiline-text-art/
# Great website <3


SCAN_COMPLETE = """
 ▄▀▀ ▄▀▀ ▄▀▄ █▄ █   ▄▀▀ ▄▀▄ █▄ ▄█ █▀▄ █   ██▀ ▀█▀ ██▀
 ▄██ ▀▄▄ █▀█ █ ▀█   ▀▄▄ ▀▄▀ █ ▀ █ █▀  █▄▄ █▄▄  █  █▄▄"""


THANKS = """
 ▀█▀ █▄█ ▄▀▄ █▄ █ █▄▀   ▀▄▀ ▄▀▄ █ █
  █  █ █ █▀█ █ ▀█ █ █    █  ▀▄▀ ▀▄█

  Thank you for using Zasto ! Have a great day
"""

DEFAULT_CONFIG = {
    "ai":{
        "api_key": "YOUR_OPENROUTER_API_KEY",
        "model": "google/gemma-4-26b-a4b-it"
    }
}

# Python should use camelCase

# OS DETECTION
if sys.platform == "win32":
    OS = "win"
elif sys.platform == "linux":
    OS = "lnx"
else:
    infoPrint("error", "Your OS is not supported, please consider buying a non-Mac PC and install Linux on it")


VERSION = "v1.2"
HOME_DIR = os.path.expanduser("~").replace("\\", "/") # Simplify \ to /
ZASTO_DIR = Path(HOME_DIR) / ".zasto"
CONFIG_FILE_PATH = ZASTO_DIR / "config.toml"

CLEAR_COMMAND = {"win": "cls", "lnx": "clear"}


# Creates ~/.zasto/ dir
ZASTO_DIR.mkdir(parents=True, exist_ok=True)

##########################
## FILES INITIALIZATION ##
##########################



if not CONFIG_FILE_PATH.exists(): # Creates config file if it doesn't exist
    CONFIG_FILE_PATH.touch()
    with open(f"{CONFIG_FILE_PATH}", "w") as f:
        toml.dump(DEFAULT_CONFIG, f)

TOML_CONFIG = toml.load(f"{ZASTO_DIR}/config.toml")

STORED_API_KEY = TOML_CONFIG["ai"]["api_key"]
STORED_MODEL = TOML_CONFIG["ai"]["model"]



##################
## ARGS PARSING ##
##################

parser = argparse.ArgumentParser(description="Zašto? - An intelligent disk analyzer")

parser.add_argument("--version", action="store_true", help=f"Shows you that the version is {VERSION} ;)")
parser.add_argument("--key", metavar="API_KEY", help="Sets an OpenRouter API key")
parser.add_argument("--storekey", metavar="API_KEY", help="Sets AND stores an OpenRouter API key (in config)")
parser.add_argument("--model", help="Sets a model to use and stores it (e.g. google/gemma-4-26b-a4b-it) and stores it in config)")
parser.add_argument("--ignorelist", metavar="FILE", help="Path to the file that contains every paths that should not be scanned")
parser.add_argument("--path", help="Recursively scans only one directory (default is root)")
parser.add_argument("--filelist", default=100, help="Defines how many file are gonna be in the file list that's gonna be transfered to the ai")
parser.add_argument("--json", metavar="FILE", help="Outputs all the worth-deleting files in a json file instead of showing them in a tui selector")
parser.add_argument("--force", action="store_true", help="Forces action that are not safe (like overwritting a  JSON file)") 
parser.add_argument("--yes", action="store_true", help="Ignores the y/n prompt")
parser.add_argument("--scan", action="store_true")

args = parser.parse_args()


# If no args are provided then show the help menu and da beautiful logo
if len(sys.argv) == 1:
    print(LOGO)
    infoPrint("info", f"Version: {VERSION}")
    parser.print_help()
    sys.exit(0)


# Da version
if args.version: # BOOL, false by default
    infoPrint("info", f"Version: {VERSION}")


# If storekey is provided
if args.storekey != None: # parser.get_default("model") is necessary because it's never False or None

    if args.storekey == "reset":
        key = ""
        infoPrint("success", f"Reset stored key")

    else:
        key = args.storekey
        infoPrint("success", f"Stored key {args.storekey[0:15]}*****")

    with open(f"{CONFIG_FILE_PATH}", "w") as f:
        TOML_CONFIG["ai"]["api_key"] = f"{key}"
        toml.dump(TOML_CONFIG, f)



key = STORED_API_KEY # Reads key in config BEFORE getting the key from the command so it doesn't overwrite the key in the command
model = STORED_MODEL

# Gets key from command
if args.key != None:
    key = args.key

key = key.strip() # Removes shii that freaks out AI requests

# Gets model
if args.model != None:
    with open(f"{CONFIG_FILE_PATH}", "w") as f:
        TOML_CONFIG["ai"]["model"] = f"{args.model}"
        toml.dump(TOML_CONFIG, f)
    infoPrint("success", f"Set model {args.model} to config")

    infoPrint("info", "If you want set it back to the default, set it to google/gemma-4-26b-a4b-it (it's free)")

    model = args.model


# Gets ignorelist
if args.ignorelist != None:
    ignoreList = []
    try: # Try statement to avoid errors

        with open(args.ignorelist, "r") as f: # Open ignorelist file
            lines = f.readlines()

        for line in lines: # Strip line by line
            ignoreList.append(line) # Adds every line of file into the list

        ignoreListStr = args.ignorelist # String to show in overview page when --scan is provided, this will show the path of ignorelist

        infoPrint("success", "Ignorelist set")

        if not args.scan: # Warns user in case the command is being used alone
            infoPrint("warning", "Warning: It looks like you're using this command with no other option, ignore list is not stored in config file.")

    except:
        infoPrint("error", "Error occured while trying to import ignorelist, file might not exists")
        sys.exit(1)

else:
    ignorelist = [] # Create empty ignorelist
    ignoreListStr = "Not set" # String to show in overview page when --scan is provided


focusedPath = "/"
# Sets path to scan
if args.path != None:
    if os.path.exists(args.path): # Checks if path exist
        focusedPath = args.path

        infoPrint("success", "Focused path set")

        if not args.scan: # Warns user in case the command is being used alone
            infoPrint("warning", "Warning: It looks like you're using this command with no other option, focused path is not stored in config file.")

    else:
        infoPrint("error", "Path does not exist")


filelist = 100
# Gets filelist number
if args.filelist != 100:

    filelist = args.filelist

    infoPrint("success", "Filelist number set")

    if not args.scan: # Warns user in case the command is being used alone
        infoPrint("warning", "Warning: It looks like you're using this command with no other option, filelist is not stored in config file.")


# Gets ignorelist
jsonStr = "No"
if args.json != None:
    
    if os.path.exists(args.json): # If file exist
        
        if args.force == False: # If --force is not used
            infoPrint("error", "Error, this file already exists, use '--force' to ignore")
            sys.exit(1)

        infoPrint("warning", "Warning: You will overwrite an existing JSON file")

    jsonStr = f"Yes ({args.json})"

    infoPrint("success", "JSON output file set")

    if not args.scan: # Warns user in case the command is being used alone
        infoPrint("warning", "Warning: It looks like you're using this command with no other option, JSON outputting is not stored in config file.")

if args.yes:
    infoPrint("success", "Yes option applied")
    
    if not args.scan: # Warns user in case the command is being used alone
        infoPrint("warning", "Warning: It looks like you're using this command with no other option, \"Yes\" option is not stored in config file.")


############
##  SCAN  ##
############


# Verifies if --scan is used
if args.scan == False: sys.exit(0) # BOOL

#os.system(CLEAR_COMMAND[OS]) # Clears the shell whether the machine is on Win or Lnx
print(LOGO)
print(" ")
print("Beginning scan now")
print(" ")
print("Quick overview:")
print(f"- API Key: {key[:15]}*****")
print(f"- Model: {model}")
print(f"- Ignorelist: {ignoreListStr}")
print(f"- Path: {focusedPath}")
print(f"- File list number: {filelist}")
print(f"- JSON: {jsonStr}")
print("\n")

# Asks before scanning
if not args.yes: # If --yes is not specified
    yn = input("Are you sure to process scan with all these options ? (y/N) ").lower()
    if yn != "y":
        infoPrint("warning", "Aborted")
        sys.exit(0)

# Scans


from utils import scanner, ai # import the scanner & ai utils

fileScan = scanner.scan(focusedPath=focusedPath, listPathNumber=filelist)

if fileScan == None:
    infoPrint("error", "File scan returns 'None' for some reason wth")
    sys.exit(1)

infoPrint("success", "Scan successful")
infoPrint("info", "Contacting AI...")

aiReply = ai.ai(api_key=key, model=model, userPrompt=fileScan)


match args.json: # Do specific action depending on if the JSON option is used
    case None: # If no JSON is specified
        import questionary # Create questionnaries, for selecting which file to delete
        
        choicesToSelect = []

        for topFiles in aiReply.strip().splitlines(): # takes
            if not topFiles.strip():
                continue
            path, comment, size = topFiles.split("|") 
            choicesToSelect.append(questionary.Choice(title=path, description=f"{comment} - {size}", value=path)) # Adds every file as a choice, with each path, description and size


        os.system(CLEAR_COMMAND[OS]) # Clear

        print(SCAN_COMPLETE)
        print("\n")
        infoPrint("info", "Select files that you want to delete now. These files are sorted from heaviest to lightest")
        infoPrint("info", "When hovering a file, you can see its description at the very bottom")
        print("\n")

        filesToDelete = questionary.checkbox("Files: ", choices=choicesToSelect).ask()

        if len(filesToDelete) == 0: # If no files are selected
            infoPrint("success", "Done, nothing to delete")
            sys.exit(0)

        if len(filesToDelete) == 1: # If 1 file is selected (to say 'this file' and not 'these 1 files' cuz that sounds weird)
            askConfirmation = input("Are you sure to delete this file ? (y/N) ")

        else: # If >1 files are selected
            askConfirmation = input(f"Are you sure to delete these {len(filesToDelete)} files ? (y/N) ")

        if askConfirmation.lower() != "y": # Checks confirmation
            infoPrint("warning", "No files were deleted")
            sys.exit(0)

        infoPrint("info", "Deleting files...")
        for i in filesToDelete:
            try:
                os.remove(i.replace("\"", "")) # Removes quotes
                infoPrint("success", f"Removed: {i}")
            except Exception as e:
                infoPrint("error", f"Error deleting {i}, skipping")

        print(THANKS)



    case _: # If --json is used
        
        """
        JSON output should look like this:
        {
            "~/Downloads/HeavyFile.txt": {
                "size": 12303204,
                "comment": "Heavy file in download folder"
            },
            ...
        }
        """

        jsonOutput = {} # Create empty JSON object

        for topFiles in aiReply.strip().splitlines():
            if not topFiles.strip():
                continue
            path, comment, size = topFiles.split("|") 

            #TODO: Fix bug that make path quadruble backslash for some reason
            jsonOutput[path] = {"size": size, "comment": comment}


        try:
            Path(args.json).touch() # Creates the file
            with open(args.json, "w") as f: # Write JSON to the file
                f.write(str(jsonOutput))

            infoPrint("success", "JSON file written !")

        except:
            infoPrint("error", "Error while creating JSON file")

        print(THANKS)