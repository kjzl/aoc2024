advent_of_code::solution!(5);

// page 0 must be somewhere before page 1 in a update
#[derive(Debug)]
pub struct OrderingRule(u64, u64);

pub struct UpdateError {
    p1_idx: usize,
    p2_idx: usize,
}

// Contains a Vec of page numbers that are contained in the update
#[derive(Debug)]
pub struct Update(Vec<u64>);

impl Update {
    fn is_in_order(&self, rules: &[OrderingRule]) -> bool {
        self.first_idx_of_page_not_in_order(rules).is_none()
    }

    fn first_idx_of_page_not_in_order(&self, rules: &[OrderingRule]) -> Option<UpdateError> {
        for rule in rules.iter() {
            let page_indices = [
                self.0.iter().position(|page| *page == rule.0),
                self.0.iter().position(|page| *page == rule.1),
            ];
            if let [Some(p1_idx), Some(p2_idx)] = page_indices {
                if p1_idx > p2_idx {
                    return Some(UpdateError { p1_idx, p2_idx });
                }
            }
        }
        None
    }

    fn bring_in_order(&mut self, rules: &[OrderingRule]) -> bool {
        let Some(err) = self.first_idx_of_page_not_in_order(rules) else {
            return true;
        };
        let p1 = self.0.remove(err.p1_idx);
        self.0.insert(err.p2_idx, p1);
        return self.bring_in_order(rules);
    }
}

#[inline]
pub fn split_once<F, T>(arr: &[T], pred: F) -> Option<(&[T], &[T])>
where
    F: FnMut(&T) -> bool,
{
    let index = arr.iter().position(pred)?;
    Some((&arr[..index], &arr[index + 1..]))
}

fn parse_input(input: &str) -> (Vec<OrderingRule>, Vec<Update>) {
    let lines = input.lines().collect::<Vec<_>>();
    let (rules, updates) = split_once(&lines, |line| *line == "").unwrap();

    let rules: Vec<OrderingRule> = rules
        .into_iter()
        .map(|line| {
            let (p1, p2) = line
                .split_once('|')
                .expect("rule must have exactly one ´|´ to seperate the page numbers");
            OrderingRule(p1.parse().unwrap(), p2.parse().unwrap())
        })
        .collect();

    let updates: Vec<Update> = updates
        .into_iter()
        .map(|line| Update(line.split(',').map(|page| page.parse().unwrap()).collect()))
        .collect();
    (rules, updates)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let (rules, updates) = parse_input(input);
    for update in updates {
        if update.is_in_order(&rules) {
            result += update.0[update.0.len() / 2];
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let (rules, mut updates) = parse_input(input);
    updates.retain(|update| !update.is_in_order(&rules));
    updates.iter_mut().for_each(|update| {
        if !update.bring_in_order(&rules) {
            panic!("could not bring update {update:?} in order!");
        }
    });
    for update in updates {
        result += update.0[update.0.len() / 2];
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
