# Shell/terminal fundamentals

https://learnxinyminutes.com/docs/bash/ (English)

https://learnxinyminutes.com/docs/fr-fr/bash-fr/ (French)

https://devhints.io/bash (cheatsheet)



# RC files

A `.{xyz}rc` file is a script file that is run when your terminal is launched, and provides a bunch of extra configurations that can be useful.

In this repo, you'll find some `.bashrc` files (which are all intentionally very similar to each other) - you won't even need to replace his username anywhere, since they were made to be generic, by using the `whoami` command.

To install them, you just need to put it at the right path:
- `.shrc-windows` -> `~/.bashrc`
- `.shrc-linux` -> `~/.bashrc`
- `.shrc-macos` -> `~/.zshrc`

You might also want to look into `oh-my-zsh` and `powerlevel9k` (or `powerlevel10k`) for a nicer shell experience.



# Commands

Here is a list of common shell commands and their uses.

NOTE: when there is a `${something}` written, that means that is text that you should replace, since it should be specific to your command

To learn more about each of these commands listed below, remember to always try both:
```sh
mycommand --help # should give you some short help/options summary for "mycommand"
man mycommand # should give you the complete instruction manual for "mycommand"
```

## Basic commands
- `echo ${text}`: Displays the same text which was given as input.
- `printf ${text}`: Displays the same text given as input, with a specific format.
- `pwd`: Display the path of the current working directory.
- `cd ${folder}`: Change the directory to another directory.
- `cd -`: Go back to the previous directory.
- `ls ${folder}`: List files in folder.
- `ls -l ${folder}` or `ll ${folder}`: List files by type and permission.
- `ls -a ${folder}` or `la ${folder}`: List all files, including hidden files.
- `cat ${file_1} ... ${file_n}`: Print the content of file(s) to terminal.
- `history`: List the history of all previous commands.
- `alias ${newcommand}='something'`: Used to define an alias for a specific command.
- `exit ${status}`: Used to exit the shell, with a certain exit status number.
- `. ${scriptfile}` or `source ${scriptfile}`: Read and executes commands from a specified file in the current shell.

## Searching & text manipulation
- `find ${folder} -name *.txt`: Search all text files in a certain directory.
- `find ${folder} -size 10k -print`: Find all files greater than 10k in a certain directory.
- `locate ${filepath}`: Find the location of a certain file quickly.
- `grep "foo" ${filepath}`: Searches the word "foo" in a text file.
- `egrep "(foo|bar)" ${filepath}`: Find the words "foo" and "bar" in a text file.
- `sed "s/foo/bar/g" ${filepath}`: Find the word "foo" and replace it with a "bar" in a text file.
- `awk '{ print; }' ${filepath}`: Run a custom text-manipulation script for a certain file (learn more about `awk`: https://learnxinyminutes.com/docs/awk/)

## System management
- `top`: List/view all running processes, memory usage, cpu usage in real-time.
- `ps -ef`: List/view all running services.
- `jobs`: List all active jobs.
- `lsof`: Lists files/sockets opened by running processes.
- `kill ${PID}` or `pkill ${NAME}`: Kill a currently-running program/process.
- `du -sh`: Shows total disk usage of the current directory.
- `df -h`: Shows free and used space on mounted filesystems.
- `free -m`: Show free and used memory and swap space.

## Users & permissions
- `whoami`: Display the name of the currently-logged-in user.
- `chmod ${filepermissions} ${filepath}`: Change the user, group, and other permissions for file.txt.
- `chown ${user}:${group} ${filepath}`: Change the owner of the file and directory.
- `su ${username}`: switch logged-in user to another user, for the current shell.
- `passwd ${username}`: Used to set or reset the user password.
- `sudo ${anycommand}`: 

## Networking
- `ping ${hostname}`: Used to check host reachability.
- `ifconfig`: To check all network interfaces, IPs, and Mac addresses.
- `ssh ${username}@${hostname}`: Used to login remotely to another machine.
- `netstat -ant`: To check all network connections.
- `netstat -ent`: To check established network connections.
- `nslookup ${hostname}`: Used for DNS query.
- `traceroute ${hostname}`: Used for network debugging.
- `scp -r dir ${username}@${hostname}:/opt/`: Copy all files and directories recursively from the local system to a remote system.
- `scp -r ${username}@${hostname}:/opt/ dir/`: Copy all files and directories recursively from the remote system to a local system.
- `rsync -avz localdir ${username}@${hostname}:/backup`: Synchronize files/directories between the local and remote systems.

## Advanced
- `read`: Read one line from STDIN and assigns it to a variable (for user input prompts).
- `declare`: Used to declare a variable.
- `export`: Used to set a variable available globally for all sub-processes.
- `fc`: Select a list of commands from the history list.
- `fg`: Run a job in foreground mode.
- `bg`: Run a job in background mode.
- `disown`: Remove a job from the job table.
- `bind`: Used to bind a keyboard sequence.
- `command`: Run a specific command without the normal shell lookup.
- `exec`: Replace the shell process with the specified command.
- `dirs`: Shows a list of all remembered directories.
- `enable`: Used to enable or disable built-in command.
- `hash`: Used to find and remember the full path of the specified command.
- `help`: Used to display the help file.
- `logout`: Used to exit from the current shell.
- `popd`: Removes entries from the directory stack.
- `pushd`: Add a directory to the directory stack.
- `times`: Displays the accumulated user and system shell time.
- `wait`: Make the shell wait for a job to finish.
- `last`: Display last user logins information.


# Shell operators

You have a "pipe" operator (written `|`) which can feed the output of a previous command to the input of a following command. For example `ps -aux | grep av` will find all running processes that contain `av` in their name. This can often be combined with the `xargs` command to supply another command (that takes multiple arguments as input) with the right argument properly.

You can also redirect file streams via chevrons (*aka* angle brackets, or carets), the symbols `<` and `>`.

   - `>` sends the content of the standard output and standard error streams of the terminal to the file after it. For example, `echo "Hello world"`

   - `<` sends the content of the file to the standard input of the terminal. For example, `grep "My Text" < /path/to/input.file` will read the content of `input.file`, filter the lines that contain "My Text" with `grep`, and write these lines to the standard output (the terminal).

   - You can also use both. For example, `grep "My Text" < /path/to/input.file > /path/to/output.file ` would take the content of the file `input.file` as input, filter all the lines that contain "My Text" with `grep`, and write these lines that contain "My Text", the output of `grep` to the file `output.file`.

There are also special redirection operators: `1>` redirects only STDOUT, `2>` redirects only STDERR, and more.

For more info on shell operators (and there's a bunch!), [see here](https://unix.stackexchange.com/questions/159513/what-are-the-shells-control-and-redirection-operators).



# Windows specific issues


## Installing and using cygwin

TODO


## Installing and using WSL

TODO


## Short guide on how to install python3.10 on cygwin

Since cygwin only distributes versions up to python3.9, we need to manually install python3.10, and make it accessible from inside cygwin.

1. Start by downloading python3.10 (the official distribution, not the cygwin distribution), from: https://www.python.org/downloads/windows/
By default, it will install python in the `C:\Python3.10` (aka `/cygdrive/c/Python3.10`) directory - I will assume this is your install folder for the rest of this guide (you may have a different folder if you install python from the microsoft store, for instance)

2. We need to allow cygwin to find `python.exe`, so we can use it as a command (ie: `python myscript.py`). To do this, we will add the python install directory to our cygwin PATH environment variable - this will allow cygwin to find it. You can do this temporarily, just for your current cygwin terminal, by doing:
```sh
export PATH="$PATH:/cygdrive/c/Python3.10"
```
But what we really want is for this to happen automatically, so you can just copy the line above, and add it to your `.bashrc`.

3. We need to create a symbolic link, which we will name `python3.10.exe`, so that we can use this command. You can create the symbolic link by doing:
```sh
ln -s "/cygdrive/c/Python3.10/python.exe" /usr/bin/python3.10
```
You can check that this step was successful by doing:
```sh
python3.10 --version
```
which should show the installed python3.10 version (3.10.something).

4. We need to make sure that the `pip` package manager works for our newly installed python3.10 - so first, do
```sh
python3.10 -m pip --version
```
If this command worked, it shows the version of pip you have installed. Otherwise, you will get an error like `no module named pip` or something - in this case, you can install pip by running the `get-pip.py` official script: you can get this script at https://bootstrap.pypa.io/get-pip.py, and run it by doing:
```sh
python3.10 ./get-pip.py
```

---

NOTE: the big difference between the cygwin-python and official-python is that cygwin smartly converts folderpaths between windows-style (e.g. `C:\Users\whatever` and unix-style `/home/whatever`) - since your newly installed python3.10 is the official-python, you need to be careful when writing folderpaths: you should never directly write in your python code any `/` forward-slashes or `\` back-slashes - you should instead always use the `os.path` functions. So, for example:
```py
folder = "./some/random/path" # This is bad! won't work on windows
folder = ".\\some\\random\\path" # This is bad! won't work on unix
folder = os.path.join(".","some","random","path") # This is correct: will work everywhere
```
