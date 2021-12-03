type Output = String;

fn parse_binary_string(bin_str: &str) -> u32 {
    bin_str
        .chars()
        .fold(0, |a, c| (a << 1) + (if c == '1' { 1 } else { 0 }))
}

fn count_places(acc: Vec<i32>, bin_str: &str) -> Vec<i32> {
    let mut counts = acc.clone();
    if counts.len() < bin_str.len() {
        counts.resize(bin_str.len(), 0);
    }
    for (i, c) in bin_str.chars().enumerate() {
        if c == '1' {
            counts[i] += 1;
        } else {
            counts[i] -= 1;
        }
    }
    counts
}

fn place_count_to_gamma(a: i32, n: i32) -> i32 {
    (a << 1) + (if n > 0 { 1 } else { 0 })
}

fn generate_gamma(list: &[&str]) -> i32 {
    list.iter()
        .fold(vec![], |a, s| count_places(a, s))
        .iter()
        .fold(0, |a, &v| place_count_to_gamma(a, v))
}

fn generate_epsilon(gamma: i32, places: usize) -> i32 {
    // flip bits on the gamma to get epsilon
    let mask = (1 << places) - 1;
    (gamma ^ mask) & mask
}

fn process_list1(list: &[&str]) -> i32 {
    let gamma = generate_gamma(list);

    // flip bits on the gamma to get epsilon
    let places = list[0].len();
    let epsilon = generate_epsilon(gamma, places);

    // calculate the power
    gamma * epsilon
}

fn process_list2(list: &[&str]) -> i32 {
    // let gamma = generate_gamma(list);

    // // flip bits on the gamma to get epsilon
    // let places = list[0].len();
    // let epsilon = generate_epsilon(gamma, places);

    0
}

pub fn process_input(input: &str) -> Option<Output> {
    let list: Vec<&str> = input.split('\n').filter(|&s| !s.is_empty()).collect();
    let part_a = process_list1(&list);
    let part_b = process_list2(&list);
    let part_b = 0;
    Some(format!("\n\tPart A: {}\n\tPart B: {}", part_a, part_b))
}

#[test]
fn test_binary_parser() {
    assert_eq!(4, parse_binary_string("100"));
}

#[test]
fn test_count_places() {
    assert_eq!(vec![2, 0, 1, -1], count_places(vec![1, 1, 0, 0], "1010"));
}

#[test]
fn test_place_count_gamma() {
    assert_eq!(0b101 as i32, place_count_to_gamma(0b10 as i32, 15))
}

#[test]
fn test_gamma() {
    let str_list = ["10000", "00001", "10000"];
    assert_eq!(16, generate_gamma(&str_list))
}

#[test]
fn test_output1() {
    let input = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    assert_eq!(198, process_list1(&input));
}
