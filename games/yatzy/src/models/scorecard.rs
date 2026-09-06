use crate::error::Result;
use crate::error::YatzyError;
use crate::models::category::ScoreCategory;
use crate::models::category_section::CategorySection;
use crate::models::score::Score;
use crate::models::score_entry::ScoreEntry;
use serde::Deserialize;
use serde::Serialize;

/// A player's scorecard tracking all 13 Kniffel categories.
///
/// Each category can be in one of three states: empty, filled with a
/// score, or crossed out. The scorecard also tracks Yatzy bonus
/// points awarded for additional Yatzys rolled after the Yatzy
/// category has been filled with 50 points.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scorecard {
    /// The 13 category entries, indexed by `ScoreCategory::index()`.
    entries: [ScoreEntry; 13],
    /// Bonus points for additional Yatzys (100 per additional Yatzy).
    yatzy_bonus: Score,
}

impl Scorecard {
    /// Create a new empty scorecard.
    pub fn new() -> Self {
        Self {
            entries: [ScoreEntry::Empty; 13],
            yatzy_bonus: Score::ZERO,
        }
    }

    /// Enter a score into a category.
    ///
    /// Returns an error if the category is already filled or crossed out.
    pub fn enter(&mut self, category: ScoreCategory, score: Score) -> Result<()> {
        let idx = category.index();
        if self.entries[idx].is_used() {
            return Err(YatzyError::CategoryAlreadyFilled(category));
        }
        self.entries[idx] = ScoreEntry::Filled(score);
        Ok(())
    }

    /// Cross out a category (score 0).
    ///
    /// Returns an error if the category is already filled or crossed out.
    pub fn cross_out(&mut self, category: ScoreCategory) -> Result<()> {
        let idx = category.index();
        if self.entries[idx].is_used() {
            return Err(YatzyError::CategoryAlreadyFilled(category));
        }
        self.entries[idx] = ScoreEntry::CrossedOut;
        Ok(())
    }

    /// Get the entry for a category.
    pub fn entry(&self, category: ScoreCategory) -> ScoreEntry {
        self.entries[category.index()]
    }

    /// Get the score for a category, or `Score::ZERO` if empty or crossed out.
    pub fn score(&self, category: ScoreCategory) -> Score {
        self.entries[category.index()].score()
    }

    /// Returns true if the category has been filled or crossed out.
    pub fn is_filled(&self, category: ScoreCategory) -> bool {
        self.entries[category.index()].is_used()
    }

    /// Returns a list of all empty (available) categories.
    pub fn empty_categories(&self) -> Vec<ScoreCategory> {
        ScoreCategory::ALL.iter().filter(|c| self.entries[c.index()].is_empty()).copied().collect()
    }

    /// Upper section subtotal (sum of ones through sixes, without bonus).
    pub fn upper_subtotal(&self) -> Score {
        ScoreCategory::ALL
            .iter()
            .filter(|c| c.section() == CategorySection::Upper)
            .map(|c| self.score(*c))
            .fold(Score::ZERO, |acc, s| acc + s)
    }

    /// Upper section bonus (35 points if subtotal >= 63).
    pub fn upper_bonus(&self) -> Score {
        if self.upper_subtotal().get() >= CategorySection::UPPER_BONUS_THRESHOLD {
            Score::new(CategorySection::UPPER_BONUS_POINTS)
        } else {
            Score::ZERO
        }
    }

    /// Lower section subtotal (sum of all lower section categories).
    pub fn lower_subtotal(&self) -> Score {
        ScoreCategory::ALL
            .iter()
            .filter(|c| c.section() == CategorySection::Lower)
            .map(|c| self.score(*c))
            .fold(Score::ZERO, |acc, s| acc + s)
    }

    /// Yatzy bonus points (100 per additional Yatzy).
    pub fn yatzy_bonus(&self) -> Score {
        self.yatzy_bonus
    }

    /// Add a Yatzy bonus (100 points). Called when a second Yatzy
    /// is rolled and the Yatzy category is already filled with 50.
    pub fn add_yatzy_bonus(&mut self) {
        self.yatzy_bonus += Score::new(100);
    }

    /// Grand total including upper subtotal, upper bonus, lower subtotal,
    /// and Yatzy bonus.
    pub fn grand_total(&self) -> Score {
        self.upper_subtotal() + self.upper_bonus() + self.lower_subtotal() + self.yatzy_bonus()
    }

    /// Check if all 13 categories are filled (game over for this player).
    pub fn is_complete(&self) -> bool {
        self.entries.iter().all(|e| e.is_used())
    }

    /// Points still needed to reach the upper section bonus threshold.
    /// Returns 0 if the threshold has been reached.
    pub fn upper_bonus_remaining(&self) -> Score {
        let subtotal = self.upper_subtotal().get();
        if subtotal >= CategorySection::UPPER_BONUS_THRESHOLD {
            Score::ZERO
        } else {
            Score::new(CategorySection::UPPER_BONUS_THRESHOLD - subtotal)
        }
    }
}

impl Default for Scorecard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_scorecard_is_empty() {
        let sc = Scorecard::new();
        for cat in ScoreCategory::ALL {
            assert!(!sc.is_filled(cat));
            assert_eq!(sc.score(cat), Score::ZERO);
        }
        assert!(!sc.is_complete());
    }

    #[test]
    fn enter_score() {
        let mut sc = Scorecard::new();
        sc.enter(ScoreCategory::Ones, Score::new(3)).unwrap();
        assert!(sc.is_filled(ScoreCategory::Ones));
        assert_eq!(sc.score(ScoreCategory::Ones), Score::new(3));
    }

    #[test]
    fn enter_already_filled_fails() {
        let mut sc = Scorecard::new();
        sc.enter(ScoreCategory::Ones, Score::new(3)).unwrap();
        assert!(sc.enter(ScoreCategory::Ones, Score::new(2)).is_err());
    }

    #[test]
    fn cross_out() {
        let mut sc = Scorecard::new();
        sc.cross_out(ScoreCategory::Yatzy).unwrap();
        assert!(sc.is_filled(ScoreCategory::Yatzy));
        assert_eq!(sc.score(ScoreCategory::Yatzy), Score::ZERO);
    }

    #[test]
    fn cross_out_already_filled_fails() {
        let mut sc = Scorecard::new();
        sc.enter(ScoreCategory::Yatzy, Score::new(50)).unwrap();
        assert!(sc.cross_out(ScoreCategory::Yatzy).is_err());
    }

    #[test]
    fn upper_subtotal() {
        let mut sc = Scorecard::new();
        sc.enter(ScoreCategory::Ones, Score::new(3)).unwrap();
        sc.enter(ScoreCategory::Sixes, Score::new(18)).unwrap();
        assert_eq!(sc.upper_subtotal(), Score::new(21));
    }

    #[test]
    fn upper_bonus_achieved() {
        let mut sc = Scorecard::new();
        sc.enter(ScoreCategory::Ones, Score::new(5)).unwrap();
        sc.enter(ScoreCategory::Twos, Score::new(10)).unwrap();
        sc.enter(ScoreCategory::Threes, Score::new(15)).unwrap();
        sc.enter(ScoreCategory::Fours, Score::new(20)).unwrap();
        sc.enter(ScoreCategory::Fives, Score::new(15)).unwrap();
        // Total: 65 >= 63
        assert_eq!(sc.upper_subtotal().get(), 65);
        assert_eq!(sc.upper_bonus(), Score::new(35));
        assert_eq!(sc.upper_bonus_remaining(), Score::ZERO);
    }

    #[test]
    fn upper_bonus_not_achieved() {
        let mut sc = Scorecard::new();
        sc.enter(ScoreCategory::Ones, Score::new(3)).unwrap();
        assert_eq!(sc.upper_bonus(), Score::ZERO);
        assert_eq!(sc.upper_bonus_remaining(), Score::new(60));
    }

    #[test]
    fn lower_subtotal() {
        let mut sc = Scorecard::new();
        sc.enter(ScoreCategory::FullHouse, Score::new(25)).unwrap();
        sc.enter(ScoreCategory::Chance, Score::new(20)).unwrap();
        assert_eq!(sc.lower_subtotal(), Score::new(45));
    }

    #[test]
    fn grand_total() {
        let mut sc = Scorecard::new();
        sc.enter(ScoreCategory::Ones, Score::new(5)).unwrap();
        sc.enter(ScoreCategory::Twos, Score::new(10)).unwrap();
        sc.enter(ScoreCategory::Threes, Score::new(15)).unwrap();
        sc.enter(ScoreCategory::Fours, Score::new(20)).unwrap();
        sc.enter(ScoreCategory::Fives, Score::new(15)).unwrap();
        // Upper: 65 + 35 bonus = 100
        sc.enter(ScoreCategory::Yatzy, Score::new(50)).unwrap();
        sc.enter(ScoreCategory::Chance, Score::new(20)).unwrap();
        // Lower: 70
        sc.add_yatzy_bonus();
        // Yatzy bonus: 100
        // Total: 100 + 70 + 100 = 270
        assert_eq!(sc.grand_total(), Score::new(270));
    }

    #[test]
    fn is_complete() {
        let mut sc = Scorecard::new();
        for cat in ScoreCategory::ALL {
            sc.enter(cat, Score::new(10)).unwrap();
        }
        assert!(sc.is_complete());
    }

    #[test]
    fn empty_categories() {
        let mut sc = Scorecard::new();
        sc.enter(ScoreCategory::Ones, Score::new(3)).unwrap();
        let empty = sc.empty_categories();
        assert_eq!(empty.len(), 12);
        assert!(!empty.contains(&ScoreCategory::Ones));
    }

    #[test]
    fn yatzy_bonus() {
        let mut sc = Scorecard::new();
        sc.enter(ScoreCategory::Yatzy, Score::new(50)).unwrap();
        sc.add_yatzy_bonus();
        sc.add_yatzy_bonus();
        assert_eq!(sc.yatzy_bonus(), Score::new(200));
    }
}
