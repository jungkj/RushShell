# Rush Shell

The shell has the following requirements:

Read in each line the user enters and attempt to execute it as a new process. <br/>
Exit the shell if the user enters "exit" on a line by itself.<br/>
The "exit" command is case insensitive.<br/>
Run the new process in the background if the command ends in an "&".<br/>
If there is an &, it will be the last word in the command.<br/>
Handle the 3 basic I/O redirection operators:<br/>
* < means to redirect stdin for the command from the given file.<br/>
* > means to redirect stdout for the command to the given file, replacing the file if it already exists.<br/>
* >> means to redirect stdout for the command to the given file, appending to the file if it already exists.<br/>
* You can have at most one stdin redirection (<) and at most one stdout redirection (> or >>) in each command.  All redirection operators will appear after the command name and arguments, but before the & (if present).<br/>
