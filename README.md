Simple CLI Application for Debian and Fedora Based Linux Distributions.

What it does:
- Parses `~/.bash_aliases` for a list of aliases.
- Parses `alias <alias-name>="command"`
- Parses `
  function <function-name>() {
			  // Code Here
		  }
    `
- Parses `<function-name>() { // Code Here }`
- Displays the commands available.
- Allows for users to select a command from an available list.
---

Current Bugs
 - Doesn't work with applications that require a callback, currently only occuring with nano, but will likely occur in other similiar applications.
