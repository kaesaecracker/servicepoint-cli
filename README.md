# servicepoint-cli

This repository contains a command line interface for the ServicePoint display.

To send commands, this uses the [servicepoint crate](https://crates.io/crates/servicepoint).

## Installation with cargo

```shell
cargo install servicepoint-cli
```

If you have set your PATH to include the ~/.cargo/bin, you can now run `servicepoint-cli`.

## Running with nix

```shell
# from CCCB Forgejo
nix run git+https://git.berlin.ccc.de/servicepoint/servicepoint-cli.git -- <args>

# from GitHub mirror
nix run github:kaesaecracker/servicepoint-cli -- <args>
```


## Running a debug build

```shell
git clone https://git.berlin.ccc.de/servicepoint/servicepoint-cli.git
cd servicepoint-cli
cargo run -- <args>
```

## Usage

```
Usage: servicepoint-cli [OPTIONS] <COMMAND>

Commands:
  reset-everything  [aliases: r]
  pixels            [aliases: p]
  brightness        [aliases: b]
  stream            [aliases: s]
  help              Print this message or the help of the given subcommand(s)

Options:
  -d, --destination <DESTINATION>  ip:port of the servicepoint display [default: 127.0.0.1:2342]
  -t, --transport <TRANSPORT>      protocol to use for communication with display [default: udp] [possible values: udp, web-socket, fake]
  -v, --verbose                    verbose logging
  -h, --help                       Print help
  -V, --version                    Print version
```

### Stream

```
Usage: servicepoint-cli stream <COMMAND>

Commands:
  stdin   
  screen  
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Brightness

```
Usage: servicepoint-cli brightness <COMMAND>

Commands:
  reset  [aliases: r]
  set    [aliases: s]
  min    
  max    
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Pixels
```
Usage: servicepoint-cli pixels <COMMAND>

Commands:
  reset  [aliases: r]
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Contributing

If you have ideas on how to improve the code, add features or improve documentation feel free to open a pull request.

You think you found a bug? Please open an issue.

Submissions on [Forgejo](https://git.berlin.ccc.de/servicepoint/servicepoint-cli) are preferred, but you can also use [GitHub](https://github.com/kaesaecracker/servicepoint-cli). 

All creatures welcome.

## License

This code is licensed under [GNU General Public License v3.0 or later](https://www.gnu.org/licenses/gpl-3.0-standalone.html).
