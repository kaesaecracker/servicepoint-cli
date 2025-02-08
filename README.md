# servicepoint-cli

This repository contains a command line interface for the ServicePoint display.

To send commands, this uses the [servicepoint crate](https://crates.io/crates/servicepoint).

## Running

Any OS:
```shell
git clone https://git.berlin.ccc.de/servicepoint/servicepoint-cli.git
cd servicepoint-cli
cargo run -- <args>
```

Using nix:

```shell
# from CCCB Forgejo
nix run git+https://git.berlin.ccc.de/servicepoint/servicepoint-cli.git -- <args>

# from GitHub mirror
nix run github:kaesaecracker/servicepoint-cli -- <args>
```

## Contributing

If you have ideas on how to improve the code, add features or improve documentation feel free to open a pull request.

You think you found a bug? Please open an issue.

Submissions on Forgejo are preferred, but you can also use GitHub. 

All creatures welcome.

## License

This code is licensed under [GNU General Public License v3.0 or later](https://www.gnu.org/licenses/gpl-3.0-standalone.html).
