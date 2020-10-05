use nom::character::complete::multispace0;

use super::*;
use crate::Span;

pub(crate) fn parse_maidata_insns(input: Span) -> nom::IResult<Span, Vec<RawInsn>> {
    use nom::multi::many0;

    many0(parse_one_maidata_insn)(input)
}

fn parse_one_maidata_insn(input: Span) -> nom::IResult<Span, RawInsn> {
    let (s, _) = multispace0(input)?;
    let (s, insn) = nom::branch::alt((t_bpm, t_beat_divisor, t_rest, t_tap_single))(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, insn))
}

fn t_note_sep(input: Span) -> nom::IResult<Span, ()> {
    use nom::character::complete::char;

    let (s, _) = multispace0(input)?;
    let (s, _) = char(',')(s)?;
    Ok((s, ()))
}

fn t_bpm(input: Span) -> nom::IResult<Span, RawInsn> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, _) = multispace0(input)?;
    let (s, _) = char('(')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, bpm) = float(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(')')(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, RawInsn::Bpm(BpmParams { new_bpm: bpm })))
}

fn t_beat_divisor(input: Span) -> nom::IResult<Span, RawInsn> {
    use nom::character::complete::char;
    use nom::character::complete::digit1;

    let (s, _) = multispace0(input)?;
    let (s, _) = char('{')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, divisor_str) = digit1(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;
    let (s, _) = multispace0(s)?;

    // TODO: out-of-range conversion failures
    let divisor = divisor_str.fragment().parse().unwrap();

    Ok((
        s,
        RawInsn::BeatDivisor(BeatDivisorParams {
            new_divisor: divisor,
        }),
    ))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn t_key(input: Span) -> nom::IResult<Span, Key> {
    use std::convert::TryFrom;
    use nom::combinator::map;
    use nom::character::complete::one_of;

    map(one_of("12345678"), |s| Key::try_from(s).unwrap())(input)
}

fn t_rest(input: Span) -> nom::IResult<Span, RawInsn> {
    let (s, _) = multispace0(input)?;
    let (s, _) = t_note_sep(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, RawInsn::Rest))
}

fn t_tap_single(input: Span) -> nom::IResult<Span, RawInsn> {
    let (s, _) = multispace0(input)?;
    let (s, key) = t_key(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, _) = multispace0(s)?;

    Ok((
        s,
        RawInsn::Note(RawNoteInsn::Tap(TapParams {
            variant: TapVariant::Tap,
            key,
        })),
    ))
}
