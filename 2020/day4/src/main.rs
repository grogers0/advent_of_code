use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn parse(puzzle_input: &str) -> Vec<HashMap<String, String>> {
    puzzle_input.split("\n\n").map(|fields| {
        fields.split(char::is_whitespace)
            .filter(|field| !field.is_empty())
            .map(|field| {
                let mut parts = field.split(':');
                let key = parts.next().unwrap().to_string();
                let val = parts.next().unwrap().to_string();
                assert!(parts.next().is_none());
                (key, val)
            }).collect()
    }).collect()
}

fn part1(passports: &Vec<HashMap<String, String>>) -> usize {
    const REQUIRED_FIELDS: &[&str] =
        &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    passports.iter().filter(|passport| {
        REQUIRED_FIELDS.iter().all(|&k| passport.contains_key(k))
    }).count()
}

fn part2(passports: &Vec<HashMap<String, String>>) -> usize {
    let eye_colors: HashSet<&'static str> =
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().map(|&s| s).collect();
    passports.iter().filter(|passport| {
        passport.get("byr").and_then(|v| v.parse::<u16>().ok()).map_or(false, |v| v >= 1920 && v <= 2002) &&
            passport.get("iyr").and_then(|v| v.parse::<u16>().ok()).map_or(false, |v| v >= 2010 && v <= 2020) &&
            passport.get("eyr").and_then(|v| v.parse::<u16>().ok()).map_or(false, |v| v >= 2020 && v <= 2030) &&
            passport.get("hgt").map_or(false, |v| {
                v.strip_suffix("cm").and_then(|v| v.parse::<u8>().ok()).map_or(false, |v| v >= 150 && v <= 193) ||
                    v.strip_suffix("in").and_then(|v| v.parse::<u8>().ok()).map_or(false, |v| v >= 59 && v <= 76)
            }) &&
            passport.get("hcl").map_or(false, |v| {
                v.chars().next() == Some('#') && v.chars().skip(1).count() == 6 &&
                    v.chars().skip(1).all(|ch| ch.is_digit(16))
            }) &&
            passport.get("ecl").map_or(false, |v| eye_colors.contains(v.as_str())) &&
            passport.get("pid").map_or(false, |v| v.chars().count() == 9 && v.chars().all(char::is_numeric))
    }).count()
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();
    let passports = parse(&puzzle_input);

    println!("{}", part1(&passports));
    println!("{}", part2(&passports));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const EX2_INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const EX2_VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(&parse(EX1)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(&parse(EX2_INVALID)));
        assert_eq!(4, part2(&parse(EX2_VALID)));
    }
}
