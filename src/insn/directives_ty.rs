#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BpmParams {
    pub new_bpm: f32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BeatDivisorParams {
    pub new_divisor: u8,
}
