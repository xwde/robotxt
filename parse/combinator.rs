use nom::bytes::complete::{tag, take_while};
use nom::combinator::opt;
use nom::IResult as NomResult;

const CARRIAGE: u8 = b'\r';
const NEWLINE: u8 = b'\n';
const COMMENT: u8 = b'#';

pub fn b_not_line_ending(c: u8) -> bool {
    c != NEWLINE && c != CARRIAGE
}

pub fn b_not_line_ending_or_comment(c: u8) -> bool {
    c != NEWLINE && c != CARRIAGE && c != COMMENT
}

pub fn b_consume_newline(input: &[u8]) -> NomResult<&[u8], Option<&[u8]>> {
    let (input, _) = take_while(|i| i == CARRIAGE)(input)?;
    let (input, output) = opt(tag(b"\n"))(input)?;
    Ok((input, output))
}
