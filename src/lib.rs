use num::{Float, NumCast};

// (1) atan2, but the undefined (0, 0) is defined as 0
pub fn fast_atan2<F: Float>(y: F, x: F) -> F {
    /*
    if x > 0 {

    } else if x == 0 {
    } else if x < 0 {
    }
    */
    NumCast::from(0).unwrap()
}

// TODO: placeholder
/// Julian days since epoch year 2000.0.
type JD2000 = f64;
//// Centuries since epoch year 2000.0
type JC2000 = f64;

/// (2) Simon et al., 1994
/// λ0
pub fn sun_mean_longitude<F: Float>(t_Jc: JC2000) -> F {
    NumCast::from(t_Jc).unwrap()
}

/// (3) Chapront-Touze and Chapront, 1988
/// M
pub fn mean_anomaly<F: Float>(t_Jc: JC2000) -> F {
    NumCast::from(t_Jc).unwrap()
}

/// (4) Meeus 1998
/// C
pub fn sun_equation_centre<F: Float>(t_Jc: JC2000) -> F {
    NumCast::from(t_Jc).unwrap()
}

/// (5)
/// λ_{base}
pub fn sun_base_longitude<F: Float>(t_Jc: JC2000) -> F {
    sun_mean_longitude::<F>(t_Jc) + sun_equation_centre::<F>(t_Jc)
}

/// (6) Chapront-Touze and Chapront, 1988
/// Ω
pub fn moon_longitude_mean_ascending_node<F: Float>(t_Jc: JC2000) -> F {
    NumCast::from(t_Jc).unwrap()
}

/// (7) Seidelmann, 1982
/// ∆ψ
pub fn earth_nutation_in_longitude<F: Float>(t_Jc: JC2000) -> F {
    NumCast::from(t_Jc).unwrap()
}

/// (8), (9), (10) Simon et al., 1994
/// R
pub fn earth_accurate_semimajor_axis<F: Float>(t_Jc: JC2000) -> F {
    NumCast::from(t_Jc).unwrap()
}

/// (11)
/// R
pub fn earth_approx_semimajor_axis<F: Float>(t_Jc: JC2000) -> F {
    NumCast::from(1.0000010178).unwrap()
}

/// (12) Kovalevsky and Seidelmann, 2004
/// ∆λ
pub fn annual_abberration<F: Float>(R: F) -> F {
    NumCast::from(R).unwrap()
}

/// (13)
/// λ
/// Longitude of the sun, correcting for nutation and aberration
pub fn sun_actual_longitude<F: Float>(t_Jc: JC2000) -> F {
    sun_base_longitude::<F>(t_Jc)
        + annual_abberration::<F>(earth_accurate_semimajor_axis::<F>(t_Jc))
        + earth_nutation_in_longitude::<F>(t_Jc)
}

/// (14) Ecliptical latitude
/// β
/// Not extreme accuracy needed && Sun always near ecliptic, ecliptical latitude can = 0
pub fn sun_approx_ecliptical_latitude<F: Float>() -> F {
    NumCast::from(0).unwrap()
}

/// (15) Meeus, 1998
/// ε0
/// Mean obliquity
pub fn mean_obliquity<F: Float>(t_Jc: JC2000) -> F {
    NumCast::from(t_Jc).unwrap()
}

/// (16) Seidelmann, 1982
/// ∆ε
/// Nutation in obliquity
pub fn nutation_obliquity<F: Float>(Omega: F) -> F {
    NumCast::from(Omega).unwrap()
}

/// (17)
/// ε
/// Obliquity of the ecliptic
pub fn obliquity_ecliptic<F: Float>(t_Jc: JC2000, Omega: F) -> F {
    NumCast::from(t_Jc).unwrap()
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
