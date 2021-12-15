use std::io::{BufReader, BufRead};
use std::fs::{File};

fn count_increases(meas: Vec<i32>) -> usize
{
    let mut i2 = meas.iter();
    i2.next();
    let iter = meas.iter().zip(i2);
    iter.filter(|(x, y)| x<y).count()
}

fn sum_three_window(meas: Vec<i32>) -> Vec<i32>
{
    let mut i3 = meas.iter();
    i3.next();
    i3.next();
    let mut i2 = meas.iter();
    i2.next();
    let iter = meas.iter().zip(i2).zip(i3);
    iter.map(|((x,y),z)| x+y+z).collect()
}

pub fn _1() -> usize
{
    let file_reader = BufReader::new(File::open("input_1.txt").unwrap());
    let numbers: Vec<i32> = file_reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect();


    count_increases(sum_three_window(numbers))
}
