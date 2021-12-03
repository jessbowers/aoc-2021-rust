type Output = String;

#[derive(Clone, Debug, Default, PartialEq)]
struct Submarine {
    pub x: i32,
    pub y: i32,
    pub aim: i32,
}

impl Submarine {
    pub fn move_by(&self, direction: &Direction) -> Submarine {
        match direction {
            Direction::Up(y) => Submarine {
                y: self.y - y,
                ..*self
            },
            Direction::Down(y) => Submarine {
                y: self.y + y,
                ..*self
            },
            Direction::Forward(x) => Submarine {
                x: self.x + x,
                ..*self
            },
        }
    }
    pub fn move_and_aim(&self, direction: &Direction) -> Submarine {
        match direction {
            Direction::Up(y) => Submarine {
                aim: self.aim - y,
                ..*self
            },
            Direction::Down(y) => Submarine {
                aim: self.aim + y,
                ..*self
            },
            Direction::Forward(x) => Submarine {
                x: self.x + x,
                y: self.y + x * self.aim,
                ..*self
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn parse_direction(input: &str) -> Option<Direction> {
    let mut tokens = input.split_whitespace();
    let dir = tokens.next();
    let value: i32 = tokens.next().unwrap().parse().unwrap();

    match dir {
        Some("up") => Some(Direction::Up(value)),
        Some("down") => Some(Direction::Down(value)),
        Some("forward") => Some(Direction::Forward(value)),
        _ => None,
    }
}

fn process_list1(list: &[Direction]) -> i32 {
    let zero: Submarine = Default::default();
    let loc = list.iter().fold(zero, |sub, d| sub.move_by(d));
    loc.x * loc.y
}

fn process_list2(list: &[Direction]) -> i32 {
    let zero: Submarine = Default::default();
    let loc = list.iter().fold(zero, |sub, d| sub.move_and_aim(d));
    loc.x * loc.y
}

pub fn process_input(input: &str) -> Option<Output> {
    let list: Vec<Direction> = input
        .split('\n')
        .filter(|&s| !s.is_empty())
        .map(|s| -> Direction { parse_direction(s).unwrap() })
        .collect();
    let part_a = process_list1(&list);
    let part_b = process_list2(&list);
    Some(format!("\n\tPart A: {}\n\tPart B: {}", part_a, part_b))
}

#[test]
fn parse_direction1() {
    assert_eq!(parse_direction("forward 9"), Some(Direction::Forward(9)))
}

#[test]
fn forward() {
    let sub = Submarine { x: 5, y: 1, aim: 5 };
    assert_eq!(
        Submarine {
            x: 13,
            y: 41,
            aim: 5
        },
        sub.move_and_aim(&Direction::Forward(8))
    )
}

#[test]
fn down() {
    let sub = Submarine { x: 5, y: 1, aim: 5 };
    assert_eq!(
        Submarine {
            x: 5,
            y: 1,
            aim: 13
        },
        sub.move_and_aim(&Direction::Down(8))
    )
}

#[test]
fn up() {
    let sub = Submarine { x: 5, y: 1, aim: 5 };
    assert_eq!(
        Submarine { x: 5, y: 1, aim: 3 },
        sub.move_and_aim(&Direction::Up(2))
    )
}
