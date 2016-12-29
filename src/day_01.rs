use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    turn: Turn,
    blocks: u32,
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
        let blocks = match chars.next() {
            Some(blocks_char) => {
                match blocks_char.to_digit(10) {
                    Some(digit) => digit,
                    None => {
                        return Err("Could not parse blocks digit");
                    }
                }
            }
            None => return Err("Blocks character not found"),
        };
        Ok(Self {
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
        assert_eq!(instruction, Err("Blocks character not found"));
    }

    #[test]
    fn test_parse_instruction_invalid_blocks_digit() {
        let instruction = Instruction::try_from("LL");
        assert_eq!(instruction, Err("Could not parse blocks digit"));
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
}
