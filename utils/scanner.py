#################
## SCANNER LIB ##
#################

# IMPORTS
from pathlib import Path
import os

# FUNCTIONS

# Main scanning function
def scan(ignoreList: str = None, focusedPath: str = None, listPathNumber: int = 100):

    # ignoreList (list) is for ignoring some paths (default is None)
    # focusedPath (str) is for searching in a specified path (default is None)
    # listPathNumber (int) is the number of paths that will be given to the ai 

    if not startPath:
        startPath = "/" # Sets focused path to root if not provided

    
    items = [] # list for ALL FILES and their size

    for root, dirs, files in os.walk(startPath): # Scans all the provided path 
        for file in files: # Loops all files because each file isn't represented as a path but another type, so we have to make them links
            filepath = os.path.join(root, file) # Creates the path via the file, and uses root to sort of "connect them" root <--path--> file
            
            # Verifies if path is included in ignoreList
            if ignoreList and any(ignore in filepath for ignore in ignoreList):
                continue
            
            
            try: # Try statement if some shii happens like permission errors
                size = os.path.getsize(filepath)
                items.append((filepath, size))

            except:
                pass
    
    # Sorts all file and reverse them so it's from highest to lowest
    items.sort(key=lambda x: x[1], reverse=True)
    
    
    return items[:listPathNumber] # only gets the firsts