use std::fs;

const DAY_TEMP: &str = "{DAY}";
const FILE_PATH_TEMPLATE: &str =
    "/workspaces/Advent-of-Code-2024/inputs/day{DAY}.txt";

pub fn get_inputs(day: u8) -> Vec<Vec<u32>> {
    let file_name = FILE_PATH_TEMPLATE
        .replace(DAY_TEMP, &day.to_string());

    let file_contents = fs::read_to_string(&file_name)
        .expect(&format!(
            "Error reading from file: {}",
            &file_name
        ));

    return file_contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect()
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inputs = get_inputs(99);
        println!("{:?}", inputs);
        assert_eq!(inputs.len(), 3);
        assert_eq!(inputs[0], vec![1, 7, 2]);
        assert_eq!(inputs[1], vec![6, 3]);
        assert_eq!(inputs[2], vec![9, 1, 2, 8, 4, 2]);
    }
}
