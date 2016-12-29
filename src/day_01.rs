use std::convert::TryFrom;

fn blocks_away(instructions: &str) -> i16 {
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::North;
    for instruction in Instruction::try_many_from(instructions).unwrap() {
        direction.turn(instruction.turn);
        match direction {
            Direction::North => y += instruction.blocks,
            Direction::East => x += instruction.blocks,
            Direction::South => y -= instruction.blocks,
            Direction::West => x -= instruction.blocks,
        }
        println!("direction: {:?}", direction);
        println!("x: {:?}", x);
        println!("y: {:?}", y);
    }
    x.abs() + y.abs()
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&mut self, turn: Turn) {
        *self = match *self {
            Direction::North => {
                match turn {
                    Turn::Left => Direction::West,
                    Turn::Right => Direction::East,
                }
            }
            Direction::East => {
                match turn {
                    Turn::Left => Direction::North,
                    Turn::Right => Direction::South,
                }
            }
            Direction::South => {
                match turn {
                    Turn::Left => Direction::East,
                    Turn::Right => Direction::West,
                }
            }
            Direction::West => {
                match turn {
                    Turn::Left => Direction::South,
                    Turn::Right => Direction::North,
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    turn: Turn,
    blocks: i16,
}

impl<'a> Instruction {
    fn try_many_from(s: &'a str) -> Result<Vec<Self>, &'a str> {
        s.split(", ").map(Self::try_from).collect()
    }
}

impl<'a> TryFrom<&'a str> for Instruction {
    type Err = &'a str;

    fn try_from(s: &'a str) -> Result<Self, &'a str> {
        use self::Turn::*;
        let mut chars = s.chars();
        let turn = match chars.next() {
            Some(turn_char) => {
                match turn_char {
                    'L' => Left,
                    'R' => Right,
                    _ => return Err("Turn character invalid"),
                }
            }
            None => return Err("Instruction string is empty"),
        };
        let blocks = try!(chars.as_str().parse::<i16>().map_err(|_| "Could not parse blocks"));
        Ok(Instruction {
            turn: turn,
            blocks: blocks,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_instruction_success() {
        let instruction = Instruction::try_from("L1").unwrap();
        assert_eq!(instruction,
                   Instruction {
                       turn: Turn::Left,
                       blocks: 1,
                   });
        let instruction = Instruction::try_from("R2").unwrap();
        assert_eq!(instruction,
                   Instruction {
                       turn: Turn::Right,
                       blocks: 2,
                   });
    }

    #[test]
    fn test_parse_instruction_invalid_turn_character() {
        let instruction = Instruction::try_from("S1");
        assert_eq!(instruction, Err("Turn character invalid"));
    }

    #[test]
    fn test_parse_instruction_empty_string() {
        let instruction = Instruction::try_from("");
        assert_eq!(instruction, Err("Instruction string is empty"));
    }

    #[test]
    fn test_parse_instruction_missing_blocks_digit() {
        let instruction = Instruction::try_from("L");
        assert_eq!(instruction, Err("Could not parse blocks"));
    }

    #[test]
    fn test_parse_instruction_invalid_blocks_digit() {
        let instruction = Instruction::try_from("LL");
        assert_eq!(instruction, Err("Could not parse blocks"));
    }

    #[test]
    fn test_parse_instructions() {
        let instructions = Instruction::try_many_from("L1, R2").unwrap();
        assert_eq!(instructions,
                   vec![Instruction {
                            turn: Turn::Left,
                            blocks: 1,
                        },
                        Instruction {
                            turn: Turn::Right,
                            blocks: 2,
                        }]);
    }

    // Assert that parsing failure returns the first error
    #[test]
    fn test_parse_instructions_with_error() {
        let instructions = Instruction::try_many_from("L1, , R2, S2");
        assert_eq!(instructions, Err("Instruction string is empty"));
    }

    #[test]
    fn test_blocks_away() {
        let blocks_away = blocks_away("L4, L1, L1");
        assert_eq!(blocks_away, 4);
    }

    #[test]
    fn test_blocks_away_advent_input() {
        let blocks_away =
            blocks_away("L4, L1, R4, R1, R1, L3, R5, L5, L2, L3, R2, R1, L4, R5, R4, L2, R1, R3, \
                         L5, R1, L3, L2, R5, L4, L5, R1, R2, L1, R5, L3, R2, R2, L1, R5, R2, L1, \
                         L1, R2, L1, R1, L2, L2, R4, R3, R2, L3, L188, L3, R2, R54, R1, R1, L2, \
                         L4, L3, L2, R3, L1, L1, R3, R5, L1, R5, L1, L1, R2, R4, R4, L5, L4, L1, \
                         R2, R4, R5, L2, L3, R5, L5, R1, R5, L2, R4, L2, L1, R4, R3, R4, L4, R3, \
                         L4, R78, R2, L3, R188, R2, R3, L2, R2, R3, R1, R5, R1, L1, L1, R4, R2, \
                         R1, R5, L1, R4, L4, R2, R5, L2, L5, R4, L3, L2, R1, R1, L5, L4, R1, L5, \
                         L1, L5, L1, L4, L3, L5, R4, R5, R2, L5, R5, R5, R4, R2, L1, L2, R3, R5, \
                         R5, R5, L2, L1, R4, R3, R1, L4, L2, L3, R2, L3, L5, L2, L2, L1, L2, R5, \
                         L2, L2, L3, L1, R1, L4, R2, L4, R3, R5, R3, R4, R1, R5, L3, L5, L5, L3, \
                         L2, L1, R3, L4, R3, R2, L1, R3, R1, L2, R4, L3, L3, L3, L1, L2");
        assert_eq!(blocks_away, 279);
    }
}
