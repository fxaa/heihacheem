use nom::IResult;

use super::{
    inputs::{parse_directional_input, parse_facebutton_input, MoveInput},
    stances::{parse_stance, Stance},
};

pub struct NotatedMoveInput {
    stance: Stance,
    input: MoveInput,
}

pub fn parse_notation(input: &str) -> IResult<&str, &str> {
    let (post_stance_input, stance) = parse_stance(input)?;
}

pub fn parse_input_sequence(input: &str) -> IResult<&str, &str> {
    tuple!(())
}
