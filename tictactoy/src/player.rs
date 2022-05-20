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
use super::board::{Board, Point};
use colored::*;
use promptly::{prompt, ReadlineError};
use rand::seq::SliceRandom;
use std::str::FromStr;

pub enum State {
    Watching,
    Playing,
}

pub enum PlayTypes {
    Manual,
    Random,
}

#[derive(Eq, PartialEq)]
pub enum Pointers {
    X,
    O,
}

pub struct Player {
    /// State of player
    pub state: State,
    /// Type of player
    pub play_type: PlayTypes,
    /// Pointer of player
    pub pointer: Pointers,
}

impl Player {
    /// Create a new `Player` instance
    pub fn new(play_type: PlayTypes, pointer: Pointers) -> Self {
        Self {
            state: State::Watching,
            play_type,
            pointer,
        }
    }

    /// Make manual move on the board
    fn manual_playing(&self, board: &mut Board) -> Result<(), ReadlineError> {
        loop {
            let point: usize = prompt(format!(
                "({}) Enter a point",
                if self.pointer.is_x() {
                    self.pointer.to_string().blue()
                } else {
                    self.pointer.to_string().red()
                }
            ))?;
            if point > 0 && point <= 9 && board.board[point - 1].is_empty() {
                board.board[point - 1] =
                    Point::from_str(self.pointer.to_string().as_str()).unwrap();
                break;
            } else {
                println!(
                    "Invalid point: {}",
                    format!("You can't play on {point}").red()
                );
            }
        }
        Ok(())
    }

    /// Make raw move on the board
    fn random_playing(&self, board: &mut Board) {
        let point = board
            .board
            .iter()
            .filter(|p| p.is_empty())
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .num()
            .unwrap();
        board.board[point - 1] = Point::from_str(self.pointer.to_string().as_str()).unwrap();
    }

    /// Playing the player (Make the changes on board)
    pub fn play(&self, board: &mut Board) {
        match self.play_type {
            PlayTypes::Manual => self
                .manual_playing(board)
                .unwrap_or_else(|e| panic!("ReadlineError: {}", e.to_string().red())),
            PlayTypes::Random => self.random_playing(board),
        }
    }
}

impl PlayTypes {
    /// Returns list of types
    pub fn types<'a>() -> Vec<&'a str> {
        vec!["Manual", "Random"]
    }
}

impl State {
    /// `true` if the player are watching
    pub fn is_watching(&self) -> bool {
        matches!(self, Self::Watching)
    }

    /// `true` if the player are playing
    pub fn is_playing(&self) -> bool {
        matches!(self, Self::Playing)
    }

    /// Switch the [`State`]
    pub fn switch_state(&self) -> Self {
        if self.is_playing() {
            Self::Watching
        } else {
            Self::Playing
        }
    }
}

impl Pointers {
    /// Returns `true` if is X
    pub fn is_x(&self) -> bool {
        matches!(self, Self::X)
    }
}

impl FromStr for PlayTypes {
    type Err = ();

    fn from_str(str_type: &str) -> Result<Self, Self::Err> {
        match str_type.to_lowercase().as_str() {
            "manual" | "m" => Ok(Self::Manual),
            "random" | "r" => Ok(Self::Random),
            _ => Err(()),
        }
    }
}

impl ToString for Pointers {
    fn to_string(&self) -> String {
        match self {
            Self::X => "X".to_string(),
            Self::O => "O".to_string(),
        }
    }
}
