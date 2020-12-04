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

        valid += 1;
    }

    println!("valid: {}", valid);
}
