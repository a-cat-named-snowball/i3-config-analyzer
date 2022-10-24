# i3-config-analyzer
- Parses i3 config and extracts the list of required executables.
- Capable of printing a list of executables required by your i3 config but not found on your system.
```
Usage: i3-config-analyzer [OPTIONS]

Options:
  -m, --missing                Show only executables that are missing from your system
  -p, --path <I3 CONFIG FILE>  Path of i3 config file, uses default location if omitted
  -h, --help                   Print help information
  -V, --version                Print version information
```
