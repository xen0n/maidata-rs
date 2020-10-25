mod directives_ty;
mod notes_ty;
mod parser;

pub use directives_ty::*;
pub use notes_ty::*;
pub(crate) use parser::parse_maidata_insns;

#[derive(Clone, PartialEq, Debug)]
pub enum RawNoteInsn {
    Tap(TapParams),
    Hold(HoldParams),
    Slide(SlideParams),
}

#[derive(Clone, PartialEq, Debug)]
pub enum RawInsn {
    Bpm(BpmParams),
    BeatDivisor(BeatDivisorParams),
    Rest,
    Note(SpRawNoteInsn),
    NoteBundle(crate::VecSp<RawNoteInsn>),
    EndMark,
}

pub type SpRawInsn = crate::Sp<RawInsn>;
pub type SpRawNoteInsn = crate::Sp<RawNoteInsn>;
