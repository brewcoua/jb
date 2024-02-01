# brewcoua/jb
This is a simple, unofficial CLI for installing, updating and configuring JetBrains IDEs from the command line.

## Installation
Either download the latest release from the `TODO` page or build it yourself using `cargo build --release`.

## Usage
```
USAGE:
    jb [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Print help
    -V, --version    Print version
    -v, --verbose    Enable verbose logging

OPTIONS:
    -d, --directory <directory>    Set the installation directory

SUBCOMMANDS:
    install    Install a JetBrains tool
    uninstall  Uninstall a JetBrains tool
    list       List installed JetBrains tools
    link       Link a JetBrains tool to the PATH
    unlink     Unlink a JetBrains tool from the PATH
    help       Print help
```

Some flags and options can be set using environment variables:
- `JB_TOOLS_DIR` - The installation directory (can be overridden using the `-d` option) <br />
(default: `$HOME/.local/share/jetbrains`)
- `JB_VERBOSE` - Enable verbose logging (can be overridden using the `-v` flag)
- `JB_LOG` - Set the log level (can be overridden using the `-v` flag)

## Disclaimer
This is not an official JetBrains project.
I am not affiliated with JetBrains in any way.
This is just a simple script I wrote to make my life easier.
I am not responsible for any damage this script may cause. Use at your own risk.

## FAQ
1. I don't see my IDE in the list of available IDEs. What do I do?
    - You can create an issue on GitHub and I will add it to the list.
    - Some IDEs are not available because they do not offer a linux version (e.g. ReSharper).

## License
This project is licensed under either of the following, at your option:
- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, 
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
