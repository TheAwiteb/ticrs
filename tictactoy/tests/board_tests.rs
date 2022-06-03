use std::str::FromStr;
use tictactoy::board::{Board, Point};
use tictactoy::player::{PlayTypes, Player, Pointers};

#[test]
fn empty_board_test() {
    let board = Board::default();
    let player = Player::new(PlayTypes::Random, Pointers::X);
    let player_2 = Player::new(PlayTypes::Random, Pointers::O);

    assert_eq!(
        board.board,
        vec![
            Point::Empty(1),
            Point::Empty(2),
            Point::Empty(3),
            Point::Empty(4),
            Point::Empty(5),
            Point::Empty(6),
            Point::Empty(7),
            Point::Empty(8),
            Point::Empty(9)
        ]
    );
    assert!(!board.is_finished());
    assert!(board.there_winner(&player, &player_2).is_none());
}

#[test]
fn rows_test() {
    let mut board = Board::default();
    let player = Player::new(PlayTypes::Random, Pointers::X);

    assert_eq!(
        board.rows(),
        vec![
            vec![Point::Empty(1), Point::Empty(2), Point::Empty(3)],
            vec![Point::Empty(4), Point::Empty(5), Point::Empty(6)],
            vec![Point::Empty(7), Point::Empty(8), Point::Empty(9)],
        ]
    );

    assert!(board.is_rows_winner(&player).is_none());
    board.board[0] = Point::X;
    board.board[1] = Point::X;
    board.board[2] = Point::X;
    assert!(board.is_rows_winner(&player).is_some());
}

#[test]
fn columns_test() {
    let mut board = Board::default();
    let player = Player::new(PlayTypes::Random, Pointers::X);

    assert_eq!(
        board.columns(),
        vec![
            vec![Point::Empty(1), Point::Empty(4), Point::Empty(7)],
            vec![Point::Empty(2), Point::Empty(5), Point::Empty(8)],
            vec![Point::Empty(3), Point::Empty(6), Point::Empty(9)],
        ]
    );

    assert!(board.is_columns_winner(&player).is_none());
    board.board[0] = Point::X;
    board.board[3] = Point::X;
    board.board[6] = Point::X;
    assert!(board.is_columns_winner(&player).is_some());
}

#[test]
fn horizontals_test() {
    let mut board = Board::default();
    let player = Player::new(PlayTypes::Random, Pointers::X);

    assert_eq!(
        board.horizontals(),
        vec![
            vec![Point::Empty(1), Point::Empty(5), Point::Empty(9)],
            vec![Point::Empty(3), Point::Empty(5), Point::Empty(7)],
        ]
    );

    assert!(!board.is_finished());
    assert!(board.is_horizontals_winner(&player).is_none());
    board.board[0] = Point::X;
    board.board[4] = Point::X;
    board.board[8] = Point::X;
    assert!(board.is_horizontals_winner(&player).is_some());
}

#[test]
fn from_str_point_test() {
    assert_eq!(Point::from_str("X"), Ok(Point::X));
    assert_eq!(Point::from_str("x"), Ok(Point::X));
    assert_eq!(Point::from_str("O"), Ok(Point::O));
    assert_eq!(Point::from_str("o"), Ok(Point::O));
    assert_eq!(Point::from_str("else"), Err(()));
}

#[test]
fn to_string_point() {
    assert_eq!(Point::Empty(9).to_string(), String::from("9"));
    assert_eq!(Point::X.to_string(), String::from("X"));
    assert_eq!(Point::O.to_string(), String::from("O"));
}

#[test]
fn empty_point_test() {
    assert!(Point::Empty(0).is_empty());
    assert!(!Point::X.is_empty());
}

#[test]
fn point_num_test() {
    assert_eq!(Point::Empty(0).num(), Some(0));
    assert!(Point::X.num().is_none());
}

#[test]
fn finish_board_test() {
    let mut board = Board::default();
    let player_1 = Player::new(PlayTypes::Random, Pointers::X);
    let player_2 = Player::new(PlayTypes::Random, Pointers::O);

    board.board[0] = Point::X;
    board.board[4] = Point::X;
    board.board[8] = Point::X;

    assert!(board.there_winner(&player_1, &player_2).is_some());

    board.board[5] = Point::X;
    board.board[6] = Point::X;
    board.board[7] = Point::X;
    board.board[2] = Point::X;
    board.board[3] = Point::X;
    board.board[1] = Point::X;

    assert!(board.is_finished());
}
