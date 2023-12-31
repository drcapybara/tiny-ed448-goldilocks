#![allow(non_snake_case)]
use crate::curve::{extended_edwards::ExtendedPoint, projective_niels::ProjectiveNielsPoint};
use crypto_bigint::subtle::{ConditionallySelectable, ConstantTimeEq};

pub struct LookupTable([ProjectiveNielsPoint; 8]);

/// Precomputes odd multiples of self
impl From<&ExtendedPoint> for LookupTable {
    fn from(point: &ExtendedPoint) -> LookupTable {
        let P = point.to_extensible();
        let mut table = [P.to_projective_niels(); 8];
        for i in 1..8 {
            table[i] = P.add_projective_niels(&table[i - 1]).to_projective_niels();
        }
        LookupTable(table)
    }
}

impl LookupTable {
    /// Selects a projective niels point from a lookup table in fixed-time
    pub fn select(&self, index: u32) -> ProjectiveNielsPoint {
        let mut result = ProjectiveNielsPoint::identity();
        for i in 1..9 {
            let swap = index.ct_eq(&(i as u32));
            result.conditional_assign(&self.0[i - 1], swap);
        }
        result
    }
}
