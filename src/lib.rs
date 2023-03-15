/// Represents a point on the globe.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl From<(f64, f64)> for Location {
    fn from(latlon: (f64, f64)) -> Self {
        Location {
            latitude: latlon.0,
            longitude: latlon.1,
        }
    }
}

impl From<(f32, f32)> for Location {
    fn from(latlon: (f32, f32)) -> Self {
        Location {
            latitude: latlon.0 as f64,
            longitude: latlon.1 as f64,
        }
    }
}

impl Location {
    /// Values from Moritz, H. Journal of Geodesy (2000) 74: 128. https://doi.org/10.1007/s001900050278
    const KILOMETERS: f64 = 6371.0087714;
    const MILES: f64 = 3958.76131603933;
    const NAUTICAL_MILES: f64 = Self::MILES * 1.1508;

    /// Calculates the distance in miles between two points.
    pub fn distance_mi(&self, other: Location) -> f64 {
        Self::MILES * self.distance(other)
    }

    /// Calculates the distance in nautical miles between two points.
    pub fn distance_nautical_mi(&self, other: Location) -> f64 {
        Self::NAUTICAL_MILES * self.distance(other)
    }

    /// Calculates the distance in kilometers between two points.
    pub fn distance_km(&self, other: Location) -> f64 {
        Self::KILOMETERS * self.distance(other)
    }

    /// Performs the haversine calculation without multiplying by the unit length.
    fn distance(&self, other: Location) -> f64 {
        let d_lat: f64 = (other.latitude - self.latitude).to_radians();
        let d_lon: f64 = (other.longitude - self.longitude).to_radians();
        let lat1: f64 = self.latitude.to_radians();
        let lat2: f64 = other.latitude.to_radians();

        let a: f64 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
            + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
        let c: f64 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location_from_f64() {
        let loc_tup64: Location = (38.898556, -77.037852).into();
        let loc_struct = Location {
            latitude: 38.898556,
            longitude: -77.037852,
        };
        assert_eq!(loc_tup64, loc_struct);
    }

    #[test]
    fn location_from_f32() {
        // Location uses f64, and converting from f32 will cause some change in representation
        let loc_tup32: Location = (38.898556_f32, -77.037852_f32).into();
        let loc_struct = Location {
            latitude: 38.898555755615234,
            longitude: -77.03784942626953,
        };
        assert_eq!(loc_tup32, loc_struct);
    }

    #[test]
    fn distance_in_miles() {
        let start: Location = (38.898556, -77.037852).into();
        let end: Location = (38.897147, -77.043934).into();
        assert_eq!(0.3412300584989182, start.distance_mi(end));
    }

    #[test]
    fn istance_in_kilometers() {
        let start: Location = (38.898556, -77.037852).into();
        let end: Location = (38.897147, -77.043934).into();
        assert_eq!(0.549156547264883, start.distance_km(end));
    }
}
