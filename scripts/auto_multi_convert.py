import os 
from pathlib import Path
parent_dir = os.path.dirname(os.path.dirname(__file__))

os.system("clear")

print("Hi! enter settings, if you will not enter any settings they'll be standart")
print("NOTE: if your image is bright make charset more \"sharp\" for exemple \" M90#M%@\" base settings are for dark images")
print("Avaliable settings: resolution: enter cols=\nfont: enter font=\ncharset: enter charset=")
print("Enter settings:")
print(parent_dir)
inp = input("~> ")

settings = [opt for opt in inp.split() if opt.split("=")[0] in ("cols", "font", "charset")]

in_name = "convert"
out_name = "out"

folder = Path(os.path.join(parent_dir, in_name))
files = [p.name for p in folder.iterdir() if p.is_file()]
for file in files:
    command = f"./target/release/ASCII_photo_converter {os.path.join(folder, file)} --out_name=./out/{file.split(".")[0]} "
    if len(settings) != 0:
        for opt in settings:
            command += f"--{opt} " 
    print(command)
    os.system(f"{command}")
