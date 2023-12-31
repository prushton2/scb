# Simple C Builder

I would rather build my own C build tool than use someone elses, so here it is.

# scb

Running this command will initialize the scb file if not already initialized, same effect as `scb init`.

If the project is initialized, it will build and execute the project, same effect as `scb build -r`


# Help

`scb -h` or `scb --help`

# init
Initializes the build tool, creating the file scb. Add this to your gitignore yourself.

# remove
Removes the scb file. Originally made for unit testing, but it may be useful elsewhere.

# file
Command for adding/removing files to the tool.

Arguments:<br/>
`-a \[files]`       Takes list of files seperated by space and tracks them<br/>
`-r \[files]`       Takes list of files seperated by space and untracks them<br/>
`-ls`               Lists all files to compile<br/>

# header
Displays the current directory for the header files (-I in gcc)

Arguments:<br/>
`-s \[path]`      Takes a path as the header directory

# out
Displays the current name of the outfile

Arguments:<br/>
`-s (name)`       Takes a file name for the outfile

# build
Builds the project

Arguments:<br/>
`-r`              Runs the built executable