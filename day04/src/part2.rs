use std::fs::read_to_string;
use regex::Regex;

// In actual real world code I would use `?` with the anyhow crate but for AOC `unwrap` is fine

fn main() {
    let passports = read_to_string("input.txt").unwrap();
    let splitted: Vec<_> = passports.split("\n\n").collect();
    let mut valid = 0;

    'outer: for passport in splitted {
        let a = passport.find("byr").is_some();
        let b = passport.find("iyr").is_some();
        let c = passport.find("eyr").is_some();
        let d = passport.find("hgt").is_some();
        let e = passport.find("hcl").is_some();
        let f = passport.find("ecl").is_some();
        let g = passport.find("pid").is_some();

        if !a || !b || !c || !d || !e || !f || !g {
            continue;
        }

        let splits: Vec<_> = passport.split(|c| c == ' ' || c == '\n').collect();

        for split in splits {
            let key_value_splitted: Vec<_> = split.split(":").collect();

            let key = key_value_splitted[0];

            if key.is_empty() {
                continue
            }

            let value = key_value_splitted[1];

            match key {
                "byr" => {
                    let birth_year = value.parse::<i32>().unwrap();

                    if birth_year < 1920 || birth_year > 2002 {
                        continue 'outer;
                    }
                },
                "iyr" => {
                    let issue_year = value.parse::<i32>().unwrap();

                    if issue_year < 2010 || issue_year > 2020 {
                        continue 'outer;
                    }
                },
                "eyr" => {
                    let expiration_year = value.parse::<i32>().unwrap();

                    if expiration_year < 2020 || expiration_year > 2030 {
                        continue 'outer;
                    }
                },
                "hgt" => {
                    let is_cm = value.find("cm").is_some();
                    let is_in = value.find("in").is_some();

                    if !is_cm && !is_in {
                        continue 'outer;
                    }

                    let modified = value.replace("cm", "").replace("in", "");

                    let height = modified.parse::<i32>().unwrap();

                    if (is_cm && (height < 150 || height > 193)) || (is_in && (height < 59 || height > 76)) {
                        continue 'outer;
                    }
                },
                "hcl" => {
                    let re = Regex::new(r"^#[0-9a-f]{6}").unwrap();

                    if !re.is_match(value) {
                        continue 'outer;
                    }
                },
                "ecl" => {
                    match value {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
                        _ => continue 'outer
                    }
                },
                "pid" => {
                    if value.len() != 9 || value.parse::<i32>().is_err() {
                        continue 'outer
                    }
                },
                "cid" => {},
                _ => panic!("Found unexpected value: {}", key)
            }
        }

        valid += 1;
    }

    println!("valid: {}", valid);
}
