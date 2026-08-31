use serde::Deserialize;
use serde::Serialize;

/// A score value in the Yahtzee game.
///
/// A newtype wrapper around `u32` that represents points scored in a
/// category or as a total.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Score(u32);

impl Score {
    /// Create a score from a raw value.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// A score of zero (used for crossed-out categories).
    pub const ZERO: Self = Self(0);

    /// Get the numeric value.
    pub fn get(self) -> u32 {
        self.0
    }
}

impl From<u32> for Score {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::ops::Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_get() {
        assert_eq!(Score::new(42).get(), 42);
    }

    #[test]
    fn zero() {
        assert_eq!(Score::ZERO.get(), 0);
    }

    #[test]
    fn add() {
        assert_eq!(Score::new(10) + Score::new(20), Score::new(30));
    }

    #[test]
    fn add_assign() {
        let mut s = Score::new(10);
        s += Score::new(5);
        assert_eq!(s, Score::new(15));
    }

    #[test]
    fn from_u32() {
        let s: Score = 99u32.into();
        assert_eq!(s.get(), 99);
    }

    #[test]
    fn display() {
        assert_eq!(Score::new(25).to_string(), "25");
    }
}
