# App
app-title = Yatzy

# Game mode
game-mode-single-player = Enspelare
game-mode-multi-player = Flera spelare

# Game status
game-status-setup = Konfiguration
game-status-playing = Spel pågår
game-status-game-over = Spelet slut

# Roll result
roll-result-yatzy = Yatzy!
roll-result-full-house = Kåk!
roll-result-large-straight = Stor stege!
roll-result-small-straight = Liten stege!
roll-result-four-of-a-kind = Fyrtal!
roll-result-three-of-a-kind = Triss
roll-result-normal = Normal

# Winner rank
winner-rank = { $rank }. plats

# Turn phase
turn-phase-awaiting-roll = Väntar på tärningskast
turn-phase-rolling = Kastar...
turn-phase-holding = Håller tärningar
turn-phase-scoring = För in poäng
turn-phase-turn-end = Tur slut

# Round number
round-number = Omgång { $current }/{ $total }

# Game phase
game-phase-setup = Konfiguration
game-phase-playing = Spel pågår
game-phase-game-over = Spelet slut

# Dice view
dice-tooltip-hold = Tärning { $slot } — Klicka för att hålla

# Reconnection
reconnection-reconnected = { $count } tärningar återanslutna.

# Setup screen
window-setup-title = Yatzy — Konfiguration
setup-player-title = Konfigurera spelare
scan-status-scanning = Söker efter tärningar...
scan-rescan = Sök igen

# Player setup
player-add = + Lägg till spelare
player-start = Starta spel
player-remove-tooltip = Ta bort spelare
player-default-name = Spelare { $index }
player-color-tooltip = Välj spelarfärg
player-type-human = Människa
player-type-computer = Dator
player-added = Spelare tillagd.
player-max-reached = Maximalt antal spelare nått.

# Roll button labels
roll-button-roll = Kasta
roll-button-reroll = Kasta igen
roll-button-last = Sista kastet
roll-button-none = Inga kast kvar
roll-button-rolling = Kastar...
roll-button-game-over = Spelet slut

# Roll count display
roll-count = Kast { $current }/3
roll-count-remaining = Kast { $current }/3
roll-count-zero = Kast 0/3

# UI status messages
status-ready = Redo. Sök efter tärningar för att börja.
status-scanning = Söker efter GoDice...
status-no-devices = Inga GoDice hittades.
status-devices-found = { $count } GoDice hittades, ansluter...
status-dice-connected = Tärning { $slot } ansluten: { $name }
status-all-dice-connected = Alla 5 tärningar anslutna. Redo att spela!
status-connection-failed = Anslutning misslyckades för { $name }: { $error }
status-scan-failed = Sökning misslyckades: { $error }
status-roll-started = Kastar...
status-roll-complete = Kast komplett. Välj en kategori.
status-roll-timed-out = Kastet tog för lång tid.
status-dice-disconnected = Tärning { $slot } frånkopplad.
status-score-entered = { $player }: { $category } = { $score } poäng
status-game-over = Spelet slut!
status-waiting-for-roll = { $player }s tur. Kasta för att börja.
status-hold-toggled-held = Tärning { $slot } hållen
status-hold-toggled-released = Tärning { $slot } släppt

# Window status messages
game-starting = Startar spel ({ $count } spelare)...
game-started-scanning = Spel startat! Söker efter GoDice...
error-prefix = Fel: { $error }
reconnecting = Återansluter...
player-turn = Spelare { $player }s tur.
score-entered-for-player = { $category } införd för spelare { $player }.
category-crossed-out = { $category } struket.
category-crossed-out-for-player = { $category } struket för spelare { $player }.
dice-reconnected = { $count } tärningar återanslutna.
settings-loaded = Inställningar inlästa ({ $count } spelare)

# Scan status messages
scan-no-devices = Inga tärningar hittades. Söker igen...
scan-devices-found = { $count } tärningar hittades.
scan-device-found = Tärning { $color } hittad.
scan-connecting = Ansluter till tärning { $color }...
scan-dice-connected = Tärning { $color } ansluten.
scan-failed = Sökning misslyckades. Försöker igen...
scan-all-connected = Alla 5 tärningar anslutna. Redo att spela!
scan-swapping = Byter tärning { $slot }...
scan-auto-swap = Tärning { $slot } svarar inte, automatiskt byte...

# Dice colors
dice-color-black = Svart
dice-color-red = Röd
dice-color-green = Grön
dice-color-blue = Blå
dice-color-yellow = Gul
dice-color-orange = Orange

# Dice swap
dice-swap-tooltip = Klicka för att byta denna tärning

# Scorecard
scorecard-upper-section = Övre sektion
scorecard-lower-section = Nedre sektion
scorecard-bonus = Bonus
scorecard-subtotal = Delsumma
scorecard-yatzy-bonus = Yatzy-bonus
scorecard-total = Totalt

# Categories
category-ones = Ettor
category-twos = Tvåor
category-threes = Treor
category-fours = Fyror
category-fives = Femmor
category-sixes = Sexor
category-three-of-a-kind = Triss
category-four-of-a-kind = Fyrtal
category-full-house = Kåk
category-small-straight = Liten stege
category-large-straight = Stor stege
category-yatzy = Yatzy
category-chance = Chans

# Game end screen
game-end-title = Spelet slut!
game-end-new-game = Nytt spel
game-end-winner = { $rank }: { $name }
game-end-score = { $score } poäng

# Reconnection overlay
reconnection-message = Tärningsanslutning förlorad!
reconnection-retry = Återanslut
reconnection-dismiss = Ignorera
reconnection-slot-singular = Tärning { $slot } frånkopplad
reconnection-slot-plural = Tärningar { $slots } frånkopplade

# Turn transition overlay
transition-round = Omgång { $round }
transition-player = { $name }s tur!
transition-ready = Redo!

# Bonus tracker
bonus-title = Bonusförlopp
bonus-subtotal = Övre sektion: { $current }/{ $threshold }
bonus-achieved = ✓ Bonus uppnådd: +{ $points } poäng
bonus-reached = Bonus uppnådd!
bonus-remaining = { $remaining } poäng till bonus

# Hint panel
hint-title = Strategiförslag
hint-category = → { $category }
hint-score = { $score } poäng
hint-score-zero = 0 poäng (stryk)
hint-hold-all = Behåll alla tärningar
hint-hold-none = Kasta om alla tärningar
hint-hold-some = Behåll: { $values }
hint-no-more-rolls = Inga fler kast tillgängliga

# Menu
menu-highscore = Highscore
menu-info = Info

# Highscore dialog
highscore-dialog-title = Highscore
highscore-title = Toppresultat
highscore-empty = Inga highscores ännu.
highscore-score = { $score } poäng
highscore-close = Stäng

# Info dialog
info-dialog-title = Info
info-app-name = Yatzy
info-app-description = Yatzy för GoDice, byggt med GTK 4 och dice-rs
info-github = GitHub-repository
info-docs = docs.rs-dokumentation
info-particula = Particula Tech - GoDice
info-license = Licensierad under MIT-licensen
info-close = Stäng

# Computer AI
ai-thinking = Datorn tänker...
ai-rolling = Datorn kastar tärningarna.
ai-holding = Datorn håller { $count } tärningar.
ai-scored = Datorn för in { $category } ({ $score } poäng).
ai-crossed-out = Datorn stryker { $category }.
