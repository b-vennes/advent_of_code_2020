use std::env;

/// Returns the n i64 numbers that sum to the given i64 value from the borrowed vector of i64 numbers.
/// For example, the vector [1, 2, 3] with n = 2 and value 3 would return [1, 2].  
fn n_that_sum_to_value_from_vec(n: i32, value: i64, vec: &[i64]) -> Vec<i64> {
    if n == 2 {
        vec.iter()
            .filter(|item| vec.contains(&(value - *item)))
            .copied()
            .collect()
    } else {
        vec.iter()
            .flat_map(|item| n_that_sum_to_value_from_vec(n - 1, value - item, vec))
            .collect()
    }
}

fn main() {
    let input_path = util::get_input_path_from_args(env::args()).unwrap();

    let input = util::read_input_to_vec_i64(input_path).unwrap();

    let answers = n_that_sum_to_value_from_vec(3, 2020, &input);

    println!("{:?}", answers);
}
