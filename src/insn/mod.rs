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
    Note(crate::Spanned<RawNoteInsn>),
    NoteBundle(Vec<crate::Spanned<RawNoteInsn>>),
    EndMark,
}

impl RawInsn {
    pub fn with_span(self, span: crate::Span) -> crate::Spanned<Self> {
        crate::Spanned::new(self, span)
    }
}

impl RawNoteInsn {
    pub fn with_span(self, span: crate::Span) -> crate::Spanned<Self> {
        crate::Spanned::new(self, span)
    }
}

pub type SpannedRawInsn = crate::Spanned<RawInsn>;
pub type SpannedRawNoteInsn = crate::Spanned<RawNoteInsn>;
