use std::env;

#[derive(Debug)]
struct PasswordEntry {
    pub num_req_bottom: i32,
    pub num_req_top: i32,
    pub letter_req: char,
    pub password: String,
}

fn parse_into_password(value: &str) -> PasswordEntry {
    let sections: Vec<&str> = value.split(' ').collect();

    let num_requirements: Vec<&str> = sections[0].split('-').collect();

    let num_req_bottom = num_requirements[0].parse::<i32>().unwrap();
    let num_req_top = num_requirements[1].parse::<i32>().unwrap();

    let letter_req: char = (sections[1].chars().collect::<Vec<char>>())[0];

    let password = sections[2].to_string();

    PasswordEntry {
        num_req_bottom,
        num_req_top,
        letter_req,
        password,
    }
}

fn valid_passwords(
    passwords: &[PasswordEntry],
    validator: impl Fn(&&PasswordEntry) -> bool,
) -> Vec<&PasswordEntry> {
    passwords
        .iter()
        .filter(|p: &&PasswordEntry| validator(p))
        .collect::<Vec<&PasswordEntry>>()
}

fn validate_password_1(p: &&PasswordEntry) -> bool {
    let p_chars: Vec<char> = (*p).password.chars().collect::<Vec<char>>();

    let count = p_chars
        .iter()
        .fold(0, |sum, c| if *c == p.letter_req { sum + 1 } else { sum });

    count <= p.num_req_top && count >= p.num_req_bottom
}

fn validate_password_2(p: &&PasswordEntry) -> bool {
    let p_chars: Vec<char> = (*p).password.chars().collect::<Vec<char>>();

    let bottom_letter = p_chars[(p.num_req_bottom - 1) as usize];

    let top_letter = p_chars[(p.num_req_top - 1) as usize];

    (bottom_letter == p.letter_req && top_letter != p.letter_req)
        || (bottom_letter != p.letter_req && top_letter == p.letter_req)
}

fn main() {
    let input_path = util::get_input_path_from_args(env::args()).unwrap();

    let entries = util::parse_input(input_path, parse_into_password).unwrap();

    let part_1_valids = valid_passwords(&entries, validate_password_1);
    let part_2_valids = valid_passwords(&entries, validate_password_2);

    println!("Num valid passwords for part 1: {}", part_1_valids.len());
    println!("Num valid passwords for part 2: {}", part_2_valids.len());
}
