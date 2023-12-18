import sys
import os
import json
from tabulate import tabulate

initConfig = {
    "compiler": "gcc",
    "files": [],
    "headerDir": "../",
    "outfile": "main"
}

# Internal Functions
def concat(arr, fixedlength):
    while(len(arr) < fixedlength):
        arr.append(None)
    return arr

def loadConfig():
    with open("./scb", "r") as f:
        return json.loads(f.read())

def writeConfig(newConfigObject):
    with open("./scb", "w") as f:
        f.write(json.dumps(newConfigObject))


# Command Functions
def init():
    writeConfig(initConfig)
    print(f"Initialized build tool. Add scb to gitignore")

def compiler(action, arg):
    cfg = loadConfig()

    if(action == "-s"):
        cfg["compiler"] = arg
        print(f"Set compiler directory to {arg}")

    elif(action == None):
        print(f"compiler: {cfg['compiler']}")


    writeConfig(cfg)

def files(action, files):
    cfg = loadConfig()
    if(action == "-ls"):
        print("C files:")
        print("\n".join(cfg["files"]))

    elif(action == "-a"):
        i = 0
        while(files[i] != None):
            cfg["files"].append(files[i])
            i += 1
        print(f"Added {i} file{'s' if i>1 else ''}")

    elif(action == "-r"):
        i = 0
        while(files[i] != None):
            cfg["files"].remove(files[i])
            i += 1
        print(f"Removed {i} file{'s' if i>1 else ''}")

    writeConfig(cfg)

def headerDir(action, arg):
    cfg = loadConfig()    
    if(action == "-s"):
        cfg["headerDir"] = arg
        print(f"Set header directory to {arg}")

    elif(action == None):
        print(f"Header Directory: {cfg['headerDir']}")

    writeConfig(cfg)

def build(arg):
    cfg = loadConfig()
    os.system(f"{cfg['compiler']} -I {cfg['headerDir']} {' '.join(cfg['files'])} -o {cfg['outfile']}")
    if(arg == "-r"):
        os.system(f"./{cfg['outfile']}")


def outfile(action, arg):
    cfg = loadConfig()    
    if(action == "-s"):
        cfg["outfile"] = arg
        print(f"Set outfile to {arg}")

    elif(action == None):
        print(f"Outfile: {cfg['outfile']}")

    writeConfig(cfg)

def help(command):
    if(command == None or command == "init"):
        print("\ninit\n   initialize the build tool and creates the file scb. Add this to .gitignore")
    if(command == None or command == "file"):
        print("\nfile\n   Manages files the build tool should keep track of")
        print(tabulate([
            ["Arguments", "", ""],
            ["-a", "[files]", "Takes a list of files and tracks them"],
            ["-r", "[files]", "Takes a list of files and untracks them"],
            ["-ls", "", "Lists tracked files"],
        ], headers="firstrow"))
    if(command == None or command == "header"):
        print("\nheader\n   Displays the current header directory")
        print(tabulate([
            ["Arguments", "", ""],
            ["-a", "[path]", "Takes a path and sets it as the header directory"]
        ], headers="firstrow"))
    if(command == None or command == "out"):
        print("\nout\n   Displays the name of the outfile")
        print(tabulate([
            ["Arguments", "", ""],
            ["-a", "[path]", "Takes a path and sets the outfile"]
        ], headers="firstrow"))
    
    if(command == None or command == "compiler"):
        print("\ncompiler\n   Displays the current compiler")
        print(tabulate([
            ["Arguments", "", ""],
            ["-s", "command", "Sets the compiler to the provided command"]
        ], headers="firstrow"))
    if(command == None or command == "build"):
        print("\nbuild\n   Builds the project")
        print("   "+tabulate([
            ["Arguments", "", ""],
            ["-r", "", "Runs the project after building"]
        ], headers="firstrow"))


# Main
def main(args):
    args = concat(args, 64)
    if(args[1] == "init"):
        init()
    elif(args[1] == "file"):
        files(args[2], args[3:])
    elif(args[1] == "header"):
        headerDir(args[2], args[3])
    elif(args[1] == "compiler"):
        compiler(args[2], args[3])
    elif(args[1] == "out"):
        outfile(args[2], args[3])
    elif(args[1] == "build"):
        build(args[2])
    elif(args[1] == "help"):
        help(args[2])
if(__name__ == "__main__"):
    main(sys.argv)