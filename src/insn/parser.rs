use nom::character::complete::multispace0;

use super::*;
use crate::Span;

pub(crate) fn parse_maidata_insns(input: Span) -> nom::IResult<Span, Vec<RawInsn>> {
    use nom::multi::many0;

    let (s, insns) = many0(parse_one_maidata_insn)(input)?;
    let (s, _) = t_eof(s)?;

    Ok((s, insns))
}

fn t_eof(input: Span) -> nom::IResult<Span, Span> {
    nom::eof!(input,)
}

fn parse_one_maidata_insn(input: Span) -> nom::IResult<Span, RawInsn> {
    let (s, _) = multispace0(input)?;
    let (s, insn) = nom::branch::alt((
        t_bpm,
        t_beat_divisor,
        t_rest,
        t_tap_single,
        t_tap_multi_simplified,
        t_hold_single,
        t_slide_single,
        t_bundle,
        t_end_mark,
    ))(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, insn))
}

fn t_end_mark(input: Span) -> nom::IResult<Span, RawInsn> {
    use nom::character::complete::char;

    let (s, _) = multispace0(input)?;
    let (s, _) = char('E')(s)?;
    Ok((s, RawInsn::EndMark))
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

fn t_tap_param(input: Span) -> nom::IResult<Span, TapParams> {
    use nom::character::complete::char;
    use nom::combinator::opt;

    let (s, _) = multispace0(input)?;
    let (s, key) = t_key(s)?;
    let (s, _) = multispace0(s)?;
    let (s, is_break) = opt(char('b'))(s)?;
    let (s, _) = multispace0(s)?;

    let variant = match is_break {
        Some(_) => TapVariant::Break,
        None => TapVariant::Tap,
    };

    Ok((s, TapParams { variant, key }))
}

fn t_tap(input: Span) -> nom::IResult<Span, RawNoteInsn> {
    let (s, _) = multispace0(input)?;
    let (s, params) = t_tap_param(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, RawNoteInsn::Tap(params)))
}

fn t_tap_single(input: Span) -> nom::IResult<Span, RawInsn> {
    let (s, _) = multispace0(input)?;
    let (s, note) = t_tap(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, RawInsn::Note(note)))
}

fn t_tap_multi_simplified(input: Span) -> nom::IResult<Span, RawInsn> {
    use nom::multi::many1;

    let (s, _) = multispace0(input)?;
    // TODO: do whitespaces inside a taps bundle get ignored as well?
    let (s, keys) = many1(t_key)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, _) = multispace0(s)?;

    // all taps are regular ones when using simplified notation
    let notes = keys
        .into_iter()
        .map(|key| {
            RawNoteInsn::Tap(TapParams {
                variant: TapVariant::Tap,
                key,
            })
        })
        .collect();

    Ok((s, RawInsn::NoteBundle(notes)))
}

fn t_len(input: Span) -> nom::IResult<Span, Length> {
    use nom::character::complete::char;
    use nom::character::complete::digit1;

    // TODO: absolute time support ('#')
    let (s, _) = multispace0(input)?;
    let (s, _) = char('[')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, divisor_str) = digit1(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(':')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, num_str) = digit1(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(']')(s)?;
    let (s, _) = multispace0(s)?;

    // TODO: handle conversion errors
    let divisor = divisor_str.fragment().parse().unwrap();
    let num = num_str.fragment().parse().unwrap();

    Ok((s, Length::NumBeats(NumBeatsParams { divisor, num })))
}

fn t_hold(input: Span) -> nom::IResult<Span, RawNoteInsn> {
    use nom::character::complete::char;

    let (s, _) = multispace0(input)?;
    let (s, key) = t_key(s)?;
    let (s, _) = char('h')(s)?;
    let (s, len) = t_len(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, RawNoteInsn::Hold(HoldParams { key, len })))
}

fn t_hold_single(input: Span) -> nom::IResult<Span, RawInsn> {
    let (s, _) = multispace0(input)?;
    let (s, note) = t_hold(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, RawInsn::Note(note)))
}

// FxE[len] where x is single char
// covers everything except FppE FqqE and FVRE
macro_rules! define_slide_track {
    (@ $fn_name: ident, $recog: expr, $variant: ident) => {
        fn $fn_name(input: Span) -> nom::IResult<Span, SlideTrack> {
            use nom::character::complete::char;

            let (s, _) = multispace0(input)?;
            let (s, _) = $recog(s)?;
            let (s, _) = multispace0(s)?;
            // TODO: can slide ends be breaks?
            let (s, destination) = t_tap_param(s)?;
            let (s, _) = multispace0(s)?;
            let (s, len) = t_len(s)?;
            let (s, _) = multispace0(s)?;

            Ok((
                s,
                SlideTrack::$variant(SlideTrackParams {
                    destination,
                    interim: None,
                    len,
                }),
            ))
        }
    };

    ($fn_name: ident, char $ch: expr, $variant: ident) => {
        define_slide_track!(@ $fn_name, char($ch), $variant);
    };
}

define_slide_track!(t_slide_track_line, char '-', Line);
define_slide_track!(t_slide_track_arc, char '^', Arc);
define_slide_track!(t_slide_track_circ_left, char '<', CircumferenceLeft);
define_slide_track!(t_slide_track_circ_right, char '>', CircumferenceRight);
define_slide_track!(t_slide_track_v, char 'v', V);
define_slide_track!(t_slide_track_p, char 'p', P);
define_slide_track!(t_slide_track_q, char 'q', Q);
define_slide_track!(t_slide_track_s, char 's', S);
define_slide_track!(t_slide_track_z, char 'z', Z);
define_slide_track!(t_slide_track_spread, char 'w', Spread);

fn t_slide_track_pp(input: Span) -> nom::IResult<Span, SlideTrack> {
    use nom::bytes::complete::tag;

    let (s, _) = multispace0(input)?;
    let (s, _) = tag("pp")(s)?;
    let (s, _) = multispace0(s)?;
    // TODO: can slide ends be breaks?
    let (s, destination) = t_tap_param(s)?;
    let (s, _) = multispace0(s)?;
    let (s, len) = t_len(s)?;
    let (s, _) = multispace0(s)?;

    Ok((
        s,
        SlideTrack::Pp(SlideTrackParams {
            destination,
            interim: None,
            len,
        }),
    ))
}

fn t_slide_track_qq(input: Span) -> nom::IResult<Span, SlideTrack> {
    use nom::bytes::complete::tag;

    let (s, _) = multispace0(input)?;
    let (s, _) = tag("qq")(s)?;
    let (s, _) = multispace0(s)?;
    // TODO: can slide ends be breaks?
    let (s, destination) = t_tap_param(s)?;
    let (s, _) = multispace0(s)?;
    let (s, len) = t_len(s)?;
    let (s, _) = multispace0(s)?;

    Ok((
        s,
        SlideTrack::Qq(SlideTrackParams {
            destination,
            interim: None,
            len,
        }),
    ))
}

fn t_slide_track_angle(input: Span) -> nom::IResult<Span, SlideTrack> {
    use nom::character::complete::char;

    let (s, _) = multispace0(input)?;
    let (s, _) = char('V')(s)?;
    let (s, _) = multispace0(s)?;
    // TODO: can these two be breaks?
    let (s, interim) = t_tap_param(s)?;
    let (s, _) = multispace0(s)?;
    let (s, destination) = t_tap_param(s)?;
    let (s, _) = multispace0(s)?;
    let (s, len) = t_len(s)?;
    let (s, _) = multispace0(s)?;

    Ok((
        s,
        SlideTrack::Angle(SlideTrackParams {
            destination,
            interim: Some(interim),
            len,
        }),
    ))
}

fn t_slide_track(input: Span) -> nom::IResult<Span, SlideTrack> {
    nom::branch::alt((
        t_slide_track_line,
        t_slide_track_arc,
        t_slide_track_circ_left,
        t_slide_track_circ_right,
        t_slide_track_v,
        t_slide_track_p,
        t_slide_track_q,
        t_slide_track_s,
        t_slide_track_z,
        t_slide_track_pp,
        t_slide_track_qq,
        t_slide_track_angle,
        t_slide_track_spread,
    ))(input)
}

fn t_slide_sep_track(input: Span) -> nom::IResult<Span, SlideTrack> {
    use nom::character::complete::char;

    let (s, _) = multispace0(input)?;
    let (s, _) = char('*')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, track) = t_slide_track(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, track))
}

fn t_slide(input: Span) -> nom::IResult<Span, RawNoteInsn> {
    use nom::multi::many0;

    let (s, _) = multispace0(input)?;
    let (s, start) = t_tap_param(s)?;
    let (s, first_track) = t_slide_track(s)?;
    let (s, rest_track) = many0(t_slide_sep_track)(s)?;
    let (s, _) = multispace0(s)?;

    let tracks = {
        let mut tmp = Vec::with_capacity(rest_track.len() + 1);
        tmp.push(first_track);
        tmp.extend(rest_track);
        tmp
    };

    Ok((s, RawNoteInsn::Slide(SlideParams { start, tracks })))
}

fn t_slide_single(input: Span) -> nom::IResult<Span, RawInsn> {
    let (s, _) = multispace0(input)?;
    let (s, note) = t_slide(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, RawInsn::Note(note)))
}

fn t_bundle_note(input: Span) -> nom::IResult<Span, RawNoteInsn> {
    let (s, _) = multispace0(input)?;
    // NOTE: tap must come last as it can match on the simplest key, blocking holds and slides from parsing
    let (s, note) = nom::branch::alt((t_hold, t_slide, t_tap))(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, note))
}

fn t_bundle_sep_note(input: Span) -> nom::IResult<Span, RawNoteInsn> {
    use nom::character::complete::char;

    let (s, _) = multispace0(input)?;
    let (s, _) = char('/')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, note) = t_bundle_note(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, note))
}

fn t_bundle(input: Span) -> nom::IResult<Span, RawInsn> {
    use nom::multi::many1;

    let (s, _) = multispace0(input)?;
    let (s, first) = t_bundle_note(s)?;
    let (s, _) = multispace0(s)?;
    let (s, rest) = many1(t_bundle_sep_note)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, _) = multispace0(s)?;

    let notes = {
        let mut tmp = Vec::with_capacity(rest.len() + 1);
        tmp.push(first);
        tmp.extend(rest);
        tmp
    };

    Ok((s, RawInsn::NoteBundle(notes)))
}
