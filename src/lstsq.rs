/// The sources of inspiration:
/// [https://en.wikipedia.org/wiki/Ordinary_least_squares]
/// [https://www.youtube.com/watch?v=Lx6CfgKVIuE&t=51s]
/// [https://www.youtube.com/watch?v=_cXuvTQl090]
/// [https://youtu.be/MC7l96tW8V8]
use nalgebra as na;
use plotters::prelude::*;
use std::{error::Error, result::Result};

/// Least squares approximation is all about getting the closest "ok" solution
/// from our column space (set of all possible outputs Ax) to the ordinate (b).
/// For instance, we know that there is no such vector x that would
/// satisfy our linear equation Ax=b (i.e. our basis defined in A cannot get us from x to b),
/// so instead of getting the "exact" solution we may find the nearest one,
/// vector x' on the basis of A, to that b vector.
/// To do that we need to minimize the difference||b - Ax'||, where again
/// Ax' is some vector from our column space, let's call it v.
/// So v in order to be as closest as possible to b,
/// must be the projection of b on the column space plane.
/// If A is square and of full rank (num of dim. in input == num of dim. in output),
/// then x is the “exact” solution of the equation.
/// [https://youtu.be/MC7l96tW8V8]
pub fn least_squares_gen(
    a: na::DMatrix<f64>,
    b: na::DVector<f64>,
) -> Result<na::DVector<f64>, Box<dyn Error>> {
    let eps = 1E-6f64;
    // Solve with svd decomposition if the matrix is full rank (exact solution can be found)
    if a.rank(eps) == a.ncols() {
        return Ok(a.svd(true, true).solve(&b, eps)?);
    }
    let a_trans = a.transpose();
    // Return an error if the matrix cannot be inverted
    let invert = (&a_trans * a)
        .try_inverse()
        .ok_or("On inverting a matrix")?;
    Ok(invert * a_trans * b)
}

/// least_squares_ordinary is a solver for a specific case Ax=b,
/// where A is m x 2 matrix and A[j][0] = 1 (j = 1..m), b is m x 1 vector.
/// It's useful for estimating the unknown parameters in a linear regression model
/// This function returns (m, b) in y = mx + b.
pub fn least_squares_ordinary(data_set: &[(f64, f64)]) -> (f64, f64) {
    assert!(
        data_set.len() > 2,
        "Data set must contain at least two points."
    );

    let (x_mean, y_mean) = {
        let (x_sum, y_sum) = data_set
            .iter()
            .fold((0.0f64, 0.0f64), |acc, &(x, y)| (acc.0 + x, acc.1 + y));
        (x_sum / data_set.len() as f64, y_sum / data_set.len() as f64)
    };
    let mut num = 0.0;
    let mut den = 0.0;
    for &(x, y) in data_set {
        num += (x - x_mean) * (y - y_mean);
        den += (x - x_mean) * (x - x_mean);
    }
    let m = num / den;
    (m, y_mean - m * x_mean)
}

// Used as a helper function for least_squares_gen with linear regression problems
#[allow(dead_code)]
fn construct_a_and_b(data_set: &[(f64, f64)]) -> (na::DMatrix<f64>, na::DVector<f64>) {
    let mut a = na::DMatrix::from_element(data_set.len(), 2, 1.0);
    for j in 0..data_set.len() {
        a[(j, 1)] = data_set[j].0;
    }
    let b = na::DVector::from_iterator(data_set.len(), data_set.iter().map(|val| val.1));
    (a, b)
}

// Some hairy code for drawing points and lines
#[allow(dead_code)]
fn plot_linear_regression(
    filename: &str,
    points: Vec<(f64, f64)>,
    m: f64,
    b: f64,
) -> Result<(), Box<dyn Error>> {
    let path = format!("misc/test_output/lstsq_{}.png", filename);
    let root = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_bounds = (points[0].0, points[points.len() - 1].0 + 5.0);
    let y_bounds = (points[0].1 - 5.0, points[points.len() - 1].1 + 5.0);
    let mut chart = ChartBuilder::on(&root)
        .caption("Linear regression", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(x_bounds.0..x_bounds.1, y_bounds.0..y_bounds.1)?;
    chart.configure_mesh().draw()?;

    // And we can draw something in the drawing area
    // Drawing our line segment
    chart
        .draw_series(LineSeries::new(
            points.iter().map(|&(x, _)| (x as f64, x as f64 * m + b)),
            ShapeStyle {
                color: GREEN.to_rgba(),
                filled: false,
                stroke_width: 2,
            },
        ))?
        .label(format!("y = {:.2}x + {:.2}", m, b))
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    // Drawing our scattered  points
    chart
        .draw_series(PointSeries::of_element(
            points,
            3,
            &BLUE,
            &|coords, size, style| {
                EmptyElement::at(coords) + Circle::new((0, 0), size, style.filled())
            },
        ))?
        .label("Data set")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}

#[allow(clippy::unreadable_literal)]
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, Rng};

    #[test]
    fn lstsq_test_least_squares_gen_1() -> Result<(), Box<dyn Error>> {
        /*  a matrix    b vector
            1.0, 1.0,   5.0
            1.0, 2.0,   6.0
            1.0, 3.0,   7.0
            1.0, 4.0,   9.2,
            1.0, 6.0,   11.0
            1.0. 8.0,   10.6,
            1.0  9.2,   15.2,
        */
        let data_set = vec![
            (1.0, 5.0),
            (2.0, 6.0),
            (3.0, 7.0),
            (4.0, 9.2),
            (6.0, 11.0),
            (8.0, 10.6),
            (9.2, 15.2),
        ];
        let (a, b) = construct_a_and_b(&data_set);
        let (m, b) = match least_squares_gen(a, b) {
            Ok(sol) => (sol[1], sol[0]),
            Err(e) => panic!("{}", e),
        };

        // Checking m and b coefficients for correctness
        let eps = 1E-6;
        assert!(
            (m - 1.08255).abs() < eps,
            "The expected m value is 1.08255, the value we got: {}",
            m
        );

        assert!(
            (b - 4.0084749).abs() < eps,
            "The expected b value is 4.0084749, the value we got: {}",
            b
        );

        // Drawing data set points and the approximation function
        plot_linear_regression("test_0", data_set, m, b)?;

        Ok(())
    }

    #[test]
    fn lstsq_test_least_squares_gen_2() {
        // This should be solved with svd decomposition
        let data_set = vec![(2.0, 5.0), (5.0, 15.2)];
        let (a, b) = construct_a_and_b(&data_set);
        if let Err(e) = least_squares_gen(a, b) {
            panic!("{}", e)
        }
    }

    #[test]
    fn lstsq_test_least_squares_ordinary() -> Result<(), Box<dyn Error>> {
        let data_set = vec![
            (0.0, 2.59033617),
            (1.0, 5.95751053),
            (2.0, 8.79550346),
            (3.0, 9.19608787),
            (4.0, 11.80609286),
            (5.0, 16.39060504),
            (6.0, 15.48248325),
            (7.0, 20.22656891),
            (8.0, 19.15524974),
            (9.0, 22.06974761),
        ];
        let (m, b) = least_squares_ordinary(&data_set);

        // Checking m and b coefficients for correctness
        let eps = 1E-8;
        assert!(
            (m - 2.11089638).abs() < eps,
            "The expected m value is 2.11089638, the value we got: {}",
            m
        );
        assert!(
            (b - 3.66798482).abs() < eps,
            "The expected b value is 3.66798482, the value we got: {}",
            b
        );

        // Drawing data set points and the approximation function
        plot_linear_regression("test_1", data_set, m, b)?;

        Ok(())
    }

    #[test]
    fn lstsq_test_least_squares_methods() -> Result<(), Box<dyn Error>> {
        let mut gen = rand::thread_rng();
        // Randomize our data_set
        let data_set: Vec<(f64, f64)> = (0..40)
            .map(|i| (i as f64, i as f64 + gen.gen_range(-5.0, 5.0)))
            .collect();

        let (a, b) = construct_a_and_b(&data_set);
        let (m, b) = match least_squares_gen(a, b) {
            Ok(sol) => (sol[1], sol[0]),
            Err(e) => panic!("{}", e),
        };

        let (m2, b2) = least_squares_ordinary(&data_set);
        let eps = 1E-6;
        assert!(
            (m - m2).abs() < eps && (b - b2).abs() < eps,
            "least_squares_gen() and least_squares_ordinary() result with different outputs: {} {}",
        );

        // Drawing data set points and the approximation function
        plot_linear_regression("test_2", data_set, m, b)?;

        Ok(())
    }
}
