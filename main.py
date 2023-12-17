import sys
import os
import json

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
    with open("./config.json", "r") as f:
        return json.loads(f.read())

def writeConfig(newConfigObject):
    with open("./config.json", "w") as f:
        f.write(json.dumps(newConfigObject))


# Command Functions
def init():
    writeConfig(initConfig)
    print(f"Initialized build tool. Add config.json to gitignore")

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

def build():
    cfg = loadConfig()
    os.system(f"{cfg['compiler']} -I {cfg['headerDir']} {' '.join(cfg['files'])} -o {cfg['outfile']}")

def outfile(action, arg):
    cfg = loadConfig()    
    if(action == "-s"):
        cfg["outfile"] = arg
        print(f"Set outfile to {arg}")

    elif(action == None):
        print(f"Outfile: {cfg['outfile']}")

    writeConfig(cfg)

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
        build()

if(__name__ == "__main__"):
    main(sys.argv)