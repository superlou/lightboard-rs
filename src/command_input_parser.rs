use nom::character::complete::{digit1, one_of};
use nom::sequence::pair;
use nom::multi::many1;
use nom::IResult;
use crate::effect::{Command, Action};

fn alpha_any_case(i: &str) -> IResult<&str, char> {
    one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)
}

fn effect_keymap(i: &str) -> IResult<&str, (char, &str)> {
    pair(alpha_any_case, digit1)(i)
}

fn effects(i: &str) -> IResult<&str, Vec<(char, &str)>> {
    many1(effect_keymap)(i)
}

pub fn parse(buffer: &str) -> Vec<Command> {
    match effects(buffer) {
        Ok((_, parts)) => {
            parts.iter().map(|(a, b)| Command {
                key: (*a).to_string().to_uppercase() + *b,
                action: Action::Toggle,
            }).collect()
        },
        Err(err) => {
            dbg!(err);
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        assert_eq!(vec![
            Command {
                key: "A1".to_owned(),
                action: Action::Toggle,
            },
            Command {
                key: "E52".to_owned(),
                action: Action::Toggle,
            }
        ], parse("a1E52"));
    }
}
