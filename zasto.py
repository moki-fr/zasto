# MAIN Script to execute

#############
## IMPORTS ## 
#############

import argparse # Parse command args
import sys # To exit

############
## CONSTS ## 
############

VERISON = "v1.0 Alpha"

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

#########################
## Files intialization ## 
#########################




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

if len(sys.argv) == 1:
    print(LOGO)
    print(VERISON)
    parser.print_help()
    sys.exit(0)


print(f"Key: {args.storekey}")



