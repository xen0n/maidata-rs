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

impl RawInsn {
    pub fn with_span(self, span: crate::Span) -> crate::Sp<Self> {
        crate::Sp::new(self, span)
    }
}

impl RawNoteInsn {
    pub fn with_span(self, span: crate::Span) -> crate::Sp<Self> {
        crate::Sp::new(self, span)
    }
}

pub type SpRawInsn = crate::Sp<RawInsn>;
pub type SpRawNoteInsn = crate::Sp<RawNoteInsn>;
