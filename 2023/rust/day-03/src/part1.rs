use std::collections::BTreeMap;

use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug)]
enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(
                move |(x, character)| {
                    (
                    (y, x),
                    match character {
                        '.' => Value::Empty,
                        c if c.is_ascii_digit() => {
                            Value::Number(
                                c.to_digit(10).expect(
                                    "should be a number",
                                ),
                            )
                        }
                        c => Value::Symbol(c),
                    },
                )
                },
            )
        })
        .collect::<BTreeMap<(usize, usize), Value>>();
    let mut numbers: Vec<Vec<((usize, usize), u32)>> =
        vec![];
    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers
                                    .iter_mut()
                                    .last()
                                    .expect("should exist");
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![(
                                    (*x, *y),
                                    *num,
                                )]);
                            }
                        }
                        None => unimplemented!(
                            "shouldn't happen"
                        ),
                    }
                }
                None => {
                    numbers.push(vec![((*x, *y), *num)]);
                }
            }
            println!("{x}: {y}");
        }
    }

    // map: entire grid
    // numbers: sequential numbers
    for num_list in numbers {
        let positions = [
            // (x,y)
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let pos_to_check = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions.iter().map(|outer_pos| {
                    // outer_pos.x + pos.x, .y + .y
                    (
                        outer_pos.0 + pos.1,
                        outer_pos.1 + pos.0,
                    )
                })
            })
            .unique()
            .collect();
    }

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}