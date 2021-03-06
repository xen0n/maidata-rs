use nom::character::complete::multispace0;

use super::*;
use crate::{NomSpan, PResult, WithSpan};

pub(crate) fn parse_maidata_insns(s: NomSpan) -> PResult<Vec<SpRawInsn>> {
    use nom::multi::many0;

    let (s, insns) = many0(parse_one_maidata_insn)(s)?;
    let (s, _) = t_eof(s)?;

    Ok((s, insns))
}

fn t_eof(s: NomSpan) -> PResult<NomSpan> {
    nom::eof!(s,)
}

fn parse_one_maidata_insn(s: NomSpan) -> PResult<SpRawInsn> {
    let (s, _) = multispace0(s)?;
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

fn t_end_mark(s: NomSpan) -> PResult<SpRawInsn> {
    use nom::character::complete::char;

    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = char('E')(s)?;
    let (s, end_loc) = nom_locate::position(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::EndMark.with_span(span)))
}

fn t_note_sep(s: NomSpan) -> PResult<()> {
    use nom::character::complete::char;

    let (s, _) = multispace0(s)?;
    let (s, _) = char(',')(s)?;
    Ok((s, ()))
}

fn t_bpm(s: NomSpan) -> PResult<SpRawInsn> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, _) = multispace0(s)?;
    let (s, _) = char('(')(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;
    let (s, bpm) = float(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(')')(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);

    Ok((s, RawInsn::Bpm(BpmParams { new_bpm: bpm }).with_span(span)))
}

fn t_absolute_duration(s: NomSpan) -> PResult<f32> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, _) = char('#')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, dur) = float(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, dur))
}

fn t_beat_divisor_param_int(s: NomSpan) -> PResult<BeatDivisorParams> {
    use nom::character::complete::digit1;

    let (s, divisor_str) = digit1(s)?;
    let (s, _) = multispace0(s)?;

    // TODO: out-of-range conversion failures
    let divisor = divisor_str.fragment().parse().unwrap();

    Ok((s, BeatDivisorParams::NewDivisor(divisor)))
}

fn t_beat_divisor_param_float(s: NomSpan) -> PResult<BeatDivisorParams> {
    let (s, dur) = t_absolute_duration(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, BeatDivisorParams::NewAbsoluteDuration(dur)))
}

fn t_beat_divisor_param(s: NomSpan) -> PResult<BeatDivisorParams> {
    use nom::branch::alt;

    alt((t_beat_divisor_param_int, t_beat_divisor_param_float))(s)
}

fn t_beat_divisor(s: NomSpan) -> PResult<SpRawInsn> {
    use nom::character::complete::char;

    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = char('{')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, params) = t_beat_divisor_param(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('}')(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::BeatDivisor(params).with_span(span)))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn t_key(s: NomSpan) -> PResult<Key> {
    use std::convert::TryFrom;
    use nom::combinator::map;
    use nom::character::complete::one_of;

    map(one_of("12345678"), |s| Key::try_from(s).unwrap())(s)
}

fn t_rest(s: NomSpan) -> PResult<SpRawInsn> {
    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::Rest.with_span(span)))
}

fn t_tap_param(s: NomSpan) -> PResult<TapParams> {
    use nom::character::complete::char;
    use nom::combinator::opt;

    let (s, _) = multispace0(s)?;
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

fn t_tap(s: NomSpan) -> PResult<SpRawNoteInsn> {
    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, params) = t_tap_param(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawNoteInsn::Tap(params).with_span(span)))
}

fn t_tap_single(s: NomSpan) -> PResult<SpRawInsn> {
    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, note) = t_tap(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::Note(note).with_span(span)))
}

fn t_tap_multi_simplified_every(s: NomSpan) -> PResult<SpRawNoteInsn> {
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, key) = t_key(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    // all taps are regular ones when using simplified notation
    let variant = TapVariant::Tap;

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::Tap(TapParams { variant, key }).with_span(span),
    ))
}

fn t_tap_multi_simplified(s: NomSpan) -> PResult<SpRawInsn> {
    use nom::multi::many1;

    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    // all whitespaces are ignored, including those inside a taps bundle
    // we must parse every key individually (also for getting proper span info)
    let (s, notes) = many1(t_tap_multi_simplified_every)(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::NoteBundle(notes).with_span(span)))
}

fn t_len_spec_beats(s: NomSpan) -> PResult<Length> {
    use nom::character::complete::char;
    use nom::character::complete::digit1;

    let (s, divisor_str) = digit1(s)?;
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

fn t_len_spec_absolute(s: NomSpan) -> PResult<Length> {
    let (s, dur) = t_absolute_duration(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, Length::Seconds(dur)))
}

fn t_len_spec(s: NomSpan) -> PResult<Length> {
    use nom::branch::alt;

    alt((t_len_spec_beats, t_len_spec_absolute))(s)
}

fn t_len(s: NomSpan) -> PResult<Length> {
    use nom::character::complete::char;

    // TODO: star-time/BPM overrides
    let (s, _) = multispace0(s)?;
    let (s, _) = char('[')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, len) = t_len_spec(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(']')(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, len))
}

fn t_hold(s: NomSpan) -> PResult<SpRawNoteInsn> {
    use nom::character::complete::char;

    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, key) = t_key(s)?;
    let (s, _) = char('h')(s)?;
    let (s, len) = t_len(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::Hold(HoldParams { key, len }).with_span(span),
    ))
}

fn t_hold_single(s: NomSpan) -> PResult<SpRawInsn> {
    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, note) = t_hold(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::Note(note).with_span(span)))
}

fn t_slide_len_simple(s: NomSpan) -> PResult<SlideLength> {
    let (s, len) = t_len(s)?;

    Ok((s, SlideLength::Simple(len)))
}

// NOTE: must run after t_slide_len_simple
fn t_slide_len_custom(s: NomSpan) -> PResult<SlideLength> {
    use nom::character::complete::char;
    use nom::number::complete::float;

    let (s, _) = multispace0(s)?;
    let (s, _) = char('[')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, x1) = float(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('#')(s)?;
    let (s, len) = t_len_spec(s)?;
    let (s, _) = char(']')(s)?;
    let (s, _) = multispace0(s)?;

    // following cases are possible in this combinator:
    //
    // - `[160#8:3]` -> stop time=(as in BPM 160) len=8:3
    // - `[3##1.5]` -> stop time=(absolute 3s) len=1.5s
    let stop_time_spec = match len {
        Length::NumBeats(_) => SlideStopTimeSpec::Bpm(x1),
        Length::Seconds(_) => SlideStopTimeSpec::Seconds(x1),
    };

    Ok((s, SlideLength::Custom(stop_time_spec, len)))
}

fn t_slide_len(s: NomSpan) -> PResult<SlideLength> {
    use nom::branch::alt;

    // simple variant must come before custom
    alt((t_slide_len_simple, t_slide_len_custom))(s)
}

// FxE[len]
// covers everything except FVRE
macro_rules! define_slide_track {
    (@ $fn_name: ident, $recog: expr, $variant: ident) => {
        #[allow(unused_imports)]
        fn $fn_name(s: NomSpan) -> PResult<SlideTrack> {
            use nom::character::complete::char;
            use nom::bytes::complete::tag;

            let (s, _) = multispace0(s)?;
            let (s, _) = $recog(s)?;
            let (s, _) = multispace0(s)?;
            // TODO: can slide ends be breaks?
            let (s, destination) = t_tap_param(s)?;
            let (s, _) = multispace0(s)?;
            let (s, len) = t_slide_len(s)?;
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

fn t_slide_track_angle(s: NomSpan) -> PResult<SlideTrack> {
    use nom::character::complete::char;

    let (s, _) = multispace0(s)?;
    let (s, _) = char('V')(s)?;
    let (s, _) = multispace0(s)?;
    // TODO: can these two be breaks?
    let (s, interim) = t_tap_param(s)?;
    let (s, _) = multispace0(s)?;
    let (s, destination) = t_tap_param(s)?;
    let (s, _) = multispace0(s)?;
    let (s, len) = t_slide_len(s)?;
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

fn t_slide_track(s: NomSpan) -> PResult<SlideTrack> {
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
    ))(s)
}

fn t_slide_sep_track(s: NomSpan) -> PResult<SlideTrack> {
    use nom::character::complete::char;

    let (s, _) = multispace0(s)?;
    let (s, _) = char('*')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, track) = t_slide_track(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, track))
}

fn t_slide(s: NomSpan) -> PResult<SpRawNoteInsn> {
    use nom::multi::many0;

    let (s, _) = multispace0(s)?;
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

    let span = (start_loc, end_loc);
    Ok((
        s,
        RawNoteInsn::Slide(SlideParams { start, tracks }).with_span(span),
    ))
}

fn t_slide_single(s: NomSpan) -> PResult<SpRawInsn> {
    let (s, _) = multispace0(s)?;
    let (s, start_loc) = nom_locate::position(s)?;
    let (s, note) = t_slide(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = t_note_sep(s)?;
    let (s, end_loc) = nom_locate::position(s)?;
    let (s, _) = multispace0(s)?;

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::Note(note).with_span(span)))
}

fn t_bundle_note(s: NomSpan) -> PResult<SpRawNoteInsn> {
    let (s, _) = multispace0(s)?;
    // NOTE: tap must come last as it can match on the simplest key, blocking holds and slides from parsing
    let (s, note) = nom::branch::alt((t_hold, t_slide, t_tap))(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, note))
}

fn t_bundle_sep_note(s: NomSpan) -> PResult<SpRawNoteInsn> {
    use nom::character::complete::char;

    let (s, _) = multispace0(s)?;
    let (s, _) = char('/')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, note) = t_bundle_note(s)?;
    let (s, _) = multispace0(s)?;

    Ok((s, note))
}

fn t_bundle(s: NomSpan) -> PResult<SpRawInsn> {
    use nom::multi::many1;

    let (s, _) = multispace0(s)?;
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

    let span = (start_loc, end_loc);
    Ok((s, RawInsn::NoteBundle(notes).with_span(span)))
}
