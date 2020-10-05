mod directives_ty;
mod notes_ty;

pub use directives_ty::*;
pub use notes_ty::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Insn {
    Bpm(BpmParams),
    BeatDivisor(BeatDivisorParams),
    Rest(RestParams),
    Tap(TapParams),
    Hold(HoldParams),
    Slide(SlideParams),
}
