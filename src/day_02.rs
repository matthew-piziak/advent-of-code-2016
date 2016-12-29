use std::convert::TryFrom;

type Error = String;
type Result<T> = ::std::result::Result<T, Error>;

fn code(instructions: &str) -> &str {
    ""
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Up,
    Right,
    Down,
    Left,
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
}
