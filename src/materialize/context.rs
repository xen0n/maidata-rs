use super::Note;
use crate::insn;
use crate::materialize::{
    MaterializedHold, MaterializedSlideTrack, MaterializedTap, MaterializedTapShape,
};

pub(crate) struct MaterializationContext {
    // TODO: is slides' default stop time really independent of BPM changes?
    // currently it is dependent -- add a separate non-changing value (initialized by the "wholebpm"
    // thing) to move to independent
    curr_beat_dur: f32,
    curr_note_dur: f32,
    curr_ts: f32,
}

impl MaterializationContext {
    pub(crate) fn with_offset(offset_secs: f32) -> Self {
        Self {
            curr_beat_dur: 0.0,
            curr_note_dur: 0.0,
            curr_ts: offset_secs,
        }
    }

    /// Materialize a list of raw instructions into notes.
    pub(crate) fn materialize_insns<
        'a,
        I: IntoIterator<Item = &'a crate::Spanned<insn::RawInsn>>,
    >(
        &mut self,
        insns: I,
    ) -> Vec<Note> {
        insns
            .into_iter()
            .map(|insn| self.materialize_raw_insn(insn))
            .flatten()
            .collect()
    }

    /// Read in one raw instruction and materialize into note(s) if applicable.
    pub(crate) fn materialize_raw_insn(
        &mut self,
        insn: &crate::Spanned<insn::RawInsn>,
    ) -> Vec<Note> {
        use std::ops::Deref;
        match insn.deref() {
            insn::RawInsn::Bpm(params) => {
                self.set_bpm(params.new_bpm);
                vec![]
            }
            insn::RawInsn::BeatDivisor(params) => {
                self.set_beat_divisor(params.new_divisor);
                vec![]
            }
            insn::RawInsn::Rest => {
                // currently rests don't materialize to anything
                let _ = self.advance_time();
                vec![]
            }
            insn::RawInsn::EndMark => {
                // TODO: make later materialize calls return error?
                vec![]
            }
            insn::RawInsn::Note(raw_note) => {
                let ts = self.advance_time();
                self.materialize_raw_note(ts, raw_note)
            }
            insn::RawInsn::NoteBundle(raw_notes) => {
                let ts = self.advance_time();
                raw_notes
                    .iter()
                    .map(|raw_note| self.materialize_raw_note(ts, raw_note))
                    .flatten()
                    .collect()
            }
        }
    }

    fn set_bpm(&mut self, new_bpm: f32) {
        self.curr_beat_dur = bpm_to_beat_dur(new_bpm);
    }

    fn set_beat_divisor(&mut self, new_divisor: u8) {
        self.curr_note_dur = divide_beat(self.curr_beat_dur, new_divisor);
    }

    /// Advances timestamp by one "note", return the timestamp before advancing (that of the
    /// current note being materialized).
    fn advance_time(&mut self) -> f32 {
        let res = self.curr_ts;
        self.curr_ts += self.curr_note_dur;
        res
    }

    fn materialize_raw_note(&self, ts: f32, raw_note: &insn::RawNoteInsn) -> Vec<Note> {
        match raw_note {
            insn::RawNoteInsn::Tap(params) => {
                let m_params = materialize_tap_params(ts, params, false);
                vec![Note::Tap(m_params)]
            }
            insn::RawNoteInsn::Slide(params) => materialize_slide(ts, self.curr_beat_dur, params),
            insn::RawNoteInsn::Hold(params) => {
                let m_params = materialize_hold_params(ts, self.curr_beat_dur, params);
                vec![Note::Hold(m_params)]
            }
        }
    }
}

fn bpm_to_beat_dur(bpm: f32) -> f32 {
    60.0 / bpm
}

fn divide_beat(beat_dur: f32, beat_divisor: u8) -> f32 {
    beat_dur * 4.0 / (beat_divisor as f32)
}

fn materialize_tap_params(ts: f32, p: &insn::TapParams, is_slide_star: bool) -> MaterializedTap {
    let shape = match (is_slide_star, p.variant) {
        (false, insn::TapVariant::Tap) => MaterializedTapShape::Ring,
        (false, insn::TapVariant::Break) => MaterializedTapShape::Break,
        (true, _) => MaterializedTapShape::Star,
    };

    MaterializedTap {
        ts,
        key: p.key,
        shape,
    }
}

/// slide insn -> `vec![star tap, track, track, ...]`
fn materialize_slide(ts: f32, beat_dur: f32, p: &insn::SlideParams) -> Vec<Note> {
    // star
    let star = Note::Tap(materialize_tap_params(ts, &p.start, true));
    let start_key = p.start.key;

    let tracks = p.tracks.iter().map(|track| {
        Note::SlideTrack(materialize_slide_track_params(
            ts, beat_dur, start_key, track,
        ))
    });

    let mut result = Vec::with_capacity(tracks.len() + 1);
    result.push(star);
    result.extend(tracks);
    result
}

fn materialize_slide_track_params(
    ts: f32,
    beat_dur: f32,
    start_key: insn::Key,
    track: &insn::SlideTrack,
) -> MaterializedSlideTrack {
    let shape = track.shape();
    let params = track.params();

    // in simai, stop time is actually encoded (overridden) in the duration spec of individual
    // slide track
    //
    // TODO: currently overriding stop time is not implemented, hardcoded to 1 beat
    let stop_time = beat_dur;
    let start_ts = ts + stop_time;

    MaterializedSlideTrack {
        ts,
        start_ts,
        dur: materialize_duration(params.len, beat_dur),
        start: start_key,
        destination: params.destination.key,
        interim: params.interim.map(|x| x.key),
        shape,
    }
}

fn materialize_hold_params(ts: f32, beat_dur: f32, p: &insn::HoldParams) -> MaterializedHold {
    MaterializedHold {
        ts,
        dur: materialize_duration(p.len, beat_dur),
        key: p.key,
    }
}

fn materialize_duration(x: insn::Length, beat_dur: f32) -> f32 {
    match x {
        insn::Length::NumBeats(p) => divide_beat(beat_dur, p.divisor) * (p.num as f32),
        insn::Length::Seconds(x) => x,
    }
}
