use game_theory::GameState;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(1);

macro_rules! reachability {
    ($id:literal) => {
        GameState::from_board_str_unverified(include_str!(concat!(
            "../../data/boards/",
            $id,
            ".txt"
        )))
        .unwrap()
        .verify_reachability(TIMEOUT)
    };
}

#[test]
fn reachable_boards_verification() {
    assert_eq!(reachability!("r1"), Some(true));
    assert_eq!(reachability!("r2"), Some(true));
    assert_eq!(reachability!("r3"), Some(true));
}

#[test]
fn unreachable_boards_verification() {
    assert_eq!(reachability!("u1"), Some(false));
    assert_eq!(reachability!("u2"), Some(false));
    assert_eq!(reachability!("u3"), Some(false));
}
