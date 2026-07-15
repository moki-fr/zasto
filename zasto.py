# MAIN Script to execute

#############
## IMPORTS ## 
#############

import argparse # Parse command args
import sys # Detect OS and exit
import os # Set home directory 
from pathlib import Path # Create config files
import questionary # Create questionnaries, for selecting which file to delete

from utils import scanner, ai # Our libs for scanning and analyzing

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
"""

DEBUG = True
def debugPrint(text):
    if DEBUG: print(text)

VERISON = "v1.0"
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
        f.write("google/gemma-4-26b-a4b-it")

KEY = None

##################
## ARGS PARSING ## 
##################

parser = argparse.ArgumentParser(description="Zašto? - An intelligent disk analyzer")

parser.add_argument("--version", action="store_true", help=f"Shows you that the version is {VERISON} ;)")
parser.add_argument("--key", metavar="API_KEY", help="Sets an OpenRouter API key")
parser.add_argument("--storekey", metavar="API_KEY", help="Sets AND stores an OpenRouter API key (at ~/.Zasto/key)")
parser.add_argument("--model", help="Sets a model to use and stores it (e.g. google/gemma-4-26b-a4b-it) and stores it at ~/.Zasto/model")
parser.add_argument("--ignorelist", metavar="FILE", help="Path to the file that contains every paths that should not be verified")
parser.add_argument("--path", help="Recursively scans only one directory (default is root)")
parser.add_argument("--filelist", default=100, help="Defines how many file are gonna be in the file list that's gonna be transfered to the ai")
parser.add_argument("--scan", action="store_true")


args = parser.parse_args()


# If no args are provided then show the help menu and da beautiful logo
if len(sys.argv) == 1:
    print(LOGO)
    print(VERISON)
    parser.print_help()
    sys.exit(0)


# Da version
if args.version: # BOOL, false by default
    print(f"Version: {VERISON}")


# If storekey is provided
if args.storekey != None: # parser.get_default("model") is necessary because it's never False or None

    if args.storekey == "reset":
        key = ""
        print("Reset key in config")

    else:
        key = args.storekey
        print(f"Stored key {args.storekey[0:15]}*****")

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
    print("If you want set it back to the default, set it to google/gemma-4-26b-a4b-it (free)")
# Gets model from config file
model = open(f"{ZASTO_DIR}/model", "r").read()




# Gets ignorelist
if args.ignorelist != None:
    ignoreList = []
    try: # Try statement to avoid errors
        
        with open(args.ignorelist, "r") as f: # Open ignorelist file 
            lines = f.readlines()
        
        for line in lines: # Strip line by line
            ignoreList.append(line) # Adds every line of file into the list
        
        ignoreListStr = args.ignorelist # String to show in overview page when --scan is provided, this will show the path of ignorelist

        print("Set ignorelist")
        
        if len(sys.argv) == 3: # Warns user in case the command is being used alone
            print("Warning: It looks like you're using this command with no other option, ignore list is not stored in config files.")

    except:
        print("Error occured while trying to import ignorelist, file might not exists")
        sys.exit(1)

else:
    ignorelist = [] # Create empty ignorelist
    ignoreListStr = "Not set" # String to show in overview page when --scan is provided


focusedPath = "/"
# Sets path to scan
if args.path != None:
    if os.path.exists(args.path): # Checks if path exist
        focusedPath = args.path

        print("Focused path set")

        if len(sys.argv) == 3: # Warns user in case the command is being used alone
            print("Warning: It looks like you're using this command with no other option, focused path is not stored in config files.")

    else:
        print("Path does not exist")


filelist = 100
# Gets filelist number
if args.filelist != 100: 

    filelist = args.filelist

    print("Filelist number set")


###########
## SCAN  ##
###########


# Start scanning
if args.scan: # BOOL
    os.system(CLEAR_COMMAND[OS]) # Clears the shell whether the machine is on Win or Lnx
    print(LOGO)

    print(" ")

    print("Now beginning scan")
    print(" ")
    print("Quick overview:")
    print(f"- API Key: {key[:15]}*****")
    print(f"- Model: {model}")
    print(f"- Ignorelist: {ignoreListStr}")
    print(f"- Path: {focusedPath}")
    print(f"- File list number: {filelist}")
    
    print(" ")


    # Asks before scanning
    if input("Are you sure to process scan with all these options ? (y/N) ").lower() != "y": 
        print("Aborted.")
        sys.exit(0)

    # Scans
    fileScan = scanner.scan(focusedPath=focusedPath, listPathNumber=filelist)
    
    if fileScan == None:
        print("File scan returns 'None' for some reason wth") 
        sys.exit(1)

    print("Scan successful")

    print("Contacting AI...")
    aiReply = ai.ai(api_key=key, model=model, userPrompt=fileScan)



    
    topFiles = aiReply.split("|") # Sets a list where each item is a group of path & comment: ["/home/user/file@Big file"]
    

    choicesToSelect = [] # Choices to select i guess...

    for file in topFiles: # Loops all the top files 
        path, comment, size = file.split("@") # Separates the path from the comment
        
        choicesToSelect.append(questionary.Choice(title=path, description=f"{comment} - {size}", value=path)) # Adds every file as a choice, with each path, description and size


    os.system(CLEAR_COMMAND[OS]) # Clear

    print(SCAN_COMPLETE)
    print("")
    print("Select files that you want to delete now. These files are sorted from heaviest to lightest")
    print("When hovering a file, you can see its description at the very bottom")

    filesToDelete = questionary.checkbox("Files: ", choices=choicesToSelect).ask()

    if len(filesToDelete) == 0: # If no files are selected
        print("Done, nothing to delete")
        sys.exit(0)

    if len(filesToDelete) == 1: # If 1 file is selected (to say 'this file' and not 'these 1 files' cuz that sounds weird)
        askConfirmation = input("Are you sure to delete this file ? (y/N) ")

    if len(filesToDelete): # If >1 files are selected
        askConfirmation = input(f"Are you sure to delete these {len(filesToDelete)} files ? (y/N) ")

    if askConfirmation.lower() != "y": # Checks confirmation
        print("No files were deleted")
        sys.exit(0)

    print("Deleting files...")
    for i in filesToDelete:
        try:
            os.remove(i.replace("\"", "")) # Removes quotes
            print(f"Removed: {i}")
        except Exception as e:
            print(f"Error deleting {i}, skipping")

    

    print(THANKS)
    print("Thank you for using Zasto, have a great day")