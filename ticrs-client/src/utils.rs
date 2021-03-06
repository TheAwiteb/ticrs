/// Rust Tic Tac Toy (x/o)
/// Copyright (C) 2020-2022  TheAwiteb
/// <https://github.com/TheAwiteb/ticrs>
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
use super::cli::CliApp;
use colored::Colorize;
use promptly::{prompt, ReadlineError};
use std::str::FromStr;
use tictactoy::player::{PlayTypes, Player, Pointers};

/// Ask user to add player
pub fn get_player(pointer: Pointers, app: &CliApp) -> Result<Player, ReadlineError> {
    if app.randomly || app.manually {
        if app.randomly {
            Ok(Player::new(PlayTypes::Random, pointer))
        } else {
            Ok(Player::new(PlayTypes::Manual, pointer))
        }
    } else {
        let types: Vec<&str> = PlayTypes::types();
        loop {
            let user_input: String = prompt(format!(
                "Enter type of player {} ({})",
                pointer.to_string(),
                types
                    .iter()
                    .map(|t| format!(
                        "{}/{}",
                        t.chars()
                            .next()
                            .unwrap_or_else(|| panic!("Invalid type {t}")),
                        t
                    ))
                    .collect::<Vec<_>>()
                    .join(", "),
            ))?;
            if let Ok(play_type) = PlayTypes::from_str(user_input.as_ref()) {
                return Ok(Player::new(play_type, pointer));
            } else {
                eprintln!(
                    "{}: '{}' is not play type",
                    "Type Error".red(),
                    user_input.bold()
                )
            }
        }
    }
}
