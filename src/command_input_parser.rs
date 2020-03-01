use nom::character::complete::{digit1, one_of};
use nom::sequence::pair;
use nom::multi::many1;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::separated_list;
use nom::IResult;
use crate::effect::{Command, Action};

#[derive(Debug, PartialEq)]
pub enum Chunk {
    Effect(Command),
    CueNum(usize),
}

fn alpha_any_case(i: &str) -> IResult<&str, char> {
    one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)
}

fn effect_keymap(i: &str) -> IResult<&str, Chunk> {
    match pair(alpha_any_case, digit1)(i) {
        Ok((i, o)) => {
            let command = Command {
                key: o.0.to_string().to_uppercase() + o.1,
                action: Action::Toggle,
            };
            Ok((i, Chunk::Effect(command)))
        },
        Err(e) => Err(e),
    }
}

fn cue_num(i: &str) -> IResult<&str, Chunk> {
    match digit1(i) {
        Ok((i, o)) => Ok((i, Chunk::CueNum(o.parse::<usize>().unwrap()))),
        Err(e) => Err(e)
    }
}

fn chunk(i: &str) -> IResult<&str, Chunk> {
    alt((effect_keymap, cue_num))(i)
}

fn chunks(i: &str) -> IResult<&str, Vec<Chunk>> {
    separated_list(tag(" "), chunk)(i)
}

pub fn parse(buffer: &str) -> Vec<Chunk> {
    match chunks(buffer) {
        Ok((_, parts)) => {
            parts
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
            Chunk::Effect(Command {
                key: "A1".to_owned(),
                action: Action::Toggle,
            }),
            Chunk::CueNum(102),
            Chunk::Effect(Command {
                key: "E52".to_owned(),
                action: Action::Toggle,
            })
        ], parse("a1 102 E52"));
    }
}
