use std::fs::read_to_string;

#[inline]
fn retrieve_calibration(line: &str) -> u32 {
    let d1 = line.chars().find_map(|x| x.to_digit(10)).unwrap();
    let d2 = line.chars().rev().find_map(|x| x.to_digit(10)).unwrap();
    d1 * 10 + d2
}

fn process_calibration(line: &str) -> String {
    line.chars().enumerate().map(|(idx, _)|
        if line[idx..].starts_with("one") { "1" }
        else if line[idx..].starts_with("two") { "2" }
        else if line[idx..].starts_with("three") { "3" }
        else if line[idx..].starts_with("four") { "4" }
        else if line[idx..].starts_with("five") { "5" }
        else if line[idx..].starts_with("six") { "6" }
        else if line[idx..].starts_with("seven") { "7" }
        else if line[idx..].starts_with("eight") { "8" }
        else if line[idx..].starts_with("nine") { "9" }
        else if line[idx..].starts_with("zero") { "0" }
        else { &line[idx..idx+1] }
    ).collect::<String>()
}

fn main() {
    let data = read_to_string("./data/01.txt").expect("check file path");
    let data = data.lines().map(|x| x.trim_end());
    let sol1 = data
            .clone()
            .fold(0, |acc, line| acc + retrieve_calibration(line));
    dbg!(sol1);

    let part2 = move |x| {
        let x = process_calibration(x);
        retrieve_calibration(x.as_str())
    };

    let sol2 = data.fold(0, |acc, line| acc + part2(line));
    dbg!(sol2);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(retrieve_calibration("asdfasdjfk56fsdaf"), 56);
        assert_eq!(retrieve_calibration("9fasdfasdfasf0"), 90);
        assert_eq!(retrieve_calibration("as33333333333sdaf"), 33);
    }

    #[test]
    fn test_part2() {
        let part2 = move |x| {
            let x = process_calibration(x);
            retrieve_calibration(x.as_str())
        };
        assert_eq!(part2("aoneasdjfk56fsdaf"), 16);
        assert_eq!(part2("9fasdfasdfasf0"), 90);
        assert_eq!(part2("ninefasdfasdfasfzerofdsafas"), 90);
    }
}