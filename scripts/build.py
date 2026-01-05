import os
import shutil
import pathlib as pb

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
print("Hi, welcome to ASCII photo converter, add photo to \"convert\" pholder and after a while they appear in \"out\" folder read README for more instructions")
