use game_theory::{p, GameState, Outcome, Player};

fn from_transcript(transcript: &'static str) -> GameState {
    let mut gs = GameState::othello_initial();
    for m in transcript
        .as_bytes()
        .chunks_exact(2)
        .map(|c| p(std::str::from_utf8(c).unwrap()))
    {
        assert_eq!(gs.outcome(), None);
        gs = gs.make_move(m).unwrap();
    }
    gs
}

fn assert_outcome(gs: &GameState, outcome: Outcome, black: usize, white: usize) {
    assert_eq!(gs.outcome(), Some(outcome));
    assert_eq!(gs.discs_of(Player::Black).count(), black);
    assert_eq!(gs.discs_of(Player::White).count(), white);
}

#[test]
fn shortest_possible_game() {
    // Shortest possible game, Manubu Maruo 1975
    let gs = from_transcript("E6F4E3F6G5D6E7F5C5");
    assert_outcome(&gs, Outcome::Winner(Player::Black), 13, 0);
}

#[test]
fn game_finished_in_draw() {
    // Draw game, Othello Quest Online 2018
    let gs = from_transcript("E6F4C3C4D3D6C5C6D7B5B6F7A6A5B4A7F3C8E8C7F6E7G8G6F8F5G4E3D2H3G5G3H4H5H7D8B8A4B3D1C1B1C2E1E2A3B7F2F1G1B2A2G2H6H2G7");
    assert_outcome(&gs, Outcome::Draw, 30, 30);
}

#[test]
fn grand_prix_ghent_2017_hassan_verstuyft() {
    // Hassan 3 – 17 Verstuyft, European Grand Prix Ghent 2017
    let gs = from_transcript("D3E3F4G3F3C5H3F2C4C3E2E1B3H4H5A3");
    assert_outcome(&gs, Outcome::Winner(Player::White), 3, 17);
}

#[test]
fn woc_ghent_2017_vecchi_nicolas() {
    // Vecchi 13 – 45 Nicolas, World Othello Championship Ghent 2017
    let gs = from_transcript("F5D6C5F6C4F4E6D7E7C6F7D8C8E8G5B8E3F8B6B5A6A4A5A7C7B4G6H6H7B3D3C3C2C1H5D2E2F1B1D1B2A3E1H4H3G4G3F3G1F2B7H2H1G7");
    assert_outcome(&gs, Outcome::Winner(Player::White), 13, 45);
}

#[test]
fn woc_mito_2016_nagano_schotte() {
    // Nagano 50 - 14 Schotte, World Othello Championship Mito 2016
    let gs = from_transcript("F5F6E6F4G5E7E3F3C5D3G4G3G6C3D7C6C4D6B5B3B4H4C7D8H5H3F8F7H2A6A5H7F2C8B6A4B7E2G2F1B8A8A7H1H6G7G1D2C1C2D1E1H8G8E8B1A1B2A3A2");
    assert_outcome(&gs, Outcome::Winner(Player::Black), 50, 14);
}

#[test]
fn woc_prague_2018_aunchulee_fukuchi() {
    // Aunchulee 30 - 34 Fukuchi, World Othello Championship Prague 2018
    let gs = from_transcript("D3C5F6F5E6E3C3F3C4B4B3D6C7C2D2C6A4D7G5F4D1G4F7E7G6H5E8C8D8F8E2F2B6A6B5A5A7A3A2F1E1H6G1B2G3B7H4B1A8B8G7H8H7G8A1H2C1H3G2H1");
    assert_outcome(&gs, Outcome::Winner(Player::White), 30, 34);
}

#[test]
fn us_open_2019_chitra_fu() {
    // Chitra 32 - 32 Fu, U.S. Open 2019
    let gs = from_transcript("F5D6C3D3C4F4F6G5E6D7E3G6C7C5C6B6G3B5B4F2A5A6H5C2E8F3E2A3A4B3G4E7D2F7D8D1H6C8F8E1B8H4B7H7G7B2A1A2B1C1F1H8G8A8A7G2H2H3G1H1");
    assert_outcome(&gs, Outcome::Draw, 32, 32);
}
