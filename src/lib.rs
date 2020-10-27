

use std::ops::Shr;
use num_complex::Complex64;
use num::zero;
use std::f64::consts::PI;

pub enum Direction {
    Forward,
    Inverse
}

pub fn fft(samples: &[Complex64], dir: Direction) -> Result<Vec<Complex64>, String> {
    let n = samples.len() as usize;
    if !n.is_power_of_two() && n > 0
    {
        return Err(format!("Sample length must be power of 2, was {}!", n));
    }

    let mut spec = Vec::with_capacity(n);
    if n == 0 {
        return Ok(spec);
    }

    let exponent = (n - 1).count_ones() as usize;
    let shift = std::mem::size_of::<usize>() * 8 - exponent;
    let dir = match dir {
        Direction::Forward => -2.,
        Direction::Inverse => 2.
    };

    // bit reversal
    spec.resize(n, zero());
    for idx in 0..n {
        let reversed = idx.reverse_bits().shr(shift);
        spec[reversed] = Complex64::from(samples[idx]);
    }

    let mut block_size: usize = 1;
    for _ in 0..exponent {
        let butterfly_offset = block_size;
        block_size *= 2;

        let wn = Complex64::new(0., dir * PI / (block_size) as f64).exp();

        for block_index in (0..n).step_by(block_size) {
            let mut w = Complex64::from(1.);
            for block_offset in 0..butterfly_offset {
                let index0 = block_index + block_offset;
                let index1 = block_index + block_offset + butterfly_offset;

                spec[index0] = spec[index0] + spec[index1] * w;
                spec[index1] = spec[index0] - spec[index1] * w;

                w *= wn;
            }
        }
    }

    Ok(spec)
}