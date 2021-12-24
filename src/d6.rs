use std::fs::{File};
use std::io::{BufReader, BufRead};

pub fn solve() -> i64
{
    let file_reader = BufReader::new(File::open("input_6.txt").unwrap());
    let vals = file_reader.lines().map(|line| line.unwrap())
        .map(|x|
            x.split(',').map(|v| v.parse::<i64>().unwrap()).collect::<Vec<_>>()
        ).flatten().collect::<Vec<_>>();
    let mut counts = vec![0; 9];
    for v in vals {
        counts[v as usize] += 1;
    }

    for _ in 0..256
    {
        let mut tmp = vec![0; 9];
        for (i, v) in counts.iter().enumerate()
        {
            if i == 0 {
                tmp[8] += v;
                tmp[6] += v;
            }
            else {
                tmp[i-1] += v;
            }
        }
        counts = tmp;
    }
    counts.iter().sum()
}
