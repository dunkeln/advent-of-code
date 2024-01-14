use std::fs::read_to_string;
use nom::{character::complete::newline, sequence::tuple};

/// 3 fields: red, green, blue
struct BallsConfig(i32, i32, i32);


fn parser(input: &str) -> IResult<&str, (u32, Vec<BallsConfig>)> {
    let (prefix, (_, num, _)) = tuple((tage("Game"), u32, tag(": ")))(input).unwrap();
    let (prefix, configs) = 
}

fn main() {
    // let given_cfg = BallsConfig(12, 13, 14);
    let data = read_to_string("./data/02.txt").expect("check file path");
    // let data = data.lines().map(|line| {
    //     todo!()
    // });
    let sample = data.lines().take(1).collect::<Vec<_>>();
}