advent_of_code::solution!(4);

// we need to find XMAS in horizontal, vertical, diagonal (can be written backwards)
pub fn part_one(input: &str) -> Option<u64> {
    let matrix = parse_input(input);
    let mut result = 0u64;
    let xmas = ["XMAS", "SAMX"];
    for xmas in xmas {
        result += count_xmas_horizontal(&matrix, xmas);
        result += count_xmas_vertical(&matrix, xmas);
        result += count_xmas_diagonal(&matrix, xmas);
    }
    Some(result)
}

// we want to parse the file as a matrix of chars
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn count_xmas_horizontal(matrix: &[Vec<char>], xmas: &str) -> u64 {
    let mut count = 0;
    let xmas = xmas.chars().collect::<Vec<char>>();
    for row in matrix {
        for i in 0..row.len() - 3 {
            if row[i..i + 4] == xmas[..4] {
                count += 1;
            }
        }
    }
    count
}

pub fn count_xmas_vertical(matrix: &[Vec<char>], xmas: &str) -> u64 {
    let mut count = 0;
    for i in 0..matrix[0].len() {
        for j in 0..matrix.len() - 3 {
            if (0..4).all(|k| matrix[j + k][i] == xmas[k..k + 1].chars().next().unwrap()) {
                count += 1;
            }
        }
    }
    count
}

pub fn count_xmas_diagonal(matrix: &[Vec<char>], xmas: &str) -> u64 {
    let mut count = 0;
    for i in 0..matrix.len() - 3 {
        for j in 0..matrix[0].len() - 3 {
            if (0..4).all(|k| matrix[i + k][j + k] == xmas[k..k + 1].chars().next().unwrap()) {
                count += 1;
            }
        }
    }
    for i in 0..matrix.len() - 3 {
        for j in 3..matrix[0].len() {
            if (0..4).all(|k| matrix[i + k][j - k] == xmas[k..k + 1].chars().next().unwrap()) {
                count += 1;
            }
        }
    }
    count
}

/* PART TWO */

pub fn part_two(input: &str) -> Option<u64> {
    let matrix = parse_input(input);
    let mut result = 0u64;
    for i in 0..matrix.len() - 2 {
        for j in 0..matrix[0].len() - 2 {
            result += has_xmas(i, j, &matrix) as u64;
        }
    }
    Some(result)
}

// this function checks from matrix[i][j] to the right and down for a match
pub fn has_xmas(i: usize, j: usize, matrix: &[Vec<char>]) -> bool {
    (has_xmas_diag_left(i, j, matrix, "MAS") || has_xmas_diag_left(i, j, matrix, "SAM"))
        && (has_xmas_diag_right(i, j, matrix, "MAS") || has_xmas_diag_right(i, j, matrix, "SAM"))
}

// this function checks from matrix[i][j] to the right and down for a match
pub fn has_xmas_diag_right(i: usize, j: usize, matrix: &[Vec<char>], xmas: &str) -> bool {
    for char_i in 0..3 {
        if matrix[i + char_i][j + char_i] != xmas.chars().skip(char_i).next().unwrap() {
            break;
        }
        if char_i == 2 {
            return true;
        }
    }
    false
}

pub fn has_xmas_diag_left(i: usize, j: usize, matrix: &[Vec<char>], xmas: &str) -> bool {
    for char_i in 0..3 {
        if matrix[i + char_i][j + 2 - char_i] != xmas.chars().skip(char_i).next().unwrap() {
            break;
        }
        if char_i == 2 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
