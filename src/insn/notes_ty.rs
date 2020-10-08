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

#[derive(Clone, Debug)]
pub enum KeyParseError {
    InvalidKey(char),
}

impl std::convert::TryFrom<char> for Key {
    type Error = KeyParseError;

    fn try_from(x: char) -> Result<Self, Self::Error> {
        match x {
            '1' => Ok(Self::K1),
            '2' => Ok(Self::K2),
            '3' => Ok(Self::K3),
            '4' => Ok(Self::K4),
            '5' => Ok(Self::K5),
            '6' => Ok(Self::K6),
            '7' => Ok(Self::K7),
            '8' => Ok(Self::K8),
            _ => Err(KeyParseError::InvalidKey(x)),
        }
    }
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Length {
    NumBeats(NumBeatsParams),
    Seconds(f32),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NumBeatsParams {
    pub divisor: u8,
    pub num: u8,
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HoldParams {
    pub key: Key,
    pub len: Length,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SlideParams {
    pub start: TapParams,
    pub tracks: Vec<SlideTrack>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SlideTrack {
    Line(SlideTrackParams),
    Arc(SlideTrackParams), // ???
    CircumferenceLeft(SlideTrackParams),
    CircumferenceRight(SlideTrackParams),
    V(SlideTrackParams),
    P(SlideTrackParams),
    Q(SlideTrackParams),
    S(SlideTrackParams),
    Z(SlideTrackParams),
    Pp(SlideTrackParams),
    Qq(SlideTrackParams),
    Angle(SlideTrackParams),
    Spread(SlideTrackParams),
}

impl SlideTrack {
    pub fn shape(&self) -> SlideShape {
        match self {
            Self::Line(_) => SlideShape::Line,
            Self::Arc(_) => SlideShape::Arc,
            Self::CircumferenceLeft(_) => SlideShape::CircumferenceLeft,
            Self::CircumferenceRight(_) => SlideShape::CircumferenceRight,
            Self::V(_) => SlideShape::V,
            Self::P(_) => SlideShape::P,
            Self::Q(_) => SlideShape::Q,
            Self::S(_) => SlideShape::S,
            Self::Z(_) => SlideShape::Z,
            Self::Pp(_) => SlideShape::Pp,
            Self::Qq(_) => SlideShape::Qq,
            Self::Angle(_) => SlideShape::Angle,
            Self::Spread(_) => SlideShape::Spread,
        }
    }

    pub fn params(&self) -> &SlideTrackParams {
        match self {
            SlideTrack::Line(p) => p,
            SlideTrack::Arc(p) => p,
            SlideTrack::CircumferenceLeft(p) => p,
            SlideTrack::CircumferenceRight(p) => p,
            SlideTrack::V(p) => p,
            SlideTrack::P(p) => p,
            SlideTrack::Q(p) => p,
            SlideTrack::S(p) => p,
            SlideTrack::Z(p) => p,
            SlideTrack::Pp(p) => p,
            SlideTrack::Qq(p) => p,
            SlideTrack::Angle(p) => p,
            SlideTrack::Spread(p) => p,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SlideShape {
    Line,
    Arc,
    CircumferenceLeft,
    CircumferenceRight,
    V,
    P,
    Q,
    S,
    Z,
    Pp,
    Qq,
    Angle,
    Spread,
}

impl From<SlideTrack> for SlideShape {
    fn from(x: SlideTrack) -> Self {
        x.shape()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SlideTrackParams {
    pub destination: TapParams,
    pub interim: Option<TapParams>,
    pub len: Length,
}
