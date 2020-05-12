/// This module contains two famous equations, parts of the fourier transform,
/// implemented in a straightforward manner.
/// I'll be honest, this whole fourier thing still pretty much confuses me;
/// gotta take a closer look someday on it.
/// Sources: [https://betterexplained.com/articles/an-interactive-guide-to-the-fourier-transform/]

pub const TAU: f32 = 6.283_185_5;
pub struct Complex(f32, f32);

pub fn transform(data: &[f32]) -> Vec<Complex> {
    let n = data.len();
    let mut result = Vec::with_capacity(n);
    for freq in 0..n {
        let mut comp = Complex(0.0, 0.0);
        let rate = TAU * freq as f32 / n as f32;
        for (t, amplitude) in data.iter().enumerate() {
            let distance = rate * t as f32;
            comp.0 += amplitude * distance.cos();
            comp.1 += amplitude * distance.sin();
        }
        if comp.0.abs() < 1E-6 {
            comp.0 = 0.0;
        }
        if comp.1.abs() < 1E-6 {
            comp.1 = 0.0;
        }
        result.push(comp);
    }
    result
}

pub fn inverse_transform(data: &[f32]) -> Vec<Complex> {
    let n = data.len();
    let mut result = Vec::with_capacity(n);
    for freq in 0..n {
        // τ is there, since as t goes from 0 to 1, and
        // it needs to cover a distance of τ along the circle (circumference).
        // In conjunction with freq, the expression describes how much distance is covered in 1 sec.
        let mut comp = Complex(0.0, 0.0);
        let rate = -TAU * freq as f32 / n as f32;
        for (t, amplitude) in data.iter().enumerate() {
            let distance = rate * t as f32;
            comp.0 += amplitude * distance.cos();
            comp.1 += amplitude * distance.sin();
        }
        if comp.0.abs() < 1E-6 {
            comp.0 = 0.0;
        }
        if comp.1.abs() < 1E-6 {
            comp.1 = 0.0;
        }
        // Averaging terms.
        comp.0 /= n as f32;
        comp.1 /= n as f32;
        result.push(comp);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Not really a unit test, but more like a showcase.
    fn fourier_transform_showcase() {
        let data_points = &[8.0, 6.0, 7.0, 11.0, 2.0, 0.0, 1.0, 8.0, 3.0];
        let output = transform(data_points);
        println!("Transform:");
        for val in output {
            println!("{}\t{}i", val.0, val.1);
        }
        let output = inverse_transform(data_points);
        println!("Inverse transform:");
        for val in output {
            println!("{}\t{}i", val.0, val.1);
        }
    }
}
