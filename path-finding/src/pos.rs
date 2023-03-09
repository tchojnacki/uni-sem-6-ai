use crate::vec3::Vec3;

/// Latitude (in radians) of Wrocław Market Square.
const CENTER_LAT_RAD: f64 = 0.89204444;

/// Longitude (in radians) of Wrocław Market Square.
const CENTER_LON_RAD: f64 = 0.29724750;

/// Approximate radius of the Earth (in kilometers) near Wrocław Market Square's latitude.
/// Generated using https://rechneronline.de/earth-radius.
const CENTER_RADIUS_KM: f64 = 6365.343;

/// Represents a location inside a coordinate system which:
/// - is centered around Wrocław Market Square, which would have Pos(0, 0)
/// - has perpendicular, normalized bases
/// - measures distances in kilometers
#[derive(Debug, Clone, Copy)]
pub struct Pos(f64, f64);

impl Pos {
    /// Returns the distance between two positions in kilometers.
    pub fn distance_km(self, other: Self) -> f64 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}

/// Helper struct used to convert WSG-84 coordinates to Pos coordinates.
/// Caches the plane normal and orthogonal bases, because they are the same
/// for each coordinate point, yet can't be calculated at compile time.
pub struct PosConverter {
    normal: Vec3,
    e1: Vec3,
    e2: Vec3,
}

impl PosConverter {
    /// Intializes the converter, calculating all of the reusable values.
    pub fn initialize() -> Self {
        let normal = Self::wgs84_to_cartesian(CENTER_LAT_RAD, CENTER_LON_RAD);

        // https://math.stackexchange.com/questions/2450745/finding-orthogonal-vectors-in-a-plane
        let v0 = Vec3(1.0, 0.0, 0.0); // plane base seed, picked arbitrarily
        let e1 = normal.cross(v0).normalized();
        let e2 = normal.cross(e1).normalized();
        Self { normal, e1, e2 }
    }

    /// Converts a pair of WGS-84 coordinates in string form to a Pos struct.
    pub fn wgs84_to_pos(&self, lat: &str, lon: &str) -> Pos {
        let lat = lat
            .parse::<f64>()
            .expect("Invalid lat format.")
            .to_radians();

        let lon = lon
            .parse::<f64>()
            .expect("Invalid lon format.")
            .to_radians();

        self.cartesian_to_local(Self::wgs84_to_cartesian(lat, lon))
    }

    /// Converts from WGS-84 to cartesian coordinate system.
    /// The resulting point is placed relatively to the Earth's core.
    fn wgs84_to_cartesian(lat: f64, lon: f64) -> Vec3 {
        // https://stackoverflow.com/questions/1185408/converting-from-longitude-latitude-to-cartesian-coordinates
        let x = CENTER_RADIUS_KM * lat.cos() * lon.cos();
        let y = CENTER_RADIUS_KM * lat.cos() * lon.sin();
        let z = CENTER_RADIUS_KM * lat.sin();
        Vec3(x, y, z)
    }

    /// Converts from the cartesian coordinate system to the one used by Pos.
    fn cartesian_to_local(&self, v: Vec3) -> Pos {
        // https://www.baeldung.com/cs/3d-point-2d-plane
        let k = -self.normal.dot(v) / self.normal.len_2();
        let zp = v + self.normal * k;

        let s1 = self.e1.dot(zp);
        let s2 = self.e2.dot(zp);
        Pos(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    use super::{Pos, PosConverter, CENTER_RADIUS_KM};

    #[test]
    fn market_square_is_placed_at_origin() {
        let pc = PosConverter::initialize();
        let pos = pc.wgs84_to_pos("51.11038700", "17.03102025");
        assert!(pos.distance_km(Pos(0.0, 0.0)) < 0.01); // 10m
    }

    #[test]
    fn bases_are_orthogonal() {
        let pc = PosConverter::initialize();
        assert!(pc.e1.dot(pc.e2) < 0.01);
    }

    #[test]
    fn bases_are_perpendicular_to_normal() {
        let pc = PosConverter::initialize();
        assert!(pc.normal.dot(pc.e1) < 0.01);
        assert!(pc.normal.dot(pc.e2) < 0.01);
    }

    #[test]
    fn cartesian_coords_are_placed_on_sphere() {
        let v = PosConverter::wgs84_to_cartesian(51.16042707, 17.12241711);
        assert!((v.len() - CENTER_RADIUS_KM).abs() < 10.0); // 10 km
    }
}
