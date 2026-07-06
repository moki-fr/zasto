####################
## Scanner script ##
####################


# Imports
from pathlib import Path
import os


# Const
StartMessage = "Initializing the scan..."
WhileSCaning = "Currently scanning: "
FinishScanning = "The scan has finish"


# var
ScannerStart = Path("") #The Path where the scanner start
IgnoreList = [] #Paths where the script wont look 
HowMuchFile = 100 #Limites the number of file inside the TopFIle
TopFiles = [] #Contains the biggest files in size with there path to be send to the AI, limited to how much the var "HowMuchFile" is set to, default = 0
SkipExtention = [] #Extention with this extention will be skiped

#Change Var value to the corrects one
def ChangeVarValue():

    return None

#Main scanning function
LeastFile = "" #The file with the least size inside the TopFiles list, required for the MainScanningFunction
def MainScanningFunction():
    for file in ScannerStart.glob("*"):
        print(WhileSCaning, file._raw_path)
        if file.is_file:    #Check if it is a file
            for Ignore in IgnoreList:   #Check if it's not in a folder of the IgnoreList
                if not file.is_relative_to(Ignore): #Check if the file isnt in a IgnoreList path


            
