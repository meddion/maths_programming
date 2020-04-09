/// The main source of inspiration [https://brilliant.org/wiki/linear-programming/]
use nalgebra as na;

/// The simplex algorithm itself
pub fn simplex_method(
    constr: na::DMatrix<f32>,
    req: Vec<f32>,
    obj: Vec<f32>,
    with_print: bool,
) -> na::DMatrix<f32> {
    let mut table = create_augmented_mat(obj, constr, req);
    if with_print {
        println!("Init table {}", &table);
    }
    while let Some(pivot) = get_next_pivot(&table) {
        table = apply_row_operations(pivot, table);
        if with_print {
            println!("Pivot {:?}", pivot);
            println!("Table {}", &table);
        }
    }
    table
}

/// The iterations of the simplex algorithm involve exchanging basic variables
/// with non-basic variables by using matrix row operations.
fn apply_row_operations(pivot: (usize, usize), mut table: na::DMatrix<f32>) -> na::DMatrix<f32> {
    for i in 0..table.nrows() {
        if pivot.1 == i {
            continue;
        }
        // The row the entering variable of which we wanna set to 0
        let target_entry = table[(i, pivot.0)];
        for j in 0..table.ncols() {
            table[(i, j)] += (table[(pivot.1, j)] / table[(pivot.1, pivot.0)]) * -target_entry;
        }
    }
    table
}

/// If all coefficients of non-basic variables in row(0) are positive
/// then you have the optimal solution. Otherwise, select a non-basic
/// variable that has a negative coefficient in row(0) to be
/// the next entering variable, then pivot again.
fn get_next_pivot(table: &na::DMatrix<f32>) -> Option<(usize, usize)> {
    // Choosing the entering variable
    // 0 - the index of the entering variable; 1 - its value
    let mut entry: Option<(usize, f32)> = None;
    for j in 1..table.ncols() {
        if table[(0, j)] >= 0.0 {
            continue;
        }
        if let Some(e) = entry {
            if table[(0, j)] > e.1 {
                continue;
            }
        }
        entry = Some((j, table[(0, j)]));
    }
    // There is no negative non-basic variables left - stop iterations
    if entry == None {
        return None;
    }
    // 0 - the index of the row; 1 - the ratio value
    let mut pivot: Option<(usize, f32)> = None;
    // Choosing the pivot row
    let last_coll = table.ncols() - 1;
    let entry_index = entry.unwrap().0;
    for i in 1..table.nrows() {
        let ratio = table[(i, last_coll)] / table[(i, entry_index)];
        if ratio < 0.0 {
            continue;
        }
        if let Some(p) = pivot {
            if p.1 <= ratio {
                continue;
            }
        }
        pivot = Some((i, ratio));
    }
    // Returns the entry point index and pivot row index
    if let Some(p) = pivot {
        return Some((entry_index, p.0));
    }
    None
}

/// Create an augmented matrix from the given constraints and objective function
fn create_augmented_mat(
    obj: Vec<f32>,
    constr: na::DMatrix<f32>,
    req: Vec<f32>,
) -> na::DMatrix<f32> {
    // Count the objective function row as well
    let n_rows = constr.nrows() + 1;
    let n_cols = n_rows + obj.len() + 1;
    let mut table = na::DMatrix::<f32>::zeros(n_rows, n_cols);

    // Setting up objective function row
    table[(0, 0)] = 1.0;
    for i in 0..obj.len() {
        table[(0, i + 1)] = -obj[i];
    }

    // Setting up constraints & requirements
    for i in 1..n_rows {
        table[(i, constr.ncols() + i)] = 1.0;
        for j in 1..n_cols {
            if j == n_cols - 1 {
                table[(i, j)] = req[i - 1];
            } else if j <= constr.ncols() {
                table[(i, j)] = constr[(i - 1, j - 1)];
            }
        }
    }

    table
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simplex_test_problem1() {
        /*
        Objective function = 20_000x + 45_000y + 85_000z
        Constraints:
        10x + 15y + 10z <= 720
        13x + 5y + 5z <= 680
        20x + 5y + 10z <= 550
        z <= 7
        x, y, z >= 0
        */
        let obj_f = vec![20_000.0, 45_000.0, 85_000.0];
        #[rustfmt::skip]
        let constraints = na::DMatrix::from_row_slice(4, 3, &[
            10.0, 15.0, 10.0, 
            13.0, 5.0, 5.0,
            20.0, 5.0, 10.0,
            0.0, 0.0, 1.0,
        ]);
        let req = vec![720.0, 680.0, 550.0, 7.0];
        let res_table = simplex_method(constraints, req, obj_f, true);
        assert!(
            (res_table[(0, 8)] - 254_5000.0).abs() < std::f32::EPSILON,
            "The expected optimal value: 254_5000, the value we got: {}",
            res_table[(0, 8)]
        );
    }
    #[test]
    fn simplex_test_problem2() {
        /*
        Objective function = 7x + 5y
        Constraints:
        2x + 3y <= 90
        3x + 2y <= 120
        x, y >= 0
        */
        let obj_f = vec![7.0, 5.0];
        #[rustfmt::skip]
        let constraints = na::DMatrix::from_row_slice(2, 2, &[
            2.0, 3.0,
            3.0, 2.0,
        ]);
        let req = vec![90.0, 120.0];

        let res_table = simplex_method(constraints, req, obj_f, true);
        assert!(
            (res_table[(0, 5)] - 282.0).abs() < std::f32::EPSILON,
            "The expected optimal value: 282, the value we got: {}",
            res_table[(0, 5)]
        );
    }
}
