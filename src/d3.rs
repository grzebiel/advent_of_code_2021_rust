use std::fs::{File};
use std::io::{BufReader, BufRead};

pub fn _3() -> i32
{
    let file_reader = BufReader::new(File::open("input_3.txt").unwrap());
    let iter = file_reader.lines().map(|line| line.unwrap());
    let input : Vec<String> = iter.collect();
    // let first  = iter.next();
    let mut ones : Vec<i32> = Vec::with_capacity(30);
    for f in input.iter() {
        for (i, c) in f.chars().enumerate()
        {
            if ones.len() < i+1 {
                ones.push(0);
            }
            ones[i] += (c == '1') as i32;
        }
    }
    // println!("{}, {:?}", count, ones);
    //part 2
    let mut o2 = input.clone();
    for (i, _) in ones.iter().enumerate()
    {
        let count_ones = o2.iter().filter(|x| x.chars().nth(i).unwrap() == '1').count();
        if count_ones >= o2.len() - count_ones {
            o2.retain(|x| x.chars().nth(i).unwrap() == '1');
        }
        else {
            o2.retain(|x| x.chars().nth(i).unwrap() == '0');
        }
        if o2.len() == 1{
            break;
        }
    }
    let mut co2 = input.clone();
    for (i, _) in ones.iter().enumerate()
    {
        let count_ones = co2.iter().filter(|x| x.chars().nth(i).unwrap() == '1').count();
        if count_ones >= co2.len() - count_ones {
            co2.retain(|x| x.chars().nth(i).unwrap() == '0');
        }
        else {
            co2.retain(|x| x.chars().nth(i).unwrap() == '1');
        }
        if co2.len() == 1{
            break;
        }
    }

    o2[0].chars().map(|x| (x == '1') as u32).rev().enumerate().map(|(i,c)| c* 2_u32.pow(i as u32)).fold(0, |acc, x| acc + x as i32) * co2[0].chars().map(|x| (x == '1') as u32).rev().enumerate().map(|(i,c)| c* 2_u32.pow(i as u32)).fold(0, |acc, x| acc + x as i32)


    // part 1
    // let binary : Vec<i32>= ones.iter().rev().map(|c|(c > &(count / 2)) as i32).collect();
    // println!("{:?}", binary);
    // binary.iter().enumerate().map( |(i,c)| c * (2 as i32).pow(i as u32) ).fold(0, |acc, x| acc + x)
    //     * binary.iter().enumerate().map( |(i,c)| (*c == 0) as i32 * (2 as i32).pow(i as u32) ).fold(0, |acc, x| acc + x)
}
