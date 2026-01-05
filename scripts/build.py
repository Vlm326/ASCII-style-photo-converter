import os
import shutil
import pathlib as pb
import subprocess


if shutil.which("cargo") is not None:
    print("Rust is installed")
else:
    print("You need to install Rust, do you want to do it right now?")
    ans = input("y/n: ")
    if ans == "y":
        os.system("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
    else:
        print("Install Rust by yourself")
        exit()


os.system("cargo build --release")
print("Project build succesefully")


folder = pb.Path(os.path.dirname(__file__))
root_project_dir = os.path.dirname(os.path.dirname(__file__))

username = subprocess.check_output(["whoami"], text=True).strip()
font_path = f"/home/{username}/.local/share/"
font_folder = pb.Path(font_path)

font_files = [file for file in folder.iterdir() if file.is_file()]

new_font_path = ""
for file in font_files:
    if "mono" in file or "Mono" in file:
        new_font_path = os.path.join(font_folder, file)
        break

if new_font_path == "":
    print("Please install monospace font, they're better then regular once :)")
    exit()


main_rs = os.path.join(root_project_dir, "src", "main.rs")

old = 'font_path: "/home/vlm326/.local/share/fonts/JetBrainsMonoNLNerdFont-Regular.ttf"'
new = f'font_path: "{new_font_path}"'

with open(main_rs, "r", encoding="utf-8") as f:
    text = f.read()

if old not in text:
    print("How?")
else:
    text = text.replace(old, new, 1)
    with open(main_rs, "w", encoding="utf-8") as f:
        f.write(text)
    print("Alright, font is changed font_path")


files = [file for file in folder.iterdir() if file.is_file()]

try:
    os.mkdir("./out")
except:
    print("out folder exists")

try:
    os.mkdir("./convert")
except:
    print("convert folder exists")

os.system("clear")
print('Hi, welcome to ASCII photo converter, add photo to "convert" pholder and after a while they appear in "out" folder read README for more instructions')
