type Output = i32;

fn process_list(list: Vec<i32>) -> Option<Output> {
    // let mut list = list.clone();
    // list.sort();
    // // let (x, y) = search1(2020, &list)?;
    // Some(x * y)
    None
}

pub fn process_input(input: &str) -> Option<Output> {
    let list: Vec<i32> = input
        .split('\n')
        .filter(|&s| s.len() > 0)
        .map(|s| -> i32 { s.parse().unwrap() })
        .collect();
    process_list(list)
}
