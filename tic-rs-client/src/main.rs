/// Rust Tic Tac Toy (x/o)
/// Copyright (C) 2020-2022  TheAwiteb
/// <https://github.com/TheAwiteb/tic-rs>
///
/// This program is free software: you can redistribute it and/or modify it under
/// the terms of the GNU Affero General Public License as published by the Free
/// Software Foundation, either version 3 of the License, or (at your option)
/// any later version.
///
/// This program is distributed in the hope that it will be useful, but WITHOUT
/// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
/// FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
/// details.
///
/// You should have received a copy of the GNU Affero General Public License along
/// with this program.  If not, see <http://www.gnu.org/licenses/>.
mod cli;
mod utils;

use colored::Colorize;
use promptly::ReadlineError;
use std::process::Command;
use tictactoy::board::*;
use tictactoy::player::*;
use utils::get_player;

/// Main client function
fn main() -> Result<(), ReadlineError> {
    let app: cli::CliApp = cli::parse();

    let mut player_x: Player = get_player(Pointers::X, &app)?;
    let player_o: Player = get_player(Pointers::O, &app)?;
    let mut board: Board = Board::new();

    // Game loop
    loop {
        if app.clean_window {
            Command::new("cls")
                .status()
                .or_else(|_| Command::new("clear").status())
                .unwrap_or_else(|e| panic!("failed to clear window: {}", e));
        }
        println!("{}\n\n", board.to_string());
        if let Some(winner) = board.there_winner(&player_x, &player_o) {
            println!(
                "{} ✨ 🎉",
                format!("The winner is player {}", winner.to_string()).green()
            );
            break;
        } else if board.is_finished() {
            println!("Game over in a {} 👏 👏", "draw".bold());
            break;
        } else {
            if player_x.state.is_watching() {
                player_x.play(&mut board)
            } else {
                player_o.play(&mut board)
            }
            player_x.state = player_x.state.switch_state();
        }
    }

    Ok(())
}
