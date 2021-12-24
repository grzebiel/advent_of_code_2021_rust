use std::fs::{File};
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use std::num::ParseIntError;
use std::cmp::{max, min};

#[derive(Debug)]
struct Line {
    s: (i32, i32),
    e: (i32, i32)
}

impl FromStr for Line
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ps = s.split(" -> ")
            .map(|x| 
                x
                    .split(',')
                    .map(|y| y.parse::<i32>()
                        .unwrap()))
            .flatten()
            .collect::<Vec<_>>();
        Ok(Line{
            s: (ps[0],ps[1]),
            e: (ps[2], ps[3])
        })
    }
}

fn print_map(map: &Vec<Vec<i32>>)
{
    for l in map{
        println!("{:?}", l);
    }
}

pub fn solve1() -> i32
{
    let file_reader = BufReader::new(File::open("input_5.txt").unwrap());
    let lines = file_reader.lines().map(|line| line.unwrap().parse::<Line>().unwrap()).collect::<Vec<_>>();
    build_map(lines).iter().flatten().filter(|x| **x> 1).count() as i32
}

fn build_map(lines: Vec<Line>) -> Vec<Vec<i32>> {
    // let hv_lines = lines.iter().filter(|line| (line.s.0 == line.e.0) || (line.s.1 == line.e.1)); // part 1
    let size = 1000;
    let max_x = size;
    let max_y = size;
    let mut map : Vec<Vec<i32>> = vec![vec![0; max_x]; max_y];

    // for l in (hv_lines) //part 1
    for l in lines
    {
        if (l.s.1) == (l.e.1) { //horizontal
            let start = min(l.s.0, l.e.0);
            let end = max(l.s.0, l.e.0);
            for i in (start)..(end +1) {
                map[l.s.1 as usize][i as usize] += 1;
            }

        } else if (l.s.0) == (l.e.0){ // vertical
            let start = min(l.s.1, l.e.1);
            let end = max(l.s.1, l.e.1);
            for i in (start)..(end+1) {
                map[i as usize][l.s.0 as usize] += 1;
            }
        } else { // diagonal
            let (s, e) = if l.s.0 > l.e.0 {
                (l.e, l.s)
            } else
            {
                (l.s, l.e)
            };
            if s.1 > e.1 { // going down
                for i in 0..(s.1 - e.1 +1) {
                    map[(s.1 - i) as usize][(s.0 + i) as usize] += 1;
                }
            } else { // going up
                for i in 0..(e.1 - s.1 +1) {
                    map[(s.1 + i) as usize][(s.0 + i) as usize] += 1;
                }
            }
        }
    }
    print_map(&map);
    map
}
