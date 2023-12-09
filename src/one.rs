use std::sync::OnceLock;

use anyhow::bail;
use regex::Regex;

pub struct PartOne<'a>(&'a str);
pub struct PartTwo<'a>(&'a str);

pub trait Constructor<'a> {
    fn new(s: &'a str) -> Self;
}

impl<'a> Constructor<'a> for PartOne<'a> {
    fn new(s: &'a str) -> Self {
        Self(s)
    }
}

impl<'a> Constructor<'a> for PartTwo<'a> {
    fn new(s: &'a str) -> Self {
        Self(s)
    }
}

struct MyResult(u32);

impl<'a> From<PartOne<'a>> for MyResult {
    fn from(value: PartOne<'a>) -> Self {
        let s = value.0;
        let front = s
            .chars()
            .find(|&c| !c.is_alphabetic())
            .and_then(|c| c.to_digit(10));

        let rear = s
            .chars()
            .rev()
            .find(|&c| !c.is_alphabetic())
            .and_then(|c| c.to_digit(10));

        match (front, rear) {
            (Some(f), Some(r)) => MyResult((f * 10) + r),
            (_, _) => unreachable!("Expect valid problem input: {s}"),
        }
    }
}

fn num_parser(s: &str) -> u32 {
    match s {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        s => s
            .parse::<u32>()
            .or_else(|_| bail!("Failed to parse {s}, should be unreachable"))
            .unwrap(),
    }
}

fn number_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    // Capture those values that are string numerals or digits.
    static RE_STR: &str = r"(one|two|three|four|five|six|seven|eight|nine|\d)";
    RE.get_or_init(|| Regex::new(RE_STR).unwrap())
}

impl<'a> From<PartTwo<'a>> for MyResult {
    fn from(value: PartTwo<'a>) -> Self {
        let mut s = value.0;
        let my_re = number_regex();

        let mut caps = vec![];
        let mut ix = 0;
        loop {
            s = &s[ix..s.len()];

            ix = if let Some(mtch) = my_re.find(s) {
                let (new_ix, num) = (mtch.start() + 1, num_parser(mtch.as_str()));
                caps.push(num);
                new_ix
            } else {
                break;
            }
        }

        let (front, rear) = (caps[0], caps.last().expect("Should be one result"));
        Self((front * 10) + rear)
    }
}

pub fn collector<'a, I, T>(input: I) -> Vec<u32>
where
    I: Iterator<Item = &'a str>,
    T: Constructor<'a>,
    MyResult: From<T>,
{
    input
        .map(|s| {
            let t = T::new(s);
            let r = MyResult::from(t);
            r.0
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::vec::IntoIter;

    use super::*;

    #[test]
    fn test_part_1() {
        let s = "1abc2";
        let t = "pqr3stu8vwx";

        let rs = MyResult::from(PartOne(s));
        let rt = MyResult::from(PartOne(t));
        assert_eq!(rs.0, 12);
        assert_eq!(rt.0, 38);
    }

    #[test]
    fn test_num_parser() {
        let a = num_parser("1");
        let b = num_parser("nine");

        assert_eq!(1, a);
        assert_eq!(9, b);
    }

    #[test]
    fn test_regex() {
        let s = "abcone2threexyz";
        let mut caps = vec![];
        for (t, [c]) in number_regex().captures_iter(s).map(|c| c.extract()) {
            caps.push(c);
        }
        assert_eq!("one", caps[0]);
        assert_eq!("2", caps[1]);
        assert_eq!("three", caps[2]);
    }

    #[test]
    fn test_part_2() {
        let v = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let expecteds = [29, 83, 13, 24, 42, 14, 76];

        let results = collector::<IntoIter<&str>, PartTwo>(v.into_iter());

        for (ix, actual) in results.iter().enumerate() {
            let expected = expecteds[ix];
            assert_eq!(expected, *actual)
        }

        assert_eq!(281, results.iter().sum::<u32>());

        let v = vec![
            "12",
            "nkgbpqfvvb9threesixqgqvjgzdxm1rc",
            "five7seven",
            "2seven8151",
            "86eightfivethreebgbfvr4",
            "355",
            "hgldv8four2dzmtpdsmck4five",
        ];

        let results = collector::<IntoIter<&str>, PartTwo>(v.into_iter());
        let expecteds = [12, 91, 57, 21, 84, 35, 85];

        for (ix, actual) in results.iter().enumerate() {
            let expected = expecteds[ix];
            assert_eq!(expected, *actual)
        }

        assert_eq!(385, results.iter().sum::<u32>());
    }

    #[test]
    fn test_oneight() {
        let v = vec!["zoneight"];
        let expecteds = [18];

        let results = collector::<IntoIter<&str>, PartTwo>(v.into_iter());

        for (ix, actual) in results.iter().enumerate() {
            let expected = expecteds[ix];
            assert_eq!(expected, *actual)
        }
    }
}
