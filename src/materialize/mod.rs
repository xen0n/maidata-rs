mod context;

use crate::insn::{Key, SlideShape};

pub type TimestampInSeconds = f32;

pub type DurationInSeconds = f32;

pub enum Note {
    Tap(MaterializedTap),
    Hold(MaterializedHold),
    SlideTrack(MaterializedSlideTrack),
}

pub struct MaterializedTap {
    pub ts: TimestampInSeconds,
    pub key: Key,
}

pub struct MaterializedHold {
    pub ts: TimestampInSeconds,
    pub dur: DurationInSeconds,
    pub key: Key,
}

pub struct MaterializedSlideTrack {
    pub ts: TimestampInSeconds,
    pub stop_dur: DurationInSeconds,
    pub dur: DurationInSeconds,
    pub shape: SlideShape,
}
