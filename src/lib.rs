pub mod container;
pub mod insn;
pub mod materialize;
mod span;

pub use span::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Difficulty {
    /// The EASY difficulty.
    Easy = 1,
    /// The BASIC difficulty.
    Basic = 2,
    /// The ADVANCED difficulty.
    Advanced = 3,
    /// The EXPERT difficulty.
    Expert = 4,
    /// The MASTER difficulty.
    Master = 5,
    /// The Re:MASTER difficulty.
    ReMaster = 6,
    /// The ORIGINAL difficulty, previously called mai:EDIT in 2simai.
    Original = 7,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Level {
    /// The "Lv.X" form.
    Normal(u8),
    /// The "Lv.X+" form.
    Plus(u8),
    /// The special "Lv.<any char>" form.
    Char(char),
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Level::*;
        match self {
            Normal(lv) => write!(f, "{}", lv)?,
            Plus(lv) => write!(f, "{}+", lv)?,
            Char(lv) => write!(f, "{}", lv)?,
        }

        Ok(())
    }
}
