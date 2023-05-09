const ELO_K: f64 = 32.;
pub const INITIAL_ELO: i32 = 1000;

fn elo_probability(loser: i32, winner: i32) -> f64 {
    1. / (1. + 10f64.powf((loser as f64 - winner as f64) / 400.))
}

pub fn elo_update(ratings: &mut [i32], wi: usize, li: usize) {
    let pw = elo_probability(ratings[li], ratings[wi]);
    let pl = elo_probability(ratings[wi], ratings[li]);
    ratings[wi] += (ELO_K * (1. - pw)) as i32;
    ratings[li] += (-pl * ELO_K) as i32;
}
