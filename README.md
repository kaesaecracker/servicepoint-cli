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
  reset-everything  Reset both pixels and brightness [aliases: r]
  pixels            Commands for manipulating pixels [aliases: p]
  brightness        Commands for manipulating the brightness [aliases: b]
  text              Commands for sending text to the screen [aliases: t]
  help              Print this message or the help of the given subcommand(s)

Options:
  -d, --destination <DESTINATION>  ip:port of the servicepoint display [default: 127.0.0.1:2342]
  -t, --transport <TRANSPORT>      protocol to use for communication with display [default: udp] [possible values: udp, web-socket, fake]
  -v, --verbose                    verbose logging
  -h, --help                       Print help
  -V, --version                    Print version
```

### Pixels

```
Commands for manipulating pixels

Usage: servicepoint-cli pixels <COMMAND>

Commands:
  off     Reset all pixels to the default (off) state [aliases: r, reset, clear]
  flip    Invert the state of all pixels [aliases: f]
  on      Set all pixels to the on state
  image   Send an image file (e.g. jpeg or png) to the display. [aliases: i]
  screen  Stream the default screen capture source to the display. On Linux Wayland, this pops up a screen or window chooser, but it also may directly start streaming your main screen. [aliases: s]
```

#### Image

```
Send an image file (e.g. jpeg or png) to the display.

Usage: servicepoint-cli pixels image [OPTIONS] <FILE_NAME>

Arguments:
  <FILE_NAME>  

Options:
      --no-hist     Disable histogram correction
      --no-blur     Disable blur
      --no-sharp    Disable sharpening
      --no-dither   Disable dithering. Brightness will be adjusted so that around half of the pixels are on.
      --no-spacers  Do not remove the spacers from the image.
      --no-aspect   Do not keep aspect ratio when resizing.
```

#### Screen

```
Stream the default screen capture source to the display. On Linux Wayland, this pops up a screen or window chooser, but it also may directly start streaming your main screen.

Usage: servicepoint-cli pixels screen [OPTIONS]

Options:
  -p, --pointer     Show mouse pointer in video feed
      --no-hist     Disable histogram correction
      --no-blur     Disable blur
      --no-sharp    Disable sharpening
      --no-dither   Disable dithering. Brightness will be adjusted so that around half of the pixels are on.
      --no-spacers  Do not remove the spacers from the image.
      --no-aspect   Do not keep aspect ratio when resizing.
```

### Brightness

```
Commands for manipulating the brightness

Usage: servicepoint-cli brightness <COMMAND>

Commands:
  max   Reset brightness to the default (max) level [aliases: r, reset]
  set   Set one brightness for the whole screen [aliases: s]
  min   Set brightness to lowest possible level.
```

### Text

```
Commands for sending text to the screen

Usage: servicepoint-cli text <COMMAND>

Commands:
  stdin  Pipe text to the display, example: `journalctl | servicepoint-cli stream stdin`
```

#### Stdin

```
Pipe text to the display, example: `journalctl | servicepoint-cli stream stdin`

Usage: servicepoint-cli stream stdin [OPTIONS]

Options:
  -s, --slow  Wait for a short amount of time before sending the next line
```


## Contributing

If you have ideas on how to improve the code, add features or improve documentation feel free to open a pull request.

You think you found a bug? Please open an issue.

Submissions on [Forgejo](https://git.berlin.ccc.de/servicepoint/servicepoint-cli) are preferred, but you can also use [GitHub](https://github.com/kaesaecracker/servicepoint-cli). 

All creatures welcome.

## License

This code is licensed under [GNU General Public License v3.0 or later](https://www.gnu.org/licenses/gpl-3.0-standalone.html).
