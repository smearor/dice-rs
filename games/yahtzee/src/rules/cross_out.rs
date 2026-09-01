use crate::models::category::ScoreCategory;
use crate::models::dice_set::DiceSet;
use crate::models::score::Score;
use crate::models::scorecard::Scorecard;
use crate::rules::scoring::calculate_score;

/// A recommendation for which category to cross out, with reasoning.
///
/// Produced by `CrossOutAdvisor` when no valid category can be scored.
/// The advisor ranks empty categories by their potential score (lowest first),
/// so crossing out the least valuable one minimizes lost points.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossOutRecommendation {
    /// The category recommended to cross out.
    category: ScoreCategory,
    /// The potential score that would be lost by crossing out this category.
    lost_potential: Score,
}

impl CrossOutRecommendation {
    /// Get the recommended category to cross out.
    pub fn category(&self) -> ScoreCategory {
        self.category
    }

    /// Get the potential score lost by crossing out this category.
    pub fn lost_potential(&self) -> Score {
        self.lost_potential
    }
}

/// The cross-out advisor helps a player choose the least-damaging category
/// to cross out when no category can be scored with the current dice.
///
/// It ranks all empty categories by their potential score with the current
/// dice values, recommending the one with the lowest potential score.
/// This minimizes the expected point loss from crossing out.
#[derive(Debug, Clone, Copy)]
pub struct CrossOutAdvisor;

impl CrossOutAdvisor {
    /// Create a new cross-out advisor.
    pub fn new() -> Self {
        Self
    }

    /// Recommend the best category to cross out for the given scorecard
    /// and dice values.
    ///
    /// Returns `None` if there are no empty categories (scorecard is complete)
    /// or if at least one category can still be scored positively (no need
    /// to cross out).
    pub fn recommend(&self, scorecard: &Scorecard, dice: &DiceSet) -> Option<CrossOutRecommendation> {
        let empty_categories = self.empty_categories(scorecard);
        if empty_categories.is_empty() {
            return None;
        }

        // If any empty category can score positively, don't recommend crossing out
        if empty_categories.iter().any(|cat| calculate_score(*cat, dice).get() > 0) {
            return None;
        }

        // All empty categories score zero — rank by lowest potential score
        // (they're all zero, so pick the one with the lowest maximum possible score)
        self.least_valuable_category(&empty_categories, dice)
    }

    /// Recommend a category to cross out even if some categories could score.
    /// This is used when the player chooses to cross out voluntarily.
    ///
    /// Ranks empty categories by their potential score with the current dice,
    /// recommending the one with the lowest potential score.
    pub fn recommend_voluntary(&self, scorecard: &Scorecard, dice: &DiceSet) -> Option<CrossOutRecommendation> {
        let empty_categories = self.empty_categories(scorecard);
        if empty_categories.is_empty() {
            return None;
        }
        self.least_valuable_category(&empty_categories, dice)
    }

    /// Get all empty (unfilled) categories from the scorecard.
    fn empty_categories(&self, scorecard: &Scorecard) -> Vec<ScoreCategory> {
        ScoreCategory::ALL.iter().filter(|cat| !scorecard.entry(**cat).is_used()).copied().collect()
    }

    /// Find the category with the lowest potential score.
    ///
    /// Ties are broken by category priority: lower-section categories
    /// (Yahtzee, LargeStraight) are considered more valuable than
    /// upper-section ones, so upper-section categories are preferred
    /// for crossing out.
    fn least_valuable_category(&self, categories: &[ScoreCategory], dice: &DiceSet) -> Option<CrossOutRecommendation> {
        categories
            .iter()
            .map(|cat| {
                let potential = calculate_score(*cat, dice);
                let priority = self.category_priority(*cat);
                (*cat, potential, priority)
            })
            .min_by_key(|(_, potential, priority)| (*potential, *priority))
            .map(|(category, lost_potential, _)| CrossOutRecommendation { category, lost_potential })
    }

    /// Assign a priority value to a category for cross-out tie-breaking.
    ///
    /// Lower values = less valuable = preferred for crossing out.
    /// Upper section categories are less valuable than lower section ones.
    fn category_priority(&self, category: ScoreCategory) -> u8 {
        match category {
            ScoreCategory::Chance => 0,
            ScoreCategory::Ones => 1,
            ScoreCategory::Twos => 2,
            ScoreCategory::Threes => 3,
            ScoreCategory::Fours => 4,
            ScoreCategory::Fives => 5,
            ScoreCategory::Sixes => 6,
            ScoreCategory::ThreeOfAKind => 7,
            ScoreCategory::FourOfAKind => 8,
            ScoreCategory::FullHouse => 9,
            ScoreCategory::SmallStraight => 10,
            ScoreCategory::LargeStraight => 11,
            ScoreCategory::Yahtzee => 12,
        }
    }
}

impl Default for CrossOutAdvisor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::player::Player;
    use crate::models::player_color::PlayerColor;
    use crate::models::player_name::PlayerName;
    use crate::models::player_type::PlayerType;

    fn make_player() -> Player {
        Player::new(PlayerName::new("Test").unwrap(), PlayerColor::RED, PlayerType::Human)
    }

    fn dice(values: [u8; 5]) -> DiceSet {
        DiceSet::from_values(values).unwrap()
    }

    #[test]
    fn recommend_none_when_scorecard_complete() {
        let mut player = make_player();
        let dice = dice([1, 2, 3, 4, 5]);
        for cat in ScoreCategory::ALL {
            player.scorecard_mut().enter(cat, Score::new(10)).unwrap();
        }
        let advisor = CrossOutAdvisor::new();
        assert!(advisor.recommend(player.scorecard(), &dice).is_none());
    }

    #[test]
    fn recommend_none_when_valid_category_exists() {
        let player = make_player();
        let dice = dice([3, 3, 3, 3, 6]);
        let advisor = CrossOutAdvisor::new();
        // ThreeOfAKind and FourOfAKind can score — no need to cross out
        assert!(advisor.recommend(player.scorecard(), &dice).is_none());
    }

    #[test]
    fn recommend_some_when_all_empty_score_zero() {
        let mut player = make_player();
        // Fill all categories that could score with [1, 2, 3, 5, 6]
        let dice = dice([1, 2, 3, 5, 6]);
        // Chance always scores, so fill it first
        player.scorecard_mut().enter(ScoreCategory::Chance, Score::new(17)).unwrap();
        // Now no empty category can score positively with this dice
        // (Ones=1, Twos=2, Threes=3, Fours=0, Fives=5, Sixes=6, 3K=0, 4K=0, FH=0, SS=0, LS=0, Y=0)
        // Actually Ones, Twos, Threes, Fives, Sixes all score > 0
        // We need to fill ALL categories that score > 0
        player.scorecard_mut().enter(ScoreCategory::Ones, Score::new(1)).unwrap();
        player.scorecard_mut().enter(ScoreCategory::Twos, Score::new(2)).unwrap();
        player.scorecard_mut().enter(ScoreCategory::Threes, Score::new(3)).unwrap();
        player.scorecard_mut().enter(ScoreCategory::Fives, Score::new(5)).unwrap();
        player.scorecard_mut().enter(ScoreCategory::Sixes, Score::new(6)).unwrap();
        let advisor = CrossOutAdvisor::new();
        let rec = advisor.recommend(player.scorecard(), &dice);
        assert!(rec.is_some());
        // Should recommend the least valuable remaining category
        let rec = rec.unwrap();
        assert!(!rec.category().eq(&ScoreCategory::Chance));
    }

    #[test]
    fn recommend_voluntary_returns_least_valuable() {
        let player = make_player();
        let dice = dice([1, 1, 1, 1, 1]);
        let advisor = CrossOutAdvisor::new();
        let rec = advisor.recommend_voluntary(player.scorecard(), &dice);
        assert!(rec.is_some());
        // With all ones, Chance scores 5, Yahtzee scores 50
        // Least valuable should be something with low score
        // Actually all upper section categories score: Ones=5, Twos=0, Threes=0...
        // Twos, Threes, Fours, Fives, Sixes all score 0
        // Among those, priority says Twos (2) is least valuable
        let rec = rec.unwrap();
        assert_eq!(rec.category(), ScoreCategory::Twos);
    }

    #[test]
    fn recommend_voluntary_none_when_complete() {
        let mut player = make_player();
        for cat in ScoreCategory::ALL {
            player.scorecard_mut().enter(cat, Score::new(10)).unwrap();
        }
        let dice = dice([1, 2, 3, 4, 5]);
        let advisor = CrossOutAdvisor::new();
        assert!(advisor.recommend_voluntary(player.scorecard(), &dice).is_none());
    }

    #[test]
    fn recommend_prefers_upper_section_on_tie() {
        let player = make_player();
        // Dice that score zero for many categories
        let dice = dice([2, 3, 4, 5, 6]);
        let advisor = CrossOutAdvisor::new();
        let rec = advisor.recommend_voluntary(player.scorecard(), &dice);
        assert!(rec.is_some());
        // With [2,3,4,5,6]: Ones=0, Twos=2, Threes=3, Fours=4, Fives=5, Sixes=6
        // Ones scores 0, so it should be recommended (lowest score + low priority)
        let rec = rec.unwrap();
        assert_eq!(rec.category(), ScoreCategory::Ones);
        assert_eq!(rec.lost_potential(), Score::ZERO);
    }

    #[test]
    fn empty_categories_excludes_filled() {
        let mut player = make_player();
        player.scorecard_mut().enter(ScoreCategory::Ones, Score::new(3)).unwrap();
        let dice = dice([1, 1, 1, 1, 1]);
        let advisor = CrossOutAdvisor::new();
        let rec = advisor.recommend_voluntary(player.scorecard(), &dice);
        assert!(rec.is_some());
        let rec = rec.unwrap();
        // Ones is filled, so it should not be recommended
        assert_ne!(rec.category(), ScoreCategory::Ones);
    }
}
