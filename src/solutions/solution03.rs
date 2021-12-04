type Output = String;

struct Diagnostic {
    pub list: Vec<u32>,
    pub bit_width: usize,
}

impl Diagnostic {
    /// create Diagnostic from string
    fn from_string(string: &str) -> Diagnostic {
        let list: Vec<&str> = string.split('\n').filter(|&s| !s.is_empty()).collect();
        Self::from_str_list(&list)
    }

    fn from_str_list(list: &[&str]) -> Diagnostic {
        let bit_width = list[0].len();
        Diagnostic {
            list: list.iter().map(|s| parse_binary_string(s)).collect(),
            bit_width,
        }
    }

    fn power_consumption(&self) -> u32 {
        let gamma = self.find_gamma();
        let epsilon = self.find_epsilon(gamma);
        gamma * epsilon
    }

    /// generate a gamma for the diagnostic
    fn find_gamma(&self) -> u32 {
        // counts of the bits at each place
        let mut place_counts = vec![0; self.bit_width];
        for val in &self.list {
            // check each bit, add or subtract
            for (i, pc) in place_counts.iter_mut().enumerate() {
                *pc += if (val & (1 << i)) != 0 { 1 } else { -1 };
            }
        }
        // convert to a u32
        place_counts
            .iter()
            .rev()
            .fold(0, |a, &n| (a << 1) + (if n >= 0 { 1 } else { 0 }))
    }

    fn find_epsilon(&self, gamma: u32) -> u32 {
        let mask = (1 << self.bit_width) - 1;
        (gamma ^ mask) & mask
    }

    fn life_support_rating(&self) -> u32 {
        let ox = self.find_ox_generator_rating();
        let co2 = self.find_co2_scrubber_rating();
        ox * co2
    }

    fn find_ox_generator_rating(&self) -> u32 {
        let mask: u32 = 1 << (self.bit_width - 1);
        match Self::finder(mask, &self.list, |n| n >= 0) {
            Some(result) => result,
            None => 0,
        }
    }

    fn find_co2_scrubber_rating(&self) -> u32 {
        let mask: u32 = 1 << (self.bit_width - 1);
        match Self::finder(mask, &self.list, |n| n < 0) {
            Some(result) => result,
            None => 0,
        }
    }

    fn finder<F>(mask: u32, list: &[u32], compare: F) -> Option<u32>
    where
        F: Fn(i32) -> bool,
    {
        // count the number of bits set in the current position
        let pc = list.iter().filter(|&val| (val & mask) != 0).count() as i32;
        // compare set to unset, get the delta
        let delta = pc - (list.len() as i32 - pc);

        // compare ratio, either look for set (mask) or unset (0)
        let target = if compare(delta) { mask } else { 0 };

        // find all items in the list that have the proper bit set
        let findings: Vec<u32> = list
            .iter()
            .cloned()
            .filter(|&val| (val & mask) == target)
            .collect();

        if findings.len() == 1 {
            // one left, return it
            return Some(findings[0]);
        } else if mask > 1 {
            // mask is still valid, look at the next position
            return Self::finder(mask >> 1, &findings, compare);
        } else {
            return None;
        }
    }
}

fn parse_binary_string(bin_str: &str) -> u32 {
    // convert a char to a binary value
    let bin_char = |c: char| {
        if c == '1' {
            1
        } else {
            0
        }
    };

    // fold all binary chars in the string to a value
    bin_str.chars().fold(0, |a, c| (a << 1) + bin_char(c))
}

pub fn process_input(input: &str) -> Option<Output> {
    let diagnostic = Diagnostic::from_string(input);
    Some(format!(
        "\n\tPart A: {}\n\tPart B: {}",
        diagnostic.power_consumption(),
        diagnostic.life_support_rating(),
    ))
}

#[test]
fn test_power_consumption() {
    let diag = Diagnostic::from_str_list(&[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ]);
    assert_eq!(198, diag.power_consumption());
}

#[test]
fn test_diagnostic_gamma() {
    let diag = Diagnostic::from_str_list(&[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ]);
    assert_eq!(22, diag.find_gamma());
}

#[test]
fn test_diagnostic_epsilon() {
    let diag = Diagnostic::from_str_list(&[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ]);
    assert_eq!(9, diag.find_epsilon(diag.find_gamma()));
}

#[test]
fn test_diagnostic_find_ox() {
    let diag = Diagnostic::from_str_list(&[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ]);
    assert_eq!(0b10111, diag.find_ox_generator_rating());
}
