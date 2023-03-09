use crate::{pos::PosConverter, time::Time};

mod pos;
mod time;
mod vec3;

fn main() {
    let pc = PosConverter::initialize();
    let p0 = pc.wgs84_to_pos("51.16042707", "17.12241711");
    let p1 = pc.wgs84_to_pos("51.16201253", "17.12469012");
    let p2 = pc.wgs84_to_pos("51.11038700", "17.03102025");

    println!("Rynek: {p2:?}");
    println!("Odległość na Zakrzowie: {:.3} km", p0.distance_km(p1));

    println!("{}", Time::from("00:15:00"));
}
