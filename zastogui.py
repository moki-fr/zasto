import zasto
import customtkinter as ctk
from PIL import Image
import time

### COLORS ###
black = "#0f0f0f"
grey = "#1e1e1e"
white = "#c7c7c7"
whiteHover = "#e0e0e0"



### ASSET INIT ###

zastoLogoImage = Image.open("./assets/Zasto.png")
githubImage = Image.open("./assets/GithubBtn.png")
settingsImage = Image.open("./assets/SettingsBtn.png")


### APP ###

root = ctk.CTk()
root.geometry("900x600")
root.resizable(False, False)
root.configure(
    fg_color=black, # the BACKGROUND color and not the foreground took a while to realize this
)
root.title(f"Zasto {zasto.VERSION}")
root.iconbitmap("./assets/ZastoSquare.ico")

### WIDGETS ###

zastoLogo = ctk.CTkImage(dark_image=zastoLogoImage, size=(161, 51))
zastoLogoLabel = ctk.CTkLabel(root, text="", image=zastoLogo) # Label for the logo because image cannot be displayed without a label idk why

scanBtn = ctk.CTkButton(root,
    text="SCAN",
    fg_color=whiteHover,
    text_color=black,
    hover_color=white,
    font=("Inter", 31, 'bold'),
    width=290, height=88,
    corner_radius=30,
    cursor="hand2" # Hover cursor
)

githubBtnImg = ctk.CTkImage(dark_image=githubImage, size=(58,58))
githubBtn = ctk.CTkButton(root,
    text="",
    image=githubBtnImg,
    width=58, height=58,
    fg_color="transparent",
    hover_color=black,
    cursor="hand2"
)

settingsBtnImg = ctk.CTkImage(dark_image=settingsImage, size=(58,58))
settingsBtn = ctk.CTkButton(
    root,
    text="",
    image=settingsBtnImg,
    width=58, height=58,
    fg_color="transparent",
    hover_color=black,
    cursor="hand2"
)



def boingLogoCuzItsFun(*args):
    
    print("Clicked")

        

zastoLogoLabel.bind("<Button-1>", boingLogoCuzItsFun)

### DISPLAYING ###
zastoLogoLabel.place(anchor="nw", x=25, y=25)
scanBtn.place(relx=0.5, rely=0.5, anchor="center")
settingsBtn.place(relx=1.0, rely=0.0, anchor="ne", x=-10, y=10)
githubBtn.place(relx=1.0, rely=0.0, anchor="ne", x=-78, y=10)


root.mainloop()
