use super::Note;
use crate::insn;

pub(crate) struct MaterializationContext {
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

    /// Read in one raw instruction and materialize into note(s) if applicable.
    pub(crate) fn materialize_raw_insn(&mut self, insn: &insn::RawInsn) -> Vec<Note> {
        match insn {
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
            insn::RawNoteInsn::Tap(params) => todo!(),
            insn::RawNoteInsn::Slide(params) => todo!(),
            insn::RawNoteInsn::Hold(params) => todo!(),
        }
    }
}

fn bpm_to_beat_dur(bpm: f32) -> f32 {
    60.0 / bpm
}

fn divide_beat(beat_dur: f32, beat_divisor: u8) -> f32 {
    beat_dur * 4.0 / (beat_divisor as f32)
}
