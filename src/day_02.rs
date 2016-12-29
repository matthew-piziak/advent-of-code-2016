use std::convert::TryFrom;

type Error = String;
type Result<T> = ::std::result::Result<T, Error>;

// Keypad structure:
// 1 2 3
// 4 5 6
// 7 8 9
pub fn code(instructions: &str) -> Result<String> {
    use self::Instruction::*;

    let mut code = String::from("");
    let mut curr_key: Key = 5;
    for instruction in Instruction::try_many_from(instructions) {
        let instruction = instruction?;
        let row = (curr_key - 1) / 3;
        let col = (curr_key - 1) % 3;
        match (instruction, row, col) {
            (Up, 0, _) => {
                curr_key = curr_key;
            }
            (Up, _, _) => {
                curr_key = curr_key - 3;
            }
            (Down, 2, _) => {
                curr_key = curr_key;
            }
            (Down, _, _) => {
                curr_key = curr_key + 3;
            }
            (Left, _, 0) => {
                curr_key = curr_key;
            }
            (Left, _, _) => {
                curr_key = curr_key - 1;
            }
            (Right, _, 2) => {
                curr_key = curr_key;
            }
            (Right, _, _) => {
                curr_key = curr_key + 1;
            }
            (End, _, _) => {
                code.push_str(&format!("{}", curr_key));
            }
        }
    }
    Ok(code)
}

type Key = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
    End,
}

impl Instruction {
    fn try_many_from<'a>(s: &'a str) -> impl Iterator<Item = Result<Self>> + 'a {
        s.chars().map(Self::try_from)
    }
}

impl TryFrom<char> for Instruction {
    type Err = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            'U' => Ok(Instruction::Up),
            'R' => Ok(Instruction::Right),
            'D' => Ok(Instruction::Down),
            'L' => Ok(Instruction::Left),
            '\n' => Ok(Instruction::End),
            invalid @ _ => Err(format!("Instruction invalid: {}", invalid)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Instruction::*;

    #[test]
    fn test_parse_instruction_success() {
        let instruction = Instruction::try_from('U');
        assert_eq!(instruction, Ok(Up));
    }

    #[test]
    fn test_parse_instructions_success() {
        let instructions = Instruction::try_many_from("ULRD\nU").collect::<Result<Vec<_>>>();
        assert_eq!(instructions, Ok(vec![Up, Left, Right, Down, End, Up]));
    }

    #[test]
    fn test_parse_instruction_invalid() {
        let instruction = Instruction::try_from('x');
        assert_eq!(instruction, Err("Instruction invalid: x".into()));
    }

    #[test]
    fn test_parse_instructions_invalid() {
        let instructions = Instruction::try_many_from("ULxDy").collect::<Result<Vec<_>>>();
        assert_eq!(instructions, Err("Instruction invalid: x".into()));
    }

    #[test]
    fn test_code() {
        let code = code("ULL\nRRDDD\nLURDL\nUUUUD\n");
        assert_eq!(code, Ok("1985".into()));
    }

    #[test]
    fn test_code_advent_input() {
        let day_02_answer = code(include_str!("day_02_input"));
        assert_eq!(day_02_answer, Ok("69642".into()));
    }
}
