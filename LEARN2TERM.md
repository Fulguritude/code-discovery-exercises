# Shell/terminal fundamentals

- https://learnxinyminutes.com/docs/bash/ (English)
- https://learnxinyminutes.com/docs/fr-fr/bash-fr/ (French)
- https://devhints.io/bash (cheatsheet)

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

### Basic commands
- `echo ${text}`: Displays the same text which was given as input.
- `printf ${text}`: Displays the same text given as input, with a specific format.
- `history`: List the history of all previous commands.
- `alias ${newcommand}='something'`: Used to define a new alias for a specific command.
- `exit ${status}`: Used to exit the shell, with a certain exit status number.
- `. ${scriptfile}` or `source ${scriptfile}`: Read and executes commands from a specified file in the current shell.

### Files and folders
- `pwd`: Display the path of the current working directory.
- `cd ${folder}`: Change the directory to another directory.
- `cd -`: Go back to the previous directory.
- `ls ${folder}`: List files in folder.
- `ls -l ${folder}` or `ll ${folder}`: List files by type and permission.
- `ls -a ${folder}` or `la ${folder}`: List all files, including hidden files.
- `cp ${source_file} ${target_file}`: Copies a file to a new location.
- `mv ${source_file} ${target_file}`: Move or rename a file to a new location (essentially a shorthand for `cp`+`rm`).
- `touch ${file}`: Create an empty file (or change date/time metadata of an existing file).
- `rm ${file}`: Delete a file.
- `mkdir ${folder}`: Create a new empty folder.
- `rmdir ${folder}`: Delete a folder (only if empty).
- `rm -r ${folder}`: Delete a folder, recursively deleting any files/folders inside of it.
- `ln ${source_file} ${target_file}`: Creates a hard-link clone of a certain file
- `ln -s ${source_file} ${link_name}`: Creates a symbolic-link of a certain file

### Searching & text manipulation
- `cat ${file}`: Display the contents of a text file.
- `diff ${file1} ${file2}`: Display the differences between two files.
- `find ${folder} -name *.txt`: Search all text files in a certain directory.
- `find ${folder} -size 10k -print`: Find all files greater than 10k in a certain directory.
- `locate ${filepath}`: Find the location of a certain file quickly.
- `grep "foo" ${filepath}`: Searches the word "foo" in a text file.
- `egrep "(foo|bar)" ${filepath}`: Find the words "foo" and "bar" in a text file (this command can do many different [regular expressions](https://regexr.com/)).
- `sed "s/foo/bar/g" ${filepath}`: Find the word "foo" and replace it with a "bar" in a text file.
- `awk '{ print; }' ${filepath}`: Run a custom text-manipulation script for a certain file (learn more about the [awk programming language](https://learnxinyminutes.com/docs/awk/)).

### System management
- `top`: List/view all running processes, memory usage, cpu usage in real-time.
- `ps -ef`: List/view all running services.
- `jobs`: List all active jobs.
- `lsof`: Lists files/sockets opened by running processes.
- `kill ${PID}` or `pkill ${NAME}`: Kill a currently-running program/process.
- `du -sh`: Shows total disk usage of the current directory.
- `df -h`: Shows free and used space on mounted filesystems.
- `free -m`: Show free and used memory and swap space.

### Users & permissions
- `whoami`: Display the name of the currently-logged-in user.
- `chmod ${filepermissions} ${filepath}`: Change the user, group, and other permissions for file.txt.
- `chown ${user}:${group} ${filepath}`: Change the owner of the file and directory.
- `su ${username}`: switch logged-in user to another user, for the current shell.
- `sudo ${any_command}`: Used to run a command as administrator/super-user
- `passwd ${username}`: Used to set or reset a user password.

### Networking
- `ping ${hostname}`: Used to check host reachability.
- `ifconfig`: To check all network interfaces, IPs, and Mac addresses.
- `ssh -i ${sshkeyfile} ${username}@${hostname}`: Used to login remotely to another machine.
- `netstat -ant`: To check all network connections.
- `netstat -ent`: To check established network connections.
- `nslookup ${hostname}`: Used for DNS query.
- `traceroute ${hostname}`: Used for network debugging.
- `scp -r dir ${username}@${hostname}:/opt/`: Copy all files and directories recursively from the local system to a remote system.
- `scp -r ${username}@${hostname}:/opt/ dir/`: Copy all files and directories recursively from the remote system to a local system.
- `rsync -avz localdir ${username}@${hostname}:/backup`: Synchronize files/directories between the local and remote systems.

### Advanced
- `${command1} ; ${command2}`: Perform a followup command.
- `${command1} && ${command2}`: Perform a followup command only if the first one succeeded.
- `${command1} || ${command2}`: Perform a followup command only if the first one failed.
- `if ${condition} ; then ${commands} ; fi`: perform a set of commands only if a condition is true (`0` being true, anything else being false - weird, i know).
- `for ${var} in ${list} ; do ${commands} ; done`: repeat a set of commands in an iterative loop.
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
