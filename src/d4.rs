use std::fs::{File};
use std::io::{BufReader, BufRead};
use itertools::Itertools;
use std::collections::HashMap;
use std::cmp::{max, min};

fn parse_input() -> (Vec<i32>, Vec<Vec<Vec<i32>>>)
{
    let file_reader = BufReader::new(File::open("input_4.txt").unwrap());
    let mut iter = file_reader.lines().map(|line| line.unwrap());
    let calls = iter.next().unwrap().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    iter.next();
    // dbg!(calls);
    let boards = iter.chunks(6).into_iter()
        .map(|chunk| {
            chunk.take(5)
                .map(|line| {
                    line
                        .split_whitespace()
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    (calls, boards)
}

fn into_strings(board: &Vec<Vec<i32>>) -> Vec<Vec<i32>>
{
    (0..5)
        .map(|i|
            board.into_iter()
                .map(|line|
                    line[i]
                )
            .collect::<Vec<_>>()
        ).chain(board.iter().cloned()).collect()
}

pub fn solve1() -> i32
{
    let (calls, boards) = parse_input();
    let calls_map = calls.clone().into_iter().enumerate().map(|(i, v)| (v, i)).collect::<HashMap<_,_>>();
    // dbg!(calls_map);

    let strings = boards.iter().map(into_strings).collect::<Vec<_>>();
    let mut points = Vec::new();
    points.resize(strings.len(), i32::MAX);

    for (i, possibilities) in strings.iter().enumerate()
    {
        for string in possibilities
        {
            let mut max_hit = i32::MIN;
            for v in string.iter()
            {
                match calls_map.get(v) {
                    Some(pos) => max_hit = max(max_hit, *pos as i32),
                    None => break,
                }
            }
            points[i] = min(max_hit, points[i]);
        }
    }
    let min = points.iter().min().unwrap();
    let min_index = points.iter().position(|r| r == min).unwrap();

    count_points(&boards[min_index], &calls[0.. (min+1) as usize])
}

fn count_points(board: &Vec<Vec<i32>>, sequence: &[i32]) -> i32 {
    board.iter().flatten().filter(|x| ! sequence.contains(x)).sum::<i32>() * sequence.last().unwrap()
}

pub fn solve2() -> i32
{
    0
}
