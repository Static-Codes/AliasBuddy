
This application will be a simple application, likely under 1000 lines of code as it will rely on an external library for CLI input.

What it does:
- Parses `~/.bashrc` and `~/.bash_aliases` for a list of aliases.
- Parses `alias <alias-name>="command"` using regex
- Parses `
  function <function-name>() {
			  // Code Here
		  }
    `
- Displays the commands available, and an indication of if they require superuser permissions.
- Allows for users to select a command from an available list.


