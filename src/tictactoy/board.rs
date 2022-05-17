///                       Rust Tic Tac Toy (x/o)
///                 Copyright (C) 2020-2022  TheAwiteb
///                 https://github.com/TheAwiteb/tic-rs
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
use super::player::{Player, Pointers};

use comfy_table::presets::UTF8_NO_BORDERS;
use comfy_table::{Cell, Color, ContentArrangement, Row, Table};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Point {
    X,
    O,
    Empty(u8),
}

impl Point {
    /// Returns `Point` instance from string
    pub fn from_str(str_point: &str) -> Option<Self> {
        match str_point.to_lowercase().as_str() {
            "x" => Some(Self::X),
            "o" => Some(Self::O),
            _ => None,
        }
    }

    /// Returns point num if it is empty
    pub fn num(&self) -> Option<usize> {
        match self {
            Self::X => None,
            Self::O => None,
            Self::Empty(n) => Some(n.to_string().parse().unwrap()),
        }
    }

    /// Returns color of point
    pub fn color(&self) -> Color {
        match self {
            Self::X => Color::Blue,
            Self::O => Color::Red,
            Self::Empty(_) => Color::DarkGrey,
        }
    }

    /// Returns cell of point
    pub fn to_cell(self) -> Cell {
        Cell::new(self.to_string()).fg(self.color())
    }

    /// Returns `true` if is empty
    pub fn is_empty(&self) -> bool {
        matches!(self, Point::Empty(_))
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        match self {
            Self::X => "X".to_string(),
            Self::O => "O".to_string(),
            Self::Empty(n) => n.to_string(),
        }
    }
}

pub struct Board {
    pub board: Vec<Point>,
}

impl Board {
    /// Create a new `Board` instance
    pub fn new() -> Self {
        Self {
            board: (1..=9).map(Point::Empty).collect(),
        }
    }

    /// Returns all rows
    pub fn rows(&self) -> Vec<Vec<Point>> {
        self.board.chunks(3).map(|points| points.to_vec()).collect()
    }

    /// Returns all columns
    pub fn columns(&self) -> Vec<Vec<Point>> {
        let mut columns: Vec<Vec<Point>> = vec![vec![], vec![], vec![]];
        for row in self.rows() {
            columns[0].push(row[0]);
            columns[1].push(row[1]);
            columns[2].push(row[2]);
        }
        columns
    }

    /// Returns the bla
    pub fn horizontals(&self) -> Vec<Vec<Point>> {
        vec![
            vec![self.board[0], self.board[4], self.board[8]],
            vec![self.board[2], self.board[4], self.board[6]],
        ]
    }

    /// Returns `Pointers` instance if there win in rows
    fn is_rows_winner<'a>(&self, player: &'a Player) -> Option<&'a Pointers> {
        let rows = self.rows();
        let player_point = Point::from_str(player.pointer.to_string().as_str()).unwrap();

        if rows
            .iter()
            .any(|points| points.iter().all(|point| point == &player_point))
        {
            Some(&player.pointer)
        } else {
            None
        }
    }

    /// Returns `Pointers` instance if there win in columns
    fn is_columns_winner<'a>(&self, player: &'a Player) -> Option<&'a Pointers> {
        let columns = self.columns();
        let player_point = Point::from_str(player.pointer.to_string().as_str()).unwrap();

        if columns
            .iter()
            .any(|points| points.iter().all(|point| point == &player_point))
        {
            Some(&player.pointer)
        } else {
            None
        }
    }

    /// Returns `Pointers` instance if there win in bla
    fn is_horizontals_winner<'a>(&self, player: &'a Player) -> Option<&'a Pointers> {
        let player_point = Point::from_str(player.pointer.to_string().as_str()).unwrap();
        if self
            .horizontals()
            .iter()
            .any(|horizontals| horizontals.iter().all(|point| point == &player_point))
        {
            Some(&player.pointer)
        } else {
            None
        }
    }

    /// Returns the winner
    pub fn there_winner<'a>(
        &self,
        player_1: &'a Player,
        player_2: &'a Player,
    ) -> Option<&'a Pointers> {
        if self.is_rows_winner(player_1).is_some()
            || self.is_columns_winner(player_1).is_some()
            || self.is_horizontals_winner(player_1).is_some()
        {
            Some(&player_1.pointer)
        } else if self.is_rows_winner(player_2).is_some()
            || self.is_columns_winner(player_2).is_some()
            || self.is_horizontals_winner(player_2).is_some()
        {
            Some(&player_2.pointer)
        } else {
            None
        }
    }

    /// Returns `true` if the game is finished
    pub fn is_finished(&self) -> bool {
        self.board.iter().all(|point: &Point| !point.is_empty())
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut table = Table::new();
        table
            .load_preset(UTF8_NO_BORDERS)
            .set_table_width(3)
            .set_content_arrangement(ContentArrangement::Dynamic);

        for points in self.board.chunks(3) {
            let mut row: Row = Row::new();
            for point in points {
                row.add_cell(point.to_cell());
            }
            table.add_row(row);
        }
        table.to_string()
    }
}
