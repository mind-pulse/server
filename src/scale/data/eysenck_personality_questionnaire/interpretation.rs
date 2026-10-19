use super::dimensions::{E_DIMENSION, L_DIMENSION, N_DIMENSION, P_DIMENSION};
use super::norms::NORM;
use super::temperaments::{CHOLERIC, MELANCHOLIC, PHLEGMATIC, SANGUINE};
use super::{Dimensions, Interpretation, Temperaments};

pub(super) const INTERPRETATION: Interpretation = Interpretation {
    norm: NORM,
    temperaments: Temperaments {
        sanguine: &SANGUINE,
        choleric: &CHOLERIC,
        melancholic: &MELANCHOLIC,
        phlegmatic: &PHLEGMATIC,
    },
    dimensions: Dimensions {
        e: &E_DIMENSION,
        n: &N_DIMENSION,
        p: &P_DIMENSION,
        l: &L_DIMENSION,
    },
};
