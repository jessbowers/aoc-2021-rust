type Output = String;

fn process_list_single_value(list: &[i32]) -> i32 {
    let mut prev: Option<i32> = None;
    let mut count = 0;
    for &x in list {
        if let Some(p) = prev {
            if p < x {
                count += 1;
            }
        }
        prev = Some(x);
    }
    count
}

fn process_list_three_values(list: &[i32]) -> i32 {
    let mut count = 0;
    for (i, _) in list.iter().enumerate() {
        if i >= 3 {
            let prev: i32 = list[(i - 3)..i].iter().sum();
            let curr: i32 = list[(i - 2)..(i + 1)].iter().sum();
            if prev < curr {
                count += 1;
            }
        }
    }
    count
}

pub fn process_input(input: &str) -> Option<Output> {
    let list: Vec<i32> = input
        .split('\n')
        .filter(|&s| !s.is_empty())
        .map(|s| -> i32 { s.parse().unwrap() })
        .collect();
    let part_a = process_list_single_value(&list);
    let part_b = process_list_three_values(&list);
    Some(format!("\n\tPart A: {}\n\tPart B: {}", part_a, part_b))
}
