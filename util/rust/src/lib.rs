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

/// Parses the input file line-by-line using the given parsing function.
pub fn parse_input<T>(input_path: String, parser: impl Fn(&str) -> T) -> Result<Vec<T>, String> {
    parse_input_on_char("\n", input_path, parser)
}

/// Parses the input file line-by-line using the given parsing function.
pub fn parse_input_on_char<T>(split_by: &str, input_path: String, parser: impl Fn(&str) -> T) -> Result<Vec<T>, String> {
    let text = fs::read_to_string(input_path);

    match text {
        Ok(text) => {
            let result = text.split(split_by).map(parser).collect();

            Ok(result)
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}
