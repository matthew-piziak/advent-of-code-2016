use std::str::FromStr;
use std::collections::HashSet;

type Error = &'static str;
type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Part {
    One,
    Two,
}

pub fn blocks_away(instructions: &str, part: Part) -> Result<i16> {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut direction = Direction::North;
    let mut seen = HashSet::new();
    for instruction in Instruction::try_many_from(instructions) {
        let instruction = instruction?;
        direction = direction.turn(instruction.turn);
        let blocks = instruction.blocks as i16;
        for _ in 0..blocks {
            if part == Part::Two {
                if seen.contains(&(x, y)) {
                    return Ok(x.abs() + y.abs());
                }
                seen.insert((x, y));
            }
            match direction {
                Direction::North => y += 1,
                Direction::East => x += 1,
                Direction::South => y -= 1,
                Direction::West => x -= 1,
            }
        }
    }
    match part {
        Part::One => Ok(x.abs() + y.abs()),
        Part::Two => Err("Never visted same location twice"),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Self {
        use self::Direction::*;
        use self::Turn::*;

        match (*self, turn) {
            (North, Left) | (South, Right) => West,
            (North, Right) | (South, Left) => East,
            (East, Left) | (West, Right) => North,
            (East, Right) | (West, Left) => South,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Instruction {
    turn: Turn,
    blocks: u16,
}

impl Instruction {
    #[cfg_attr(feature = "cargo-clippy", allow(needless_lifetimes))]
    fn try_many_from<'a>(s: &'a str) -> impl Iterator<Item = Result<Self>> + 'a {
        s.split(", ").map(str::parse)
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut chars = s.chars();
        let turn = match chars.next() {
            Some('L') => Turn::Left,
            Some('R') => Turn::Right,
            Some(_) => return Err("Turn character invalid"),
            None => return Err("Instruction string is empty"),
        };
        let blocks = try!(chars.as_str().parse().map_err(|_| "Could not parse blocks"));
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
        let instruction = "L1".parse::<Instruction>();
        assert_eq!(instruction,
                   Ok(Instruction {
                       turn: Turn::Left,
                       blocks: 1,
                   }));
        let instruction = "R2".parse::<Instruction>();
        assert_eq!(instruction,
                   Ok(Instruction {
                       turn: Turn::Right,
                       blocks: 2,
                   }));
    }

    #[test]
    fn test_parse_instruction_invalid_turn_character() {
        let instruction = "S1".parse::<Instruction>();
        assert_eq!(instruction, Err("Turn character invalid"));
    }

    #[test]
    fn test_parse_instruction_empty_string() {
        let instruction = "".parse::<Instruction>();
        assert_eq!(instruction, Err("Instruction string is empty"));
    }

    #[test]
    fn test_parse_instruction_missing_blocks_digit() {
        let instruction = "L".parse::<Instruction>();
        assert_eq!(instruction, Err("Could not parse blocks"));
    }

    #[test]
    fn test_parse_instruction_invalid_blocks_digit() {
        let instruction = "LL".parse::<Instruction>();
        assert_eq!(instruction, Err("Could not parse blocks"));
    }

    #[test]
    fn test_parse_instructions() {
        let instructions = Instruction::try_many_from("L1, R2").collect::<Result<Vec<_>>>();
        assert_eq!(instructions,
                   Ok(vec![Instruction {
                               turn: Turn::Left,
                               blocks: 1,
                           },
                           Instruction {
                               turn: Turn::Right,
                               blocks: 2,
                           }]));
    }

    // Assert that parsing failure returns the first error
    #[test]
    fn test_parse_instructions_with_error() {
        let instructions = Instruction::try_many_from("L1, , R2, S2").collect::<Result<Vec<_>>>();
        assert_eq!(instructions, Err("Instruction string is empty"));
    }

    #[test]
    fn test_blocks_away_part_one() {
        let blocks_away = blocks_away("L4, L1, L1", Part::One);
        assert_eq!(blocks_away, Ok(4));
    }

    #[test]
    fn test_blocks_away_part_one_advent_input() {
        let day_01_answer = blocks_away(include_str!("day_01_input"), Part::One);
        assert_eq!(day_01_answer, Ok(279));
    }

    #[test]
    fn test_blocks_away_part_two() {
        let blocks_away = blocks_away("R8, R4, R4, R8", Part::Two);
        assert_eq!(blocks_away, Ok(4));
    }

    #[test]
    fn test_blocks_away_part_two_advent_input() {
        let day_01_answer = blocks_away(include_str!("day_01_input"), Part::Two);
        assert_eq!(day_01_answer, Ok(163));
    }
}
