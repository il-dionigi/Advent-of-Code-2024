use std::collections::HashMap;
use utils;

fn main() {
    let raw_contents: Vec<Vec<u32>> =
        utils::get_inputs(1u8);
    let mut left_contents: Vec<u32> =
        raw_contents.iter().map(|pair| pair[0]).collect();
    let mut right_contents: Vec<u32> =
        raw_contents.iter().map(|pair| pair[1]).collect();
    left_contents.sort_unstable();
    right_contents.sort_unstable();
    let total_similarity_1 = left_contents
        .iter()
        .zip(right_contents.iter())
        .fold(0, |acc, pair| {
            acc + pair.0.abs_diff(*pair.1)
        });
    println!("{:?}", total_similarity_1);

    let mut count_map: HashMap<u32, u32> = HashMap::new();
    right_contents.iter().for_each(|&item| {
        count_map
            .entry(item)
            .and_modify(|val| *val += item)
            .or_insert(item);
    });
    let total_similarity_2 =
        left_contents.iter().fold(0, |acc, &val| {
            acc + *count_map.entry(val).or_default()
        });
    println!("{:?}", total_similarity_2);
}
