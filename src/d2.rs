use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::num::ParseIntError;
use std::fs::{File};

enum Command
{
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl FromStr for Command
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split : Vec<&str> = s.split(" ").collect();
        if split.len() != 2
        {
            panic!("can not parse!! {}", s);
        }
        Ok(match split[..] {
            ["up", i] => Command::Up(i.parse::<i32>()?),
            ["down", i] => Command::Down(i.parse::<i32>()?),
            ["forward", i] => Command::Forward(i.parse::<i32>()?),
            _ => panic!("can not parse!"),
        })
    }
}


pub fn _2() -> i32
{
    let file_reader = BufReader::new(File::open("input_2.txt").unwrap());
    let input: Vec<Command> = file_reader.lines().map(|line| line.unwrap().parse::<Command>().unwrap()).collect();

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for i in input {
        match i {
            Command::Up(v) => { aim -= v; }
            Command::Down(v) => { aim += v; }
            Command::Forward(v) => { x += v; y += v *aim }
        }
    }
    return x * y;
}

