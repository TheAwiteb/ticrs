use std::str::FromStr;
use tictactoy::player::{PlayTypes, Pointers, State};

#[test]
fn state_test() {
    let state: State = State::Watching;

    assert_ne!(state, state.switch_state());
    assert!(state.is_watching());
    assert!(!state.is_playing());
}

#[test]
fn play_type_test() {
    assert_eq!(PlayTypes::types(), vec!["Manual", "Random"]);
    assert_eq!(PlayTypes::from_str("r"), Ok(PlayTypes::Random));
    assert_eq!(PlayTypes::from_str("R"), Ok(PlayTypes::Random));
    assert_eq!(PlayTypes::from_str("random"), Ok(PlayTypes::Random));
    assert_eq!(PlayTypes::from_str("Random"), Ok(PlayTypes::Random));
    assert_eq!(PlayTypes::from_str("m"), Ok(PlayTypes::Manual));
    assert_eq!(PlayTypes::from_str("M"), Ok(PlayTypes::Manual));
    assert_eq!(PlayTypes::from_str("manual"), Ok(PlayTypes::Manual));
    assert_eq!(PlayTypes::from_str("Manual"), Ok(PlayTypes::Manual));
    assert_eq!(PlayTypes::from_str("b"), Err(()));
    assert_eq!(PlayTypes::from_str("bla"), Err(()));
    assert_eq!(PlayTypes::from_str("bla bla"), Err(()));
}

#[test]
fn pointers_test() {
    assert!(Pointers::X.is_x());
    assert!(!Pointers::O.is_x());

    assert_eq!(Pointers::X.to_string(), "X".to_owned());
    assert_eq!(Pointers::O.to_string(), "O".to_owned());
}
