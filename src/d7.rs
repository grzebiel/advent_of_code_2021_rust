use std::fs::{File};
use std::io::{BufReader, BufRead};

fn cost(start: i64, target: i64) -> i64
{
    let n = (target-start).abs()+1;
    n * (n+1) /2
}

fn total_cost(pos: i64, vals: &Vec<i64>) -> i64
{
    vals.iter().map(|v| cost(*v, pos)).sum()
}

fn solve_recurs(min: i64, max: i64, vals: &Vec<i64>, recurse_counter : i64) -> i64
{
    if recurse_counter > 60 {
        panic!("recursion never ending")
    }
    dbg!(min, max);
    let space_size = max - min;
    let mid = min + space_size/2;
    dbg!(mid);

    let mid_cost = total_cost(mid, vals);
    let next_cost = total_cost(mid + 1, vals);
    if space_size <= 2 {
        return mid_cost.min(next_cost)
    }
    dbg!(mid_cost, next_cost);
    if mid_cost < next_cost {
        dbg!(solve_recurs(min, mid-1, vals, recurse_counter + 1)).min(mid_cost)
    } else {
        solve_recurs(mid+2, max, vals, recurse_counter + 1).min(next_cost)
    }
}

pub fn solve() -> i64
{
    let file_reader = BufReader::new(File::open("input_7.txt").unwrap());
    let vals = file_reader.lines().map(|line| line.unwrap())
        .map(|x|
            x.split(',').map(|v| v.parse::<i64>().unwrap()).collect::<Vec<_>>()
        ).flatten().collect::<Vec<_>>();
    let min = *vals.iter().min().unwrap();
    let max = *vals.iter().max().unwrap();

    solve_recurs(min, max, &vals, 0)
}
