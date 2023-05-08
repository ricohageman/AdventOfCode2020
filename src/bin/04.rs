use itertools::Itertools;
use std::num::ParseIntError;

#[derive(Debug)]
enum PassportField {
    BirthYear(String),
    IssueYear(String),
    ExpirationYear(String),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportID(String),
    CountryID(String),
}

fn parse_input(input: &str) -> Vec<Vec<PassportField>> {
    input
        .split("\n\n")
        .map(|passport_input| {
            passport_input
                .split("\n")
                .flat_map(|line| line.split(" "))
                .filter_map(|field| {
                    let mut data = field.split(":");
                    let field_id = data.next().unwrap();
                    let entry = data.next().unwrap().to_string();

                    match field_id {
                        "byr" => Some(PassportField::BirthYear(entry)),
                        "iyr" => Some(PassportField::IssueYear(entry)),
                        "eyr" => Some(PassportField::ExpirationYear(entry)),
                        "hgt" => Some(PassportField::Height(entry)),
                        "hcl" => Some(PassportField::HairColor(entry)),
                        "ecl" => Some(PassportField::EyeColor(entry)),
                        "pid" => Some(PassportField::PassportID(entry)),
                        "cid" => None,
                        &_ => panic!("{:?}", field_id),
                    }
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .into_iter()
            .filter(|fields| fields.len() == 7)
            .count() as u32,
    )
}

fn is_valid_passport(fields: &[PassportField]) -> bool {
    if fields.len() != 7 {
        return false;
    }

    fields.iter().all(|field| match field {
        PassportField::BirthYear(year) => match year.parse::<usize>() {
            Err(_) => false,
            Ok(year) => (1920..=2002).contains(&year),
        },
        PassportField::IssueYear(year) => match year.parse::<usize>() {
            Err(_) => false,
            Ok(year) => (2010..=2020).contains(&year),
        },
        PassportField::ExpirationYear(year) => match year.parse::<usize>() {
            Err(_) => false,
            Ok(year) => (2020..=2030).contains(&year),
        },
        PassportField::Height(height) => {
            let unit = height.chars().rev().take(2).collect::<Vec<_>>();
            let number = height[..height.len() - 2].parse::<usize>();

            match ((unit[0], unit[1]), number) {
                (_, Err(_)) => false,
                (('m', 'c'), Ok(height)) => (150..=193).contains(&height),
                (('n', 'i'), Ok(height)) => (59..=76).contains(&height),
                _ => false,
            }
        }
        PassportField::HairColor(color) => {
            color
                .chars()
                .enumerate()
                .all(|(index, char)| match (index, char) {
                    (0, '#') => true,
                    (0, _) => false,
                    (_, '0'..='9') => true,
                    (_, 'a'..='f') => true,
                    _ => false,
                })
        }
        PassportField::EyeColor(color) => {
            vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&**color)
        }
        PassportField::PassportID(id) => id.len() == 9 && id.parse::<usize>().is_ok(),
        PassportField::CountryID(_) => true,
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .into_iter()
            .filter(|fields| is_valid_passport(fields))
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_invalid_passports() {
        let passports = vec![
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
            "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
        ];

        for passport in passports {
            let fields = parse_input(passport);
            assert!(!is_valid_passport(&fields[0]))
        }
    }

    #[test]
    fn test_valid_passports() {
        let passports = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ];

        for passport in passports {
            let fields = parse_input(passport);
            assert!(is_valid_passport(&fields[0]))
        }
    }
}
