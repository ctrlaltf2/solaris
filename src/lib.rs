use num::{Float, NumCast};

// (1) atan2, but the undefined (0, 0) is defined as 0
pub fn fast_atan2<F: Float>(y: F, x: F) -> F {
    let zero: F = NumCast::from(0).unwrap();
    let pi: F = NumCast::from(std::f64::consts::PI).unwrap();

    if x > zero {
        return (y / x).atan();
    } else if x == zero {
        if y < zero {
            return NumCast::from(-std::f64::consts::PI / 2.).unwrap();
        } else if y == zero {
            return zero;
        } else if y > zero {
            return NumCast::from(std::f64::consts::PI / 2.).unwrap();
        }
    } else if x < zero {
        if y < zero {
            return (y / x).atan() - pi;
        } else if y >= zero {
            return (y / x).atan() + pi;
        }
    }

    return zero;
}

// TODO: placeholder
/// Julian days since epoch year 2000.0.
type JD2000 = f64;
//// Centuries since epoch year 2000.0
type JC2000 = f64;

struct SunPosition {
    /// Compass direction
    azimuth: f64,
    /// altitude angle
    altitude: f64,
}

// placeholder for organization
pub fn solaris_base_impl(
    t_Jd: JD2000,
    t_Jc: JC2000,
    use_approx_R: bool,
    obs_longitude: f64,
    obs_latitude: f64,
) -> SunPosition {
    // (2) Simon et al., 1994
    // mean latitude
    let λ0 = 4.895063168 + 628.331966786 * t_Jc + 5.291838 * 10e-6 * (t_Jc * t_Jc);

    // (3) Chapront-Touze and Chapront, 1988
    // mean anomaly
    let M = 6.240060141 + 628.301955152 * t_Jc + -2.682571 * 10e-6 * (t_Jc * t_Jc);

    // (4) Meeus 1998
    // Sun's equation of the centre
    let C = 3.34161088 * 10e-2 - 8.40725 * 10e-5 * t_Jc - 2.443 * 10e-7 * (t_Jc * t_Jc) * M.sin()
        + (3.489437 * 10e-4 - 1.76278 * 10e-6 * t_Jc) * (2.0 * M).sin();

    // (5) true longitude
    let λ = λ0 + C;

    // (6) Chapront-Touze and Chapront, 1988
    // longitude of the Moon's mean ascending node
    let Ω = 2.1824390725 - 33.7570464271 * t_Jc + 3.622256 * 10e-5 * (t_Jc * t_Jc);

    // (7) Seidelmann, 1982
    // Nutation in longitude
    let delta_ψ = -8.338601 * 10e-5 * Ω.sin();

    // (8) (9) (10) Simon et al., 1994
    let e = 0.016708634 - 4.2037 * 10e-5 * t_Jc - 1.267 * 10e-7 * (t_Jc * t_Jc);
    let v = M + C;
    let R = if use_approx_R {
        1.0000010178 // (11)
    } else {
        1.0000010178 * (1. - e * e) / (1. + e * v.cos())
    };

    // (12) Kovalevsky and Seidelmann, 2004
    // Annual aberration
    let delta_λ = (-9.93087 * 10e-5) / R;

    // (13)
    // Sun's actual correct longitude
    let λ = λ + delta_λ + delta_ψ;

    // (14)
    // Approximated ecliptical latitude of the sun
    const β: f64 = 0.0;

    // (15) Meeus, 1998
    // Mean obliquity
    let ε0 = 0.409092804222 - 2.26965525 * 10e-4 * t_Jc - 2.86 * 10e-9 * (t_Jc * t_Jc);

    // (16) Seidelmann, 1982 leading term only
    // nutation in obliquity
    let delta_ε = 4.4615 * 10e-5 * Ω.cos();

    // (17)
    // obiquity of the ecliptic
    let ε = ε0 + delta_ε;

    // (18) (19) Urban and Seidelmann, 2012
    let cos_ε = ε.cos();
    let sin_ε = ε.sin();
    let tan_β = β.tan();
    let cos_β = β.cos();
    let sin_β = β.sin();
    let sin_λ = λ.sin();
    let cos_λ = λ.cos();

    // right ascension
    let α = fast_atan2(sin_λ * cos_ε - tan_β * sin_ε, cos_λ);
    // declination
    let δ = (sin_β * cos_ε + cos_β * sin_ε * sin_λ).asin();

    // (21)
    // Local sidereal time
    let delta_θ = delta_ψ * cos_ε;
    let θ0 = 4.89496121 + 6.300388098985 * t_Jd + 6.77 * 10e-6 * (t_Jc * t_Jc);
    let θ = θ0 + delta_θ + obs_longitude;

    // (20)
    // Parallactic coordinate hour angle
    let H = θ - α;

    let cos_H = H.cos();
    let cos_δ = δ.cos();
    let sin_δ = δ.sin();
    let cos_lat = obs_latitude.cos();
    let sin_lat = obs_latitude.sin();
    // (24) Urban and Seidalmann, 2012
    // altitude h
    let h = (sin_δ * sin_lat + cos_H * cos_δ * cos_lat).asin();

    let sin_H = H.sin();
    // (25) Urban and Seidalmann, 2012
    // azimuth
    let A = fast_atan2(sin_H, cos_H * sin_lat - (sin_δ / cos_δ) * cos_lat);

    // (26) uncorrected zenith angle
    let ζ = std::f64::consts::FRAC_PI_2 - h;

    // (27) Saemundsson 1986
    // Atmospheric refraction correction
    // Air temperature, (283 K)
    const T: f64 = 283.;
    // Air pressure (101 kPa)
    const P: f64 = 101. * 1000.;
    let delta_h = 0.0002967 * (1.0 / (h + (0.0031376 / (h + 0.0892)))).tan() * (283. / T);

    // Correct altitude
    let h = h + delta_h;

    return SunPosition {
        azimuth: A,
        altitude: h,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_check() {
        // 2000-01-01 as julian day number
        const jd2000_epoch: f64 = 2451544.5;
        // ~2024-03-17T14:48
        const now: f64 = 2460387.166666667;

        // Somewhere on the road in central PA where I'm running this code :)
        const obs_lat: f64 = 40.884305 * (std::f64::consts::PI / 180.);
        const obs_lon: f64 = -77.779766 * (std::f64::consts::PI / 180.);

        const t_Jd: f64 = now - jd2000_epoch;
        const t_Jc: f64 = t_Jd / (365.25 * 100.);

        let sun_pos = solaris_base_impl(t_Jd, t_Jc, false, obs_lat, obs_lon);
        println!(
            "altitude = {}, azimuth = {}",
            sun_pos.altitude * (180. / std::f64::consts::PI),
            sun_pos.azimuth * (180. / std::f64::consts::PI)
        );
    }

    #[test]
    fn fast_atan2_test() {
        let approx_eq = |x: f64, y: f64| -> bool {
            let diff = (y - x).abs();
            println!("diff({}, {}) = {}", x, y, diff);
            diff < 1e-6
        };

        // x < 0
        assert!(approx_eq(fast_atan2::<f64>(-3., -4.), -2.4980915448));
        assert!(approx_eq(fast_atan2::<f64>(0., -4.), 3.14159265359));
        assert!(approx_eq(fast_atan2::<f64>(3., -4.), 2.4980915448));

        // x == 0
        assert!(approx_eq(fast_atan2::<f64>(-3., 0.), -1.57079632679));
        assert!(approx_eq(fast_atan2::<f64>(0., 0.), 0.));
        assert!(approx_eq(fast_atan2::<f64>(3., 0.), 1.57079632679));

        // x > 0
        assert!(approx_eq(fast_atan2::<f64>(-3., 4.), -0.643501108793));
        assert!(approx_eq(fast_atan2::<f64>(0., 4.), 0.));
        assert!(approx_eq(fast_atan2::<f64>(3., 4.), 0.643501108793));
    }
}
