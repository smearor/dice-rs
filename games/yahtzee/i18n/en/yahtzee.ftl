# App
app-title = Yahtzee

# Game mode
game-mode-single-player = Single Player
game-mode-multi-player = Multiplayer

# Game status
game-status-setup = Setup
game-status-playing = Game in progress
game-status-game-over = Game over

# Roll result
roll-result-yahtzee = Yahtzee!
roll-result-full-house = Full House!
roll-result-large-straight = Large Straight!
roll-result-small-straight = Small Straight!
roll-result-four-of-a-kind = Four of a Kind!
roll-result-three-of-a-kind = Three of a Kind
roll-result-normal = Normal

# Winner rank
winner-rank = { $rank }. Place

# Turn phase
turn-phase-awaiting-roll = Waiting for roll
turn-phase-rolling = Rolling...
turn-phase-holding = Holding dice
turn-phase-scoring = Entering score
turn-phase-turn-end = Turn ended

# Round number
round-number = Round { $current }/{ $total }

# Game phase
game-phase-setup = Setup
game-phase-playing = Playing
game-phase-game-over = Game Over

# Dice view
dice-tooltip-hold = Die { $slot } — Click to hold

# Reconnection
reconnection-reconnected = { $count } dice reconnected.

# Setup screen
window-setup-title = Yahtzee — Setup
setup-player-title = Set up players
scan-status-scanning = Scanning for dice...
scan-rescan = Scan again

# Player setup
player-add = + Add player
player-start = Start game
player-remove-tooltip = Remove player
player-default-name = Player { $index }
player-color-red = Red
player-color-green = Green
player-color-blue = Blue
player-color-yellow = Yellow
player-color-orange = Orange
player-color-purple = Purple
player-type-human = Human
player-type-computer = Computer
player-added = Player added.
player-max-reached = Maximum number of players reached.

# Roll button labels
roll-button-roll = Roll
roll-button-reroll = Roll again
roll-button-last = Last roll
roll-button-none = No rolls left
roll-button-rolling = Rolling...
roll-button-game-over = Game over

# Roll count display
roll-count = Roll { $current }/3
roll-count-remaining = Roll { $current }/3 — { $remaining } rolls left
roll-count-zero = Roll 0/3

# UI status messages
status-ready = Ready. Scan dice to begin.
status-scanning = Scanning for GoDice...
status-no-devices = No GoDice found.
status-devices-found = { $count } GoDice found, connecting...
status-dice-connected = Die { $slot } connected: { $name }
status-all-dice-connected = All 5 dice connected. Ready to play!
status-connection-failed = Connection failed for { $name }: { $error }
status-scan-failed = Scan failed: { $error }
status-roll-started = Rolling...
status-roll-complete = Roll complete. Choose a category.
status-roll-timed-out = Roll timed out.
status-dice-disconnected = Die { $slot } disconnected.
status-score-entered = { $player }: { $category } = { $score } points
status-game-over = Game over!
status-waiting-for-roll = { $player }'s turn. Roll to start.
status-hold-toggled-held = Die { $slot } held
status-hold-toggled-released = Die { $slot } released

# Window status messages
game-starting = Starting game ({ $count } players)...
game-started-scanning = Game started! Searching for GoDice...
error-prefix = Error: { $error }
reconnecting = Reconnecting...
player-turn = Player { $player }'s turn.
score-entered-for-player = { $category } entered for player { $player }.
category-crossed-out = { $category } crossed out.
category-crossed-out-for-player = { $category } crossed out for player { $player }.
dice-reconnected = { $count } dice reconnected.
settings-loaded = Settings loaded ({ $count } players)

# Scan status messages
scan-no-devices = No dice found. Scanning again...
scan-devices-found = { $count } dice found.
scan-device-found = Die { $color } found.
scan-connecting = Connecting to die { $color }...
scan-dice-connected = Die { $color } connected.
scan-failed = Scan failed. Retrying...
scan-all-connected = All 5 dice connected. Ready to play!
scan-swapping = Swapping die { $slot }...
scan-auto-swap = Die { $slot } unresponsive, auto-swapping...

# Dice colors
dice-color-black = Black
dice-color-red = Red
dice-color-green = Green
dice-color-blue = Blue
dice-color-yellow = Yellow
dice-color-orange = Orange

# Dice swap
dice-swap-tooltip = Click to swap this die

# Scorecard
scorecard-upper-section = Upper Section
scorecard-lower-section = Lower Section
scorecard-bonus = Bonus
scorecard-subtotal = Subtotal
scorecard-yahtzee-bonus = Yahtzee Bonus
scorecard-total = Total

# Categories
category-ones = Ones
category-twos = Twos
category-threes = Threes
category-fours = Fours
category-fives = Fives
category-sixes = Sixes
category-three-of-a-kind = Three of a Kind
category-four-of-a-kind = Four of a Kind
category-full-house = Full House
category-small-straight = Small Straight
category-large-straight = Large Straight
category-yahtzee = Yahtzee
category-chance = Chance

# Game end screen
game-end-title = Game Over!
game-end-new-game = New Game
game-end-winner = { $rank }: { $name }
game-end-score = { $score } points

# Reconnection overlay
reconnection-message = Dice connection lost!
reconnection-retry = Reconnect
reconnection-dismiss = Ignore
reconnection-slot-singular = Die { $slot } is disconnected
reconnection-slot-plural = Dice { $slots } are disconnected

# Turn transition overlay
transition-round = Round { $round }
transition-player = { $name }'s turn!
transition-ready = Ready!

# Bonus tracker
bonus-title = Bonus Progress
bonus-subtotal = Upper section: { $current }/{ $threshold }
bonus-achieved = ✓ Bonus achieved: +{ $points } points
bonus-reached = Bonus achieved!
bonus-remaining = { $remaining } points until bonus

# Hint panel
hint-title = Strategy Hint
hint-category = → { $category }
hint-score = { $score } points
hint-score-zero = 0 points (cross out)
hint-hold-all = Keep all dice
hint-hold-none = Reroll all dice
hint-hold-some = Keep: { $values }
hint-no-more-rolls = No more rolls available

# Menu
menu-highscore = Highscore
menu-info = Info

# Highscore dialog
highscore-dialog-title = Highscore
highscore-title = Top Scores
highscore-empty = No highscores yet.
highscore-score = { $score } points
highscore-close = Close

# Info dialog
info-dialog-title = Info
info-app-name = Yahtzee
info-app-description = Yahtzee for GoDice, built with GTK 4 and dice-rs
info-github = GitHub Repository
info-docs = docs.rs Documentation
info-particula = Particula Tech - GoDice
info-license = Licensed under the MIT License
info-close = Close
