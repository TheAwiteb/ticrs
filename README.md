# Tic-rs
```
                       Rust Tic Tac Toy (x/o)
                 Copyright (C) 2020-2022  TheAwiteb
                 https://github.com/TheAwiteb/tic-rs

This program is free software: you can redistribute it and/ormodify it under
the terms of the GNU Affero General Public License as published bythe Free
Software Foundation, either version 3 of the License, or (at youroption)
any later version.

This program is distributed in the hope that it will be useful,but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITYor FITNESS
FOR A PARTICULAR PURPOSE.  See the GNU Affero General PublicLicense for more
details.

You should have received a copy of the GNU Affero General PublicLicense along
with this program.  If not, see <http://www.gnu.org/licenses/>.
```

## Requirements
 * [Rust](https://www.rust-lang.org/)

## Features

### Considering the user experience
Providing an exciting experience in the command line interface with the [`clap`] and awesome coordination of the interface with the [`comfy-table`] as it provided a beautiful table and of course the harmonious colors [`colored`] by the color added a really special touch

[`clap`]: https://crates.io/crates/clap
[`comfy-table`]: https://crates.io/crates/comfy-table
[`colored`]: https://crates.io/crates/colored

### Easy to play and control
As you only need to activate `-c/--clean-window` to clean the interface after each round

## Install
```bash
# Clone the repo
git clone https://github.com/TheAwiteb/tic-rs
# change directory to it
cd tic-rs
# Build it with cargo
cargo build --release
# Move the binary to /usr/bin (Unix like system) (need permission to move (not run))
sudo mv ./target/release/tic-rs /usr/bin/
# Run help message
tic-rs --help
```

## Using
```
USAGE:
    tic-rs [OPTIONS]

OPTIONS:
    -c, --clean-window    Clean the window every round
    -h, --help            Print help information
    -V, --version         Print version information
```

## Images
|                    CLI                      |                    Game                    |
|:-------------------------------------------:|:------------------------------------------:|
| ![Help message](https://i.suar.me/nBKvE/l)  | ![Without clen](https://i.suar.me/XpYQm/l) |
| ![Option fixer](https://i.suar.me/5nJ49/l)  | ![X is Winner](https://i.suar.me/WQZg0/l) |

## License
GNU Affero General Public License version 3 of the license for more see http://www.gnu.org/licenses/