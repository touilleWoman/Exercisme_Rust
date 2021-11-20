// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

// solution without Hashmap

// pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
//     let mut mag = vec!["0"; magazine.len()];
//     mag.clone_from_slice(magazine);
//     for word in note {
//         match mag.iter().position(|x| x == word) {
//             None => return false,
//             Some(x) => {mag.remove(x);},
//         }
//     }
//     return true;
// }

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();
    for word in magazine {
        let count_dup = map.entry(word).or_insert(0);
        *count_dup += 1;
    }
    for word in note {
        match map.entry(word).or_insert(0) {
            0 => return false,
            x => {
                *x -= 1;
            }
        }
    }
    return true;
}
