use nom::character::complete::multispace0;

use super::*;
use crate::NomSpan;

pub(crate) fn parse_maidata_insns(input: NomSpan) -> nom::IResult<NomSpan, Vec<SpannedRawInsn>> {
    use nom::multi::many0;

    let (s, insns) = many0(parse_one_maidata_insn)(input)?;
    let (s, _) = t_eof(s)?;

    Ok((s, insns))
}

fn t_eof(input: NomSpan) -> nom::IResult<NomSpan, NomSpan> {
    nom::eof!(input,)
}

fn parse_one_maidata_insn(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawInsn> {
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

fn t_end_mark(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawInsn> {
    use nom::character::complete::char;

    let (s, _) = multispace0(input)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = char('E')(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc).into();
    Ok((s, RawInsn::EndMark.with_span(span)))
}

fn t_note_sep(input: NomSpan) -> nom::IResult<NomSpan, ()> {
    use nom::character::complete::char;

    let (s, _) = multispace0(input)?;
    let (s, _) = char(',')(s)?;
    Ok((s, ()))
}

fn t_bpm(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawInsn> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, _) = multispace0(input)?;
    let (s, _) = char('(')(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;
    let (s, bpm) = float(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(')')(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc).into();

    Ok((s, RawInsn::Bpm(BpmParams { new_bpm: bpm }).with_span(span)))
}

fn t_absolute_duration(input: NomSpan) -> nom::IResult<NomSpan, f32> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, _) = char('#')(input)?;
    let (s, _) = multispace0(s)?;
    let (s, dur) = float(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, dur))
}

fn t_beat_divisor_param_int(input: NomSpan) -> nom::IResult<NomSpan, BeatDivisorParams> {
    use nom::character::complete::digit1;

    let (s, divisor_str) = digit1(input)?;
    let (s, _) = multispace0(s)?;

    // TODO: out-of-range conversion failures
    let divisor = divisor_str.fragment().parse().unwrap();

    Ok((s, BeatDivisorParams::NewDivisor(divisor)))
}

fn t_beat_divisor_param_float(input: NomSpan) -> nom::IResult<NomSpan, BeatDivisorParams> {
    let (s, dur) = t_absolute_duration(input)?;
    let (s, _) = multispace0(s)?;

    Ok((s, BeatDivisorParams::NewAbsoluteDuration(dur)))
}

fn t_beat_divisor_param(input: NomSpan) -> nom::IResult<NomSpan, BeatDivisorParams> {
    use nom::branch::alt;

    alt((t_beat_divisor_param_int, t_beat_divisor_param_float))(input)
}

fn t_beat_divisor(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawInsn> {
    use nom::character::complete::char;

    let (s, _) = multispace0(input)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = char('{')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, params) = t_beat_divisor_param(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc).into();
    Ok((s, RawInsn::BeatDivisor(params).with_span(span)))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn t_key(input: NomSpan) -> nom::IResult<NomSpan, Key> {
    use std::convert::TryFrom;
    use nom::combinator::map;
    use nom::character::complete::one_of;

    map(one_of("12345678"), |s| Key::try_from(s).unwrap())(input)
}

fn t_rest(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawInsn> {
    let (s, _) = multispace0(input)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc).into();
    Ok((s, RawInsn::Rest.with_span(span)))
}

fn t_tap_param(input: NomSpan) -> nom::IResult<NomSpan, TapParams> {
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

fn t_tap(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawNoteInsn> {
    let (s, _) = multispace0(input)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, params) = t_tap_param(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc).into();
    Ok((s, RawNoteInsn::Tap(params).with_span(span)))
}

fn t_tap_single(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawInsn> {
    let (s, _) = multispace0(input)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, note) = t_tap(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc).into();
    Ok((s, RawInsn::Note(note).with_span(span)))
}

fn t_tap_multi_simplified_every(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawNoteInsn> {
    let (s, start_loc) = nom_locate::position(input)?;
    let (s, key) = t_key(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    // all taps are regular ones when using simplified notation
    let variant = TapVariant::Tap;

    let span = (start_loc, end_loc).into();
    Ok((
        s,
        RawNoteInsn::Tap(TapParams { variant, key }).with_span(span),
    ))
}

fn t_tap_multi_simplified(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawInsn> {
    use nom::multi::many1;

    let (s, _) = multispace0(input)?;
    let (s, start_loc) = nom_locate::position(s)?;
    // all whitespaces are ignored, including those inside a taps bundle
    // we must parse every key individually (also for getting proper span info)
    let (s, notes) = many1(t_tap_multi_simplified_every)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc).into();
    Ok((s, RawInsn::NoteBundle(notes).with_span(span)))
}

fn t_len_spec_beats(input: NomSpan) -> nom::IResult<NomSpan, Length> {
    use nom::character::complete::char;
    use nom::character::complete::digit1;

    let (s, divisor_str) = digit1(input)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(':')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, num_str) = digit1(s)?;
    let (s, _) = multispace0(s)?;

    // TODO: handle conversion errors
    let divisor = divisor_str.fragment().parse().unwrap();
    let num = num_str.fragment().parse().unwrap();

    Ok((s, Length::NumBeats(NumBeatsParams { divisor, num })))
}

fn t_len_spec_absolute(input: NomSpan) -> nom::IResult<NomSpan, Length> {
    let (s, dur) = t_absolute_duration(input)?;
    let (s, _) = multispace0(s)?;

    Ok((s, Length::Seconds(dur)))
}

fn t_len_spec(input: NomSpan) -> nom::IResult<NomSpan, Length> {
    use nom::branch::alt;

    alt((t_len_spec_beats, t_len_spec_absolute))(input)
}

fn t_len(input: NomSpan) -> nom::IResult<NomSpan, Length> {
    use nom::character::complete::char;

    // TODO: star-time/BPM overrides
    let (s, _) = multispace0(input)?;
    let (s, _) = char('[')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, len) = t_len_spec(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(']')(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, len))
}

fn t_hold(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawNoteInsn> {
    use nom::character::complete::char;

    let (s, _) = multispace0(input)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, key) = t_key(s)?;
    let (s, _) = char('h')(s)?;
    let (s, len) = t_len(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc).into();
    Ok((
        s,
        RawNoteInsn::Hold(HoldParams { key, len }).with_span(span),
    ))
}

fn t_hold_single(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawInsn> {
    let (s, _) = multispace0(input)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, note) = t_hold(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc).into();
    Ok((s, RawInsn::Note(note).with_span(span)))
}

// FxE[len]
// covers everything except FVRE
macro_rules! define_slide_track {
    (@ $fn_name: ident, $recog: expr, $variant: ident) => {
        #[allow(unused_imports)]
        fn $fn_name(input: NomSpan) -> nom::IResult<NomSpan, SlideTrack> {
            use nom::character::complete::char;
            use nom::bytes::complete::tag;

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

    ($fn_name: ident, tag $tag: expr, $variant: ident) => {
        define_slide_track!(@ $fn_name, tag($tag), $variant);
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
define_slide_track!(t_slide_track_pp, tag "pp", Pp);
define_slide_track!(t_slide_track_qq, tag "qq", Qq);
define_slide_track!(t_slide_track_spread, char 'w', Spread);

fn t_slide_track_angle(input: NomSpan) -> nom::IResult<NomSpan, SlideTrack> {
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

fn t_slide_track(input: NomSpan) -> nom::IResult<NomSpan, SlideTrack> {
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

fn t_slide_sep_track(input: NomSpan) -> nom::IResult<NomSpan, SlideTrack> {
    use nom::character::complete::char;

    let (s, _) = multispace0(input)?;
    let (s, _) = char('*')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, track) = t_slide_track(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, track))
}

fn t_slide(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawNoteInsn> {
    use nom::multi::many0;

    let (s, _) = multispace0(input)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, start) = t_tap_param(s)?;
    let (s, first_track) = t_slide_track(s)?;
    let (s, rest_track) = many0(t_slide_sep_track)(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let tracks = {
        let mut tmp = Vec::with_capacity(rest_track.len() + 1);
        tmp.push(first_track);
        tmp.extend(rest_track);
        tmp
    };

    let span = (start_loc, end_loc).into();
    Ok((
        s,
        RawNoteInsn::Slide(SlideParams { start, tracks }).with_span(span),
    ))
}

fn t_slide_single(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawInsn> {
    let (s, _) = multispace0(input)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, note) = t_slide(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc).into();
    Ok((s, RawInsn::Note(note).with_span(span)))
}

fn t_bundle_note(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawNoteInsn> {
    let (s, _) = multispace0(input)?;
    // NOTE: tap must come last as it can match on the simplest key, blocking holds and slides from parsing
    let (s, note) = nom::branch::alt((t_hold, t_slide, t_tap))(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, note))
}

fn t_bundle_sep_note(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawNoteInsn> {
    use nom::character::complete::char;

    let (s, _) = multispace0(input)?;
    let (s, _) = char('/')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, note) = t_bundle_note(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, note))
}

fn t_bundle(input: NomSpan) -> nom::IResult<NomSpan, SpannedRawInsn> {
    use nom::multi::many1;

    let (s, _) = multispace0(input)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, first) = t_bundle_note(s)?;
    let (s, _) = multispace0(s)?;
    let (s, rest) = many1(t_bundle_sep_note)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let notes = {
        let mut tmp = Vec::with_capacity(rest.len() + 1);
        tmp.push(first);
        tmp.extend(rest);
        tmp
    };

    let span = (start_loc, end_loc).into();
    Ok((s, RawInsn::NoteBundle(notes).with_span(span)))
}
