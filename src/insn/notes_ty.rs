#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Key {
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TouchSensor {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    C,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Length {
    NumBeats(NumBeatsParams),
    Seconds(f32),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NumBeatsParams {
    divisor: Option<u8>,
    num: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RestParams {
    pub len: Length,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TapParams {
    pub variant: TapVariant,
    pub key: Key,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TapVariant {
    Tap,
    Break,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct HoldParams {
    pub key: Key,
    pub len: Length,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SlideParams {
    pub start: Key,
    pub end: Key,
    pub stop_time: Option<Length>,
    pub len: Length,
    // TODO: shape
}
