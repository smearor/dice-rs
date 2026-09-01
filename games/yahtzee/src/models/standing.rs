use crate::models::player::Player;
use crate::models::score::Score;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// The ranking position of a player in the final standings.
///
/// Rank 1 is the winner. Ties produce the same rank number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WinnerRank(u8);

impl WinnerRank {
    /// Create a new winner rank. Returns `None` if rank is 0.
    pub fn new(rank: u8) -> Option<Self> {
        if rank == 0 { None } else { Some(Self(rank)) }
    }

    /// Get the rank value (1-based).
    pub fn get(&self) -> u8 {
        self.0
    }

    /// Whether this is the first place (winner).
    pub fn is_first(&self) -> bool {
        self.0 == 1
    }
}

impl Display for WinnerRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            1 => write!(f, "1. Platz"),
            2 => write!(f, "2. Platz"),
            3 => write!(f, "3. Platz"),
            n => write!(f, "{n}. Platz"),
        }
    }
}

/// A player's entry in the final standings.
///
/// Combines the player, their final score, and their rank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandingEntry {
    /// The player.
    player: Player,
    /// The player's final grand total score.
    final_score: Score,
    /// The player's rank in the standings.
    rank: WinnerRank,
}

impl StandingEntry {
    /// Create a new standing entry.
    pub fn new(player: Player, final_score: Score, rank: WinnerRank) -> Self {
        Self { player, final_score, rank }
    }

    /// Get the player.
    pub fn player(&self) -> &Player {
        &self.player
    }

    /// Get the final score.
    pub fn final_score(&self) -> Score {
        self.final_score
    }

    /// Get the rank.
    pub fn rank(&self) -> WinnerRank {
        self.rank
    }
}

/// Compute the final standings from a list of players.
///
/// Players are ranked by their grand total score in descending order.
/// Ties produce the same rank number (standard competition ranking).
pub fn compute_standings(players: Vec<Player>) -> Vec<StandingEntry> {
    let mut entries: Vec<(Player, Score)> = players
        .into_iter()
        .map(|p| {
            let score = p.grand_total();
            (p, score)
        })
        .collect();

    // Sort by score descending
    entries.sort_by_key(|b| std::cmp::Reverse(b.1));

    let mut result = Vec::new();
    let mut current_rank: u8 = 1;
    let mut current_score: Option<Score> = None;
    let mut players_at_rank: u8 = 0;

    for (player, score) in entries {
        match current_score {
            Some(cs) if cs == score => {
                // Same score — same rank
                players_at_rank += 1;
            }
            _ => {
                // New score — advance rank by number of players at previous rank
                current_rank += players_at_rank;
                players_at_rank = 1;
                current_score = Some(score);
            }
        }
        let rank = WinnerRank::new(current_rank).unwrap_or(WinnerRank::new(1).unwrap_or(WinnerRank(1)));
        result.push(StandingEntry::new(player, score, rank));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::player_color::PlayerColor;
    use crate::models::player_name::PlayerName;
    use crate::models::player_type::PlayerType;
    use crate::models::scorecard::Scorecard;

    fn make_player_with_score(name: &str, score: u32) -> Player {
        let mut player = Player::new(PlayerName::new(name).unwrap(), PlayerColor::RED, PlayerType::Human);
        if score > 0 {
            player
                .scorecard_mut()
                .enter(crate::models::category::ScoreCategory::Chance, Score::new(score))
                .unwrap();
        }
        player
    }

    #[test]
    fn winner_rank_new_valid() {
        assert_eq!(WinnerRank::new(1).unwrap().get(), 1);
        assert_eq!(WinnerRank::new(3).unwrap().get(), 3);
    }

    #[test]
    fn winner_rank_new_zero_fails() {
        assert!(WinnerRank::new(0).is_none());
    }

    #[test]
    fn winner_rank_is_first() {
        assert!(WinnerRank::new(1).unwrap().is_first());
        assert!(!WinnerRank::new(2).unwrap().is_first());
    }

    #[test]
    fn winner_rank_display() {
        assert_eq!(WinnerRank::new(1).unwrap().to_string(), "1. Platz");
        assert_eq!(WinnerRank::new(2).unwrap().to_string(), "2. Platz");
        assert_eq!(WinnerRank::new(5).unwrap().to_string(), "5. Platz");
    }

    #[test]
    fn compute_standings_single_winner() {
        let players = vec![
            make_player_with_score("Alice", 100),
            make_player_with_score("Bob", 200),
            make_player_with_score("Carol", 150),
        ];
        let standings = compute_standings(players);
        assert_eq!(standings.len(), 3);
        assert_eq!(standings[0].player().name().as_str(), "Bob");
        assert_eq!(standings[0].rank().get(), 1);
        assert_eq!(standings[1].player().name().as_str(), "Carol");
        assert_eq!(standings[1].rank().get(), 2);
        assert_eq!(standings[2].player().name().as_str(), "Alice");
        assert_eq!(standings[2].rank().get(), 3);
    }

    #[test]
    fn compute_standings_tie_produces_same_rank() {
        let players = vec![
            make_player_with_score("Alice", 200),
            make_player_with_score("Bob", 200),
            make_player_with_score("Carol", 100),
        ];
        let standings = compute_standings(players);
        assert_eq!(standings[0].rank().get(), 1);
        assert_eq!(standings[1].rank().get(), 1);
        assert_eq!(standings[2].rank().get(), 3);
    }

    #[test]
    fn compute_standings_single_player() {
        let players = vec![make_player_with_score("Solo", 42)];
        let standings = compute_standings(players);
        assert_eq!(standings.len(), 1);
        assert_eq!(standings[0].rank().get(), 1);
        assert!(standings[0].rank().is_first());
    }

    #[test]
    fn compute_standings_all_zero_scores() {
        let players = vec![make_player_with_score("A", 0), make_player_with_score("B", 0)];
        let standings = compute_standings(players);
        assert_eq!(standings[0].rank().get(), 1);
        assert_eq!(standings[1].rank().get(), 1);
    }
}
