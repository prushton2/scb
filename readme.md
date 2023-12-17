# Simple C Builder

I would rather build my own C build tool than use someone elses, so here it is.

# Help

I will implement help. In the meantime:

# init
Initializes the build tool, creating the file scb. Add this to your gitignore yourself.

# file
Command for adding/removing files to the tool.

-s \[files]       Takes list of files seperated by space and adds them to the list to compile
-r \[files]       Takes list of files seperated by space and removes them from the list to compile
-ls               Lists all files to compile

# header
Displays the current directory for the header files

Arguments:
-s \[path]      Takes a path as the header directory


# out
Displays the current name of the outfile

Arguments:
-s (name)       Takes a file name for the outfile

# build
Builds the project

Arguments:
-r              Runs the built executable