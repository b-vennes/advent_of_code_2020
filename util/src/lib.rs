use std::env::Args;
use std::fs;

/// Gets the input path which is assumed to be the first argument in the args vec.
pub fn get_input_path_from_args(args: Args) -> Result<String, String> {
    let args: Vec<String> = args.collect();

    if args.len() < 2 {
        Err(String::from("No args found."))
    } else {
        Ok(args[1].clone())
    }
}

/// Tries to read each line from the given file into a vec of i64.
pub fn read_input_to_vec_i64(input_path: String) -> Result<Vec<i64>, String> {
    let text = fs::read_to_string(input_path);

    match text {
        Ok(text) => {
            let result = text
                .split('\n')
                .map(|value: &str| value.parse::<i64>())
                .filter(|result| result.is_ok())
                .map(|result| match result {
                    Ok(value) => value,
                    _ => 0,
                })
                .collect();

            Ok(result)
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}
