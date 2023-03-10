use std::cmp::min;

use bstr::ByteSlice;
use nom::branch::{alt, Alt};
use nom::bytes::complete::{tag, tag_no_case, take_while};
use nom::character::complete::{space0, space1};
use nom::combinator::{eof, opt};
use nom::error::{Error as NomError, ParseError as NomParseError};
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::{Err as NomErr, IResult as NomResult};

use crate::parse::Directive;

const CARRIAGE: u8 = b'\r';
const NEWLINE: u8 = b'\n';
const COMMENT: u8 = b'#';

///
pub fn b_not_line_ending(c: u8) -> bool {
    c != NEWLINE && c != CARRIAGE
}

///
pub fn b_not_line_ending_or_comment(c: u8) -> bool {
    c != NEWLINE && c != CARRIAGE && c != COMMENT
}

///
pub fn b_consume_newline(input: &[u8]) -> NomResult<&[u8], Option<&[u8]>> {
    let (input, _) = take_while(|i| i == CARRIAGE)(input)?;
    let (input, output) = opt(tag(b"\n"))(input)?;
    Ok((input, output))
}

///
fn builder<'a, O, E: NomParseError<&'a [u8]>>(
    input: &'a [u8],
    spellings: impl Alt<&'a [u8], O, E>,
) -> NomResult<&'a [u8], &'a [u8]>
where
    NomErr<NomError<&'a [u8]>>: From<NomErr<E>>,
{
    // Tries to match to the spelling list.
    let (input, _) = preceded(space0, alt(spellings))(input)?;
    // Tries to match the separator (colon or spaces).
    let (input, _) = alt((preceded(space0, tag(":")), space1))(input)?;
    // Tries to retrieve the value of the kv pair.
    let (input, line) = take_while(b_not_line_ending_or_comment)(input)?;

    // Skips the rest.
    let (input, _) = opt(preceded(tag("#"), take_while(b_not_line_ending)))(input)?;
    let (input, _) = b_consume_newline(input)?;

    let line = line.trim();
    Ok((input, line))
}

/// Attempts to parse the `user-agent` directive.
fn user_agent(input: &[u8]) -> NomResult<&[u8], Directive> {
    let matcher = (
        tag_no_case("user-agent"),
        tag_no_case("user agent"),
        tag_no_case("useragent"),
    );

    let (input, agent) = builder(input, matcher)?;
    Ok((input, Directive::UserAgent(agent)))
}

/// Attempts to parse the `allow` directive.
fn allow(input: &[u8]) -> NomResult<&[u8], Directive> {
    let matcher = (
        tag_no_case("allow"),
        tag_no_case("alow"),
        tag_no_case("allaw"),
    );

    let (input, rule) = builder(input, matcher)?;
    Ok((input, Directive::Allow(rule)))
}

/// Attempts to parse the `disallow` directive.
fn disallow(input: &[u8]) -> NomResult<&[u8], Directive> {
    let matcher = (
        tag_no_case("disallow"),
        tag_no_case("dissallow"),
        tag_no_case("dissalow"),
        tag_no_case("disalow"),
        tag_no_case("diasllow"),
        tag_no_case("disallaw"),
    );

    // Empty disallow is equivalent to allow all.
    // https://moz.com/learn/seo/robotstxt
    let (input, rule) = builder(input, matcher)?;
    match rule.is_empty() {
        true => Ok((input, Directive::Allow(b"/"))),
        false => Ok((input, Directive::Disallow(rule))),
    }
}

/// Attempts to parse the `crawl-delay` directive.
fn crawl_delay(input: &[u8]) -> NomResult<&[u8], Directive> {
    let matcher = (
        tag_no_case("crawl-delay"),
        tag_no_case("crawl delay"),
        tag_no_case("crawldelay"),
    );

    let (input, delay) = builder(input, matcher)?;
    Ok((input, Directive::CrawlDelay(delay)))
}

/// Attempts to parse the `sitemap` directive.
fn sitemap(input: &[u8]) -> NomResult<&[u8], Directive> {
    let matcher = (
        tag_no_case("sitemap"),
        tag_no_case("site-map"),
        tag_no_case("site map"),
    );

    let (input, sitemap) = builder(input, matcher)?;
    Ok((input, Directive::Sitemap(sitemap)))
}

/// Consumes the line as no directives were found here.
fn unknown(input: &[u8]) -> NomResult<&[u8], Directive> {
    let (input, unknown) = take_while(b_not_line_ending)(input)?;
    let (input, _) = b_consume_newline(input)?;
    Ok((input, Directive::Unknown(unknown)))
}

/// Google currently enforces a robots.txt file size limit of 500 kibibytes (KiB).
/// https://developers.google.com/search/docs/crawling-indexing/robots/robots_txt
pub const BYTES_LIMIT: usize = 512_000;

/// Parses the input slice into the list of directives.
fn parse(input: &[u8]) -> NomResult<&[u8], Vec<Directive>> {
    // Limits the input to 500 kibibytes.
    let limit = min(input.len(), BYTES_LIMIT);
    let input = &input[0..limit];

    // Removes the byte order mark (BOM).
    let (input, _) = opt(tag(b"\xef"))(input)?;
    let (input, _) = opt(tag(b"\xbb"))(input)?;
    let (input, _) = opt(tag(b"\xbf"))(input)?;

    // Creates and runs the matcher.
    let matcher = alt((user_agent, allow, disallow, crawl_delay, sitemap, unknown));
    let (input, (directives, _)) = many_till(matcher, eof)(input)?;

    Ok((input, directives))
}

/// Parses the input slice into the list of directives.
pub fn into_directives(input: &[u8]) -> Vec<Directive> {
    // Discards the possibility of any error as `unknown` consumes anything.
    match parse(input) {
        Ok((_, directives)) => directives,
        Err(_) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn foo() {}
}
