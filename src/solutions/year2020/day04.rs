// day4 parse              time:   [753.47 us 758.95 us 765.43 us]
// day4 part 1             time:   [27.518 us 27.814 us 28.163 us]
// day4 part 2             time:   [40.731 us 41.412 us 42.272 us]

use super::*;
use std::collections::BTreeMap;

static KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
static ECL_VALS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub fn parse(s: &String) -> Vec<BTreeMap<&str, &str>> {
    let mut passports = Vec::new();
    for lines in s.split("\n\n") {
        let mut passport = BTreeMap::new();
        for line in lines.lines() {
            for field in line.split(" ") {
                let key_value: Vec<&str> = field.split(":").collect();
                passport.insert(key_value[0], key_value[1]);
            }
        }
        passports.push(passport);
    }
    passports
}

pub fn part1(input: &Vec<BTreeMap<&str, &str>>) -> usize {
    let mut valid = 0;
    for passport in input {
        if KEYS.iter().any(|key| !passport.contains_key(key)) {
            continue;
        }
        valid += 1;
    }
    valid
}

pub fn part2(input: &Vec<BTreeMap<&str, &str>>) -> usize {
    let mut valid = 0;
    for passport in input {
        if passport_valid(passport) {
            valid += 1;
        }
    }
    valid
}

fn passport_valid(passport: &BTreeMap<&str, &str>) -> bool {
    for key in KEYS.iter() {
        if let Some(value) = passport.get(key) {
            if !validate(key, *value) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn validate(key: &str, value: &str) -> bool {
    match key {
        "byr" => (1920..=2002).contains(&value.parse::<u32>().unwrap()),
        "iyr" => (2010..=2020).contains(&value.parse::<u32>().unwrap()),
        "eyr" => (2020..=2030).contains(&value.parse::<u32>().unwrap()),
        "hgt" => {
            value.ends_with("cm")
                && (150..=193).contains(&value[..value.len() - 2].parse::<usize>().unwrap())
                || value.ends_with("in")
                    && (59..=76).contains(&value[..value.len() - 2].parse::<usize>().unwrap())
        }
        "hcl" => {
            value.len() == 7
                && value.starts_with("#")
                && value[1..].chars().all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ECL_VALS.contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_numeric()),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day04_test() {
        let test_input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            .to_owned();
        assert_eq!(part1(&parse(&test_input)), 2);
        let invalid_input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
            .to_owned();
        assert_eq!(part2(&parse(&invalid_input)), 0);
        let valid_input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            .to_owned();
        assert_eq!(part2(&parse(&valid_input)), 4);
    }

    #[test]
    fn day04() {
        let input = get_input(2020, 4).unwrap();
        assert_eq!(part1(&parse(&input)), 239);
        assert_eq!(part2(&parse(&input)), 188);
    }
}
