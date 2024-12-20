advent_of_code::solution!(3);

// parse mul(44,46) -> multiply 44 and 46 -> 2024
// no other chars allowed, not even spaces
pub fn part_one(input: &str) -> Option<u64> {
    let muls = parse(input);
    Some(muls.iter().map(|(a, b)| a * b).sum::<u64>().into())
}

pub fn parse(inp: &str) -> Vec<(u64, u64)> {
    regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)")
        .unwrap()
        .captures_iter(inp)
        .map(|cap| {
            let a = u64::from_str_radix(&cap[1], 10).unwrap();
            let b = u64::from_str_radix(&cap[2], 10).unwrap();
            (a, b)
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut do_mul = true;
    let mut result = 0u64;
    for instr in parse_part_two(input) {
        match instr {
            Instruction::Mul(a, b) => {
                if do_mul {
                    result += a * b;
                }
            }
            Instruction::Do => do_mul = true,
            Instruction::Dont => do_mul = false,
        }
    }
    Some(result)
}

pub enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

pub fn parse_part_two(inp: &str) -> Vec<Instruction> {
    regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)")
        .unwrap()
        .captures_iter(inp)
        .map(|cap| match &cap[0][0..3] {
            "mul" => {
                let a = u64::from_str_radix(&cap[1], 10).unwrap();
                let b = u64::from_str_radix(&cap[2], 10).unwrap();
                Instruction::Mul(a, b)
            }
            "don" => Instruction::Dont,
            s if s.starts_with("do") => Instruction::Do,
            s => panic!("Regex captured invalid string ({s})"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
