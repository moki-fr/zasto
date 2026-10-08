#################
## SCANNER LIB ##
#################

# IMPORTS
from pathlib import Path
import os
from utils.fancy import *
import threading

preIgnoreList = ["/proc/, /sys/, /Windows/, .git/"] # List of things that would not be considered as worth scanning


# Vars
count = 0
done = False


# FUNCTIONS

# Main scanning function
def scan(ignoreList: str = None, focusedPath: str = None, listPathNumber: int = 100):
    global count
    global done

    # ignoreList (list) is for ignoring some paths (default is None)
    # focusedPath (str) is for searching in a specified path (default is None)
    # listPathNumber (int) is the number of paths that will be given to the ai

    infoPrint("info", "Scanning... This may take a while")
    threading.Thread(target=scannerPrint).start() # Starts the scanner print func in another thread
    
    unverifiedFiles = 0 # Number of file that couldn't be verified due to permission error or stuff like that
    
    try:
        if not focusedPath:
            focusedPath = "/" # Sets focused path to root if not provided

        items = [] # list for ALL FILES and their size

        for root, dirs, files in os.walk(focusedPath): # Scans all the provided path
            for file in files: # Loops all files because each file isn't represented as a path but another type, so we have to make them links
                filepath = root + "/" + file # Names the path from the root to the file

                # Ignores useless paths to scan
                if any(ignore in filepath for ignore in preIgnoreList):
                    continue

                # Verifies if path is included in ignoreList
                if ignoreList and any(ignore in filepath for ignore in ignoreList):
                    continue


                try: # Try statement if some shii happens like permission errors
                    size = os.path.getsize(filepath)
                    items.append((filepath, size))

                except:
                    unverifiedFiles += 1

                count += len(files)
                

        # Sorts all file and reverse them so it's from highest to lowest
        done = True
        items.sort(key=lambda x: x[1], reverse=True)
        print("") # Makes a new line
        infoPrint("info", f"{unverifiedFiles} file(s) couldn't be verified due to permission errors")
        return items[:listPathNumber] # only gets the firsts

    except Exception as e:
        print("Error occured while scanning")
        print(f"Py error: {e}")
        return


def scannerPrint():
    while not done:
        print(f"\r[{CYAN}SCANNER{RESET}] {count} files are being processed", end="")
    
    