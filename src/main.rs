use std::io;
use std::io::Read;

use regex::Regex;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
}

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    println!("{:?}", parse_passports(&input));

    Ok(())
}

fn parse_passports(input: &str) -> usize {
    input
        .split("\n\n")
        .into_iter()
        .filter(|passport_record| validate(passport_record))
        .count()
}

fn validate(input: &str) -> bool {
    let hair_color_pattern = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let passport_id_pattern = Regex::new(r"^[0-9]{9}$").unwrap();
    let fields = &vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

    input
        .replace("\n", " ")
        .split(' ')
        .into_iter()
        .filter(|field_and_value| {
            let mut iterator = field_and_value.splitn(2, ':');
            match (iterator.next(), iterator.next()) {
                (Some("byr"), Some(year)) => year
                    .parse()
                    .map(|x: i64| (1920..=2002).contains(&x))
                    .unwrap_or(false),
                (Some("iyr"), Some(year)) => year
                    .parse()
                    .map(|x: i64| (2010..=2020).contains(&x))
                    .unwrap_or(false),
                (Some("eyr"), Some(year)) => year
                    .parse()
                    .map(|x: i64| (2020..=2030).contains(&x))
                    .unwrap_or(false),
                (Some("hcl"), Some(hair_color)) => hair_color_pattern.is_match(hair_color),
                (Some("pid"), Some(passport)) => passport_id_pattern.is_match(passport),
                (Some("ecl"), Some(eye_color)) => {
                    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&eye_color)
                }
                (Some("hgt"), Some(height)) => valid_height(height),
                (Some("cid"), _) => false,
                _ => false,
            }
        })
        .count()
        == fields.len()
}

fn valid_height(height: &str) -> bool {
    match height.split_at(height.len() - 2) {
        (number, "in") => number
            .parse()
            .map(|x: i64| (59..=76).contains(&x))
            .unwrap_or(false),
        (number, "cm") => number
            .parse()
            .map(|x: i64| (150..=193).contains(&x))
            .unwrap_or(false),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_passports, valid_height, validate};

    #[test]
    fn no_passports_is_empty() {
        assert_eq!(0, parse_passports(""));
    }

    #[test]
    fn example_is_2() {
        assert_eq!(
            2,
            parse_passports(
                r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#
            )
        );
    }
    #[test]
    fn example_is_4() {
        assert_eq!(
            4,
            parse_passports(
                r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#
            )
        );
    }

    #[test]
    fn valid_example() {
        assert_eq!(
            true,
            validate(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm"
            )
        )
    }

    #[test]
    fn missing_height() {
        assert_eq!(
            false,
            validate("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929")
        )
    }

    #[test]
    fn north_poll_id() {
        assert_eq!(
            true,
            validate("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm")
        )
    }

    #[test]
    fn missing_cid_and_byr() {
        assert_eq!(
            false,
            validate("hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in")
        )
    }

    #[test]
    fn assorted_invalid_1() {
        assert_eq!(
            false,
            validate("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926")
        );
    }
    #[test]
    fn assorted_invalid_2() {
        assert_eq!(
            false,
            validate("iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946")
        );
    }
    #[test]
    fn assorted_invalid_3() {
        assert_eq!(
            false,
            validate(
                "hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
            )
        );
    }

    #[test]
    fn assorted_invalid_4() {
        assert_eq!(
            false,
            validate("hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007")
        )
    }

    #[test]
    fn assorted_valid_1() {
        assert_eq!(
            true,
            validate("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f")
        );
    }

    #[test]
    fn assorted_valid_2() {
        assert_eq!(
            true,
            validate(
                "eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
            )
        );
    }

    #[test]
    fn assorted_valid_3() {
        assert_eq!(
            true,
            validate(
                "hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022"
            )
        );
    }

    #[test]
    fn assorted_valid_4() {
        assert_eq!(
            true,
            validate("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719")
        )
    }

    #[test]
    fn test_valid_height() {
        assert_eq!(true, valid_height("193cm"));
        assert_eq!(false, valid_height("194cm"));
        assert_eq!(true, valid_height("150cm"));
        assert_eq!(false, valid_height("149cm"));
        assert_eq!(true, valid_height("59in"));
        assert_eq!(false, valid_height("58in"));
        assert_eq!(true, valid_height("76in"));
        assert_eq!(false, valid_height("77in"))
    }
}
