use itertools::Itertools;
use std::collections::HashMap;

fn translate(dict: &HashMap<char, u8>, word: &Vec<char>) -> u64 {
    let mut nb: u64 = 0;
    for letter in word.iter() {
        nb = nb * 10 + *dict.get(&letter).unwrap() as u64;
    }
    nb
}

fn validation(dict: &HashMap<char, u8>, left: &Vec<&str>, right: &Vec<char>) -> bool {
    let right_nb = translate(&dict, right);
    let mut left_nb = 0;
    for word in left {
        left_nb += translate(&dict, &word.chars().into_iter().collect::<Vec<char>>());
    }
    return right_nb == left_nb;
}

fn leading_letters(left: &Vec<&str>, right: &Vec<char>) -> Vec<char> {
    let mut leadings = vec![];
    leadings.push(right[0]);
    for word in left {
        let char = word.chars().next().unwrap();
        leadings.push(char);
    }
    return leadings;
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right) = input.split_once("==").unwrap();
    let left: Vec<&str> = left.split('+').map(|c| c.trim()).collect();
    let right: Vec<char> = right.trim().chars().collect();

    let all_letters: Vec<char> = input
        .chars()
        .filter(|x| x.is_alphabetic())
        .unique()
        .collect();

    let perms = (0..10).permutations(all_letters.len());
    let leadings = leading_letters(&left, &right);
    for perm in perms {
        let mut dict: HashMap<char, u8> = HashMap::new();
        let group = all_letters.iter().zip(perm.iter());
        for (x, y) in group {
            dict.insert(*x, *y);
        }
        if leadings.iter().any(|x| dict[x] == 0) {
            continue;
        }
        if validation(&dict, &left, &right) {
            return Some(dict);
        }
    }
    return None;
}
