# AliasBuddy
### Simple CLI Application for Debian and Fedora Based Linux Distributions.

# Archival Notice:
- While this project has functional releases, it will not be updated further as of 06/02/2026.
  
What it does:
- Parses the contents of `~/.bash_aliases` for a list of aliases.
- Supported formats:
	- `alias <alias-name>="command"`
	- `function <function-name>() { // Code Here }`
  	- `<function-name>() { // Code Here }`
  
- Displays the commands available.
- Allows for users to select a command from an available list.
---

1. Download the latest release for your given CPU architecture and distribution of Linux.
2. Install the package using either `dpkg -i alias-buddy<version>.deb` or `dnf install alias-buddy<version>.rpm` ensuring the filename matches the downloaded package.
3. Run by typing `alias_buddy`

---
Known limitations:
- Cannot run aliases that require cli input such as:
```
alias example-alias="command-name argument1 argument2 "
example-alias argument3
```
		
