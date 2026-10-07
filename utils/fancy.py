BLACK  = "\033[30m"
RED    = "\033[31m"
GREEN  = "\033[32m"
YELLOW = "\033[33m"
BLUE   = "\033[34m"
PURPLE = "\033[35m"
CYAN   = "\033[36m"
WHITE  = "\033[37m"
RESET  = "\033[0m"

def infoPrint(type, text):
    type = type.strip().lower()
    if type == "success":
        print(f"[{GREEN}+{RESET}] {text}")
    elif type == "info":
        print(f"[{CYAN}i{RESET}] {text}")
    elif type == "warning":
        print(f"[{YELLOW}!{RESET}] {text}")
    elif type == "error":
        print(f"[{RED}!{RESET}] {text}")