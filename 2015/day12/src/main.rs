use std::io::{self, Read};

use serde_json::{json, Value};

fn sum_json(val: &Value, ignore_red: bool) -> i64 {
    match val {
        Value::Number(x) => x.as_i64().unwrap(),
        Value::Array(vec) => vec.iter().map(|x| sum_json(x, ignore_red)).sum(),
        Value::Object(map) => {
            if ignore_red && map.values().any(|x| *x == json!("red")) { 0 }
            else { map.values().map(|x| sum_json(x, ignore_red)).sum() }
        },
        _ => 0
    }
}

fn part1(input: &str) -> i64 {
    sum_json(&serde_json::from_str(input).unwrap(), false)
}

fn part2(input: &str) -> i64 {
    sum_json(&serde_json::from_str(input).unwrap(), true)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("[1,2,3]"), 6);
        assert_eq!(part1(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(part1("[[[3]]]"), 3);
        assert_eq!(part1(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(part1(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(part1(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(part1("[]"), 0);
        assert_eq!(part1("{}"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("[1,2,3]"), 6);
        assert_eq!(part2(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(part2(r#"[1,"red",5]"#), 6);
    }
}
