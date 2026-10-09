import customtkinter as ctk
from PIL import Image


### COLORS ###
black = "#0f0f0f"
grey = "#1e1e1e"
white = "#c7c7c7"
whiteHover = "#e0e0e0"



### ASSET INIT ###

zastoLogoImage = Image.open("./assets/Zasto.png")
githubImage = Image.open("./assets/Github.png")
settingsImage = Image.open("./assets/Settings.png")


### APP ###

root = ctk.CTk()
root.geometry("900x600")
root.resizable(False, False)
root.configure(fg_color=black)

### WIDGETS ###

zastoLogo = ctk.CTkImage(dark_image=zastoLogoImage, size=(161, 51))
zastoLogoLabel = ctk.CTkLabel(root, text="", image=zastoLogo) # Label for the logo because image cannot be displayed without a label idk why

scanBtn = ctk.CTkButton(root, text="SCAN", fg_color=white, text_color=black, hover_color=whiteHover, font=("Inter", 31, 'bold'), width=290, height=88, corner_radius=30)

githubBtnImg = ctk.CTkImage(dark_image=githubImage, size=(38,38))
githubBtn = ctk.CTkButton(root, text="", image=githubBtnImg, fg_color=grey, hover_color=white, width=58, height=58, corner_radius=29, border_spacing=0, compound="center")


### DISPLAYING ###
zastoLogoLabel.grid(row=0, column=0, padx=25, pady=25)
scanBtn.place(relx=0.5, rely=0.5, anchor="center")
githubBtn.place(x=50, y=50)

root.mainloop()
