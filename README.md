# AliasBuddy
## Simple CLI Application for Debian and Fedora Based Linux Distributions.

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

1. Download the latest release for your given CPU architecture and distribution of Linux.
2. Install the package using either `dpkg -i alias-buddy<version>.deb` or `dnf install alias-buddy<version>.rpm` ensuring the filename matches the downloaded package.
3. Run by typing `alias_buddy`
