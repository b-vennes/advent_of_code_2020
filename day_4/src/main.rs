use regex::Regex;

#[derive(Debug)]
enum Measurement {
    CM(i32),
    IN(i32),
}

impl Measurement {
    pub fn parse(value: String) -> Option<Measurement> {
        if value.ends_with("cm") {
            Some(Measurement::CM(value.replace("cm", "").parse::<i32>().unwrap()))
        } else if value.ends_with("in") {
            Some(Measurement::IN(value.replace("in", "").parse::<i32>().unwrap()))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Passport {
    pub birth_yr: Option<i32>,
    pub issue_yr: Option<i32>,
    pub expiration_yr: Option<i32>,
    pub height: Option<Measurement>,
    pub hair_clr: Option<String>,
    pub eye_clr: Option<String>,
    pub id: Option<String>,
    pub country_id: Option<String>,
}

impl Passport {

    pub fn is_valid(&self) -> bool {
        self.birth_yr_is_valid() &&
        self.issue_yr_is_valid() &&
        self.expiration_yr_is_valid() &&
        self.height_is_valid() &&
        self.hair_clr_is_valid() &&
        self.eye_clr_is_valid() &&
        self.id_is_valid() &&
        self.country_id_is_valid()
    }

    pub fn birth_yr_is_valid(&self) -> bool {
        match self.birth_yr {
            Some(year) => year >= 1920 && year <= 2002,
            _ => false
        }
    }

    pub fn issue_yr_is_valid(&self) -> bool {
        match self.issue_yr {
            Some(year) => year >= 2010 && year <= 2020,
            _ => false
        }
    }

    pub fn expiration_yr_is_valid(&self) -> bool {
        match self.expiration_yr {
            Some(year) => year >= 2020 && year <= 2030,
            _ => false
        }
    }

    pub fn height_is_valid(&self) -> bool {
        match self.height {
            Some(Measurement::IN(value)) => value >= 59 && value <= 76,
            Some(Measurement::CM(value)) => value >= 150 && value <= 193,
            _ => false 
        }
    }

    pub fn hair_clr_is_valid(&self) -> bool {
        match &self.hair_clr {
            Some(color) => {
                let must_match = Regex::new(r"^#([0-9]|[a-f]){6}").unwrap();

                must_match.is_match(&color)
            },
            _ => false
        }
    }

    pub fn eye_clr_is_valid(&self) -> bool {
        match &self.eye_clr {
            Some(color) if color == "amb" => true,
            Some(color) if color == "blu" => true,
            Some(color) if color == "brn" => true,
            Some(color) if color == "gry" => true,
            Some(color) if color == "grn" => true,
            Some(color) if color == "hzl" => true,
            Some(color) if color == "oth" => true,
            _ => false
        }
    }

    pub fn id_is_valid(&self) -> bool {
        match &self.id {
            Some(value) => value.len() == 9 && value.parse::<i32>().is_ok(),
            _ => false
        }
    }

    pub fn country_id_is_valid(&self) -> bool {
        true
    }

    pub fn empty() -> Passport {
        Passport {
            birth_yr: None,
            issue_yr: None,
            expiration_yr: None,
            height: None,
            hair_clr: None,
            eye_clr: None,
            id: None,
            country_id: None,
        }
    }
}

fn parse_into_passport(value: &str) -> Passport {
    value.replace("\r\n", " ").split(" ").fold(Passport::empty(), |state, pair| {
        let key_value_vec: Vec<&str> = pair.split(':').collect();
        let key = key_value_vec[0];
        let value = key_value_vec[1].to_string();

        match key {
            "byr" => Passport {
                birth_yr: value.parse::<i32>().ok(),
                ..state
            },
            "iyr" => Passport {
                issue_yr: value.parse::<i32>().ok(),
                ..state
            },
            "eyr" => Passport {
                expiration_yr: value.parse::<i32>().ok(),
                ..state
            },
            "hgt" => Passport {
                height: Measurement::parse(value),
                ..state
            },
            "hcl" => Passport {
                hair_clr: Some(value),
                ..state
            },
            "ecl" => Passport {
                eye_clr: Some(value),
                ..state
            },
            "pid" => Passport {
                id: Some(value),
                ..state
            },
            "cid" => Passport {
                country_id: Some(value),
                ..state
            },
            _ => state,
        }
    })
}

fn main() {
    let input_path = util::get_input_path_from_args(std::env::args()).unwrap();

    let passports = util::parse_input_on_char("\r\n\r\n", input_path, parse_into_passport).unwrap();

    let first = &passports[0];

    println!("{:?}", first);

    println!("{}", first.is_valid());

    println!("{}", first.birth_yr_is_valid());

    println!("{}", first.issue_yr_is_valid());

    let valids = passports
        .iter()
        .filter(|p| p.is_valid())
        .collect::<Vec<&Passport>>().len();

    println!("{}", valids);
}
