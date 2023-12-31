#![allow(non_snake_case)]
use super::{
    extended_edwards::ExtendedPoint, field::field_element::FieldElement,
    twisted_edwards::TwistedPoint,
};
use crypto_bigint::subtle::{Choice, ConditionallyNegatable, ConditionallySelectable};

// Variant of Niels, where a Z coordinate is added for unmixed readdition
// ((y+x)/2, (y-x)/2, dxy, Z)
#[derive(Copy, Clone)]
pub struct ProjectiveNielsPoint {
    pub Y_plus_X: FieldElement,
    pub Y_minus_X: FieldElement,
    pub Td: FieldElement,
    pub Z: FieldElement,
}

impl ProjectiveNielsPoint {
    pub fn identity() -> ProjectiveNielsPoint {
        TwistedPoint::identity().to_projective_niels()
    }

    pub fn double(&self) -> ProjectiveNielsPoint {
        self.to_extended()
            .to_extensible()
            .double()
            .to_projective_niels()
    }

    pub fn to_extended(&self) -> ExtendedPoint {
        let A = self.Y_plus_X - self.Y_minus_X;
        let B = self.Y_plus_X + self.Y_minus_X;
        ExtendedPoint {
            X: self.Z * A,
            Y: self.Z * B,
            Z: self.Z.square(),
            T: B * A,
        }
    }
}

impl ConditionallySelectable for ProjectiveNielsPoint {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        ProjectiveNielsPoint {
            Y_plus_X: FieldElement::conditional_select(&a.Y_plus_X, &b.Y_plus_X, choice),
            Y_minus_X: FieldElement::conditional_select(&a.Y_minus_X, &b.Y_minus_X, choice),
            Td: FieldElement::conditional_select(&a.Td, &b.Td, choice),
            Z: FieldElement::conditional_select(&a.Z, &b.Z, choice),
        }
    }
}

impl ConditionallyNegatable for ProjectiveNielsPoint {
    fn conditional_negate(&mut self, choice: Choice) {
        FieldElement::conditional_swap(&mut self.Y_minus_X, &mut self.Y_plus_X, choice);
        self.Td.conditional_negate(choice);
    }
}
