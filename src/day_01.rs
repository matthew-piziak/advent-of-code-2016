use std::convert::TryFrom;

enum Turn {
    Left,
    Right,
}

struct Instruction {
    turn: Turn,
    blocks: u32,
}

impl TryFrom<String> for Instruction {
    type Err = String;

    fn try_from(s: String) -> Result<Self, String> {
        use self::Turn::*;
        let mut chars = s.chars();
        let turn = match chars.next() {
            Some(turn_char) => {
                match turn_char {
                    'L' => Left,
                    'R' => Right,
                    _ => return Err("Turn character invalid".into()),
                }
            }
            None => return Err("Instruction string is empty".into()),
        };
        let blocks = match chars.next() {
            Some(blocks_char) => {
                match blocks_char.to_digit(10) {
                    Some(digit) => digit,
                    None => {
                        return Err("Could not parse blocks digit".into());
                    }
                }
            }
            None => return Err("Blocks character not found".into()),
        };
        Ok(Self { turn, blocks })
    }
}
