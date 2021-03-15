use crate::read_lines;
use anyhow::Result;

pub fn get_pairs(numbers: &[usize], sum: usize) -> Option<(usize, usize)> {
    for each in numbers {
        if sum > *each {
            let another = sum - each;
            if numbers.binary_search(&another).is_ok() {
                return Some((*each, another));
            }
        }
    }
    None
}

use std::cmp::Ordering;

pub fn get_pairs_fast(numbers: &[usize], sum: usize) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = if numbers.is_empty() {
        0
    } else {
        numbers.len() - 1
    };
    let mut pairs = None;
    while left < right {
        let left_num = numbers[left];
        let right_num = numbers[right];
        let current_sum = left_num + right_num;
        match current_sum.cmp(&sum) {
            Ordering::Equal => {
                pairs = Some((left_num, right_num));
                break;
            }
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
        }
    }
    pairs
}

pub fn get_triplets(numbers: &[usize], sum: usize) -> Option<(usize, usize, usize)> {
    for each in numbers {
        if sum > *each {
            let remainder = sum - each;
            if let Some((a, b)) = get_pairs_fast(numbers, remainder) {
                return Some((*each, a, b));
            }
        }
    }
    None
}

pub fn read_input(filename: &str) -> Result<Vec<usize>> {
    let mut numbers = Vec::new();
    for line in read_lines(filename)? {
        if let Ok(num) = line?.parse::<usize>() {
            numbers.push(num);
        }
    }
    Ok(numbers)
}

const SUM: usize = 2020;

pub fn part1(filename: &str) -> Result<Option<u32>> {
    let mut numbers = read_input(filename)?;
    numbers.sort_unstable();
    let prod2 = if let Some((a, b)) = get_pairs_fast(&numbers[..], SUM) {
        Some(a as u32 * b as u32)
    } else {
        None
    };
    Ok(prod2)
}

pub fn part2(filename: &str) -> Result<Option<u32>> {
    let mut numbers = read_input(filename)?;
    numbers.sort_unstable();
    let prod3 = if let Some((a, b, c)) = get_triplets(&numbers[..], SUM) {
        Some(a as u32 * b as u32 * c as u32)
    } else {
        None
    };
    Ok(prod3)
}
