mod context;

pub use context::*;

use crate::insn::{Key, SlideShape};

pub type TimestampInSeconds = f32;

pub type DurationInSeconds = f32;

#[derive(Copy, Clone, Debug)]
pub enum Note {
    Tap(MaterializedTap),
    Hold(MaterializedHold),
    SlideTrack(MaterializedSlideTrack),
}

#[derive(Copy, Clone, Debug)]
pub struct MaterializedTap {
    pub ts: TimestampInSeconds,
    pub key: Key,
    pub shape: MaterializedTapShape,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MaterializedTapShape {
    Ring,
    Break,
    Star,
}

#[derive(Copy, Clone, Debug)]
pub struct MaterializedHold {
    pub ts: TimestampInSeconds,
    pub dur: DurationInSeconds,
    pub key: Key,
}

#[derive(Copy, Clone, Debug)]
pub struct MaterializedSlideTrack {
    pub ts: TimestampInSeconds,
    pub start_ts: TimestampInSeconds,
    pub dur: DurationInSeconds,
    pub start: Key,
    pub destination: Key,
    pub interim: Option<Key>,
    pub shape: SlideShape,
}
