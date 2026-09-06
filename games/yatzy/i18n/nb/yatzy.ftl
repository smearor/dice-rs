# App
app-title = Yatzy

# Game mode
game-mode-single-player = Enkeltspiller
game-mode-multi-player = Flerspiller

# Game status
game-status-setup = Oppsett
game-status-playing = Spill pågår
game-status-game-over = Spillet er over

# Roll result
roll-result-yatzy = Yatzy!
roll-result-full-house = Hus!
roll-result-large-straight = Stor straight!
roll-result-small-straight = Liten straight!
roll-result-four-of-a-kind = Fire like!
roll-result-three-of-a-kind = Tre like
roll-result-normal = Normal

# Winner rank
winner-rank = { $rank }. plass

# Turn phase
turn-phase-awaiting-roll = Venter på kast
turn-phase-rolling = Kaster...
turn-phase-holding = Holder terninger
turn-phase-scoring = Fører inn poeng
turn-phase-turn-end = Tur slutt

# Round number
round-number = Runde { $current }/{ $total }

# Game phase
game-phase-setup = Oppsett
game-phase-playing = Spill pågår
game-phase-game-over = Spillet er over

# Dice view
dice-tooltip-hold = Terning { $slot } — Klikk for å holde

# Reconnection
reconnection-reconnected = { $count } terninger koblet til igjen.

# Setup screen
window-setup-title = Yatzy — Oppsett
setup-player-title = Sett opp spillere
scan-status-scanning = Søker etter terninger...
scan-rescan = Søk igjen

# Player setup
player-add = + Legg til spiller
player-start = Start spill
player-remove-tooltip = Fjern spiller
player-default-name = Spiller { $index }
player-color-tooltip = Velg spillerfarge
player-type-human = Menneske
player-type-computer = Datamaskin
player-added = Spiller lagt til.
player-max-reached = Maksimalt antall spillere nådd.

# Roll button labels
roll-button-roll = Kast
roll-button-reroll = Kast igjen
roll-button-last = Siste kast
roll-button-none = Ingen kast igjen
roll-button-rolling = Kaster...
roll-button-game-over = Spillet er over

# Roll count display
roll-count = Kast { $current }/3
roll-count-remaining = Kast { $current }/3
roll-count-zero = Kast 0/3

# UI status messages
status-ready = Klar. Søk etter terninger for å starte.
status-scanning = Søker etter GoDice...
status-no-devices = Ingen GoDice funnet.
status-devices-found = { $count } GoDice funnet, kobler til...
status-dice-connected = Terning { $slot } koblet til: { $name }
status-all-dice-connected = Alle 5 terninger koblet til. Klar for spill!
status-connection-failed = Tilkobling mislyktes for { $name }: { $error }
status-scan-failed = Søk mislyktes: { $error }
status-roll-started = Kaster...
status-roll-complete = Kast fullført. Velg en kategori.
status-roll-timed-out = Kastet tok for lang tid.
status-dice-disconnected = Terning { $slot } frakoblet.
status-score-entered = { $player }: { $category } = { $score } poeng
status-game-over = Spillet er over!
status-waiting-for-roll = { $player }s tur. Kast for å starte.
status-hold-toggled-held = Terning { $slot } holdt
status-hold-toggled-released = Terning { $slot } sluppet

# Window status messages
game-starting = Starter spill ({ $count } spillere)...
game-started-scanning = Spill startet! Søker etter GoDice...
error-prefix = Feil: { $error }
reconnecting = Kobler til igjen...
player-turn = Spiller { $player }s tur.
score-entered-for-player = { $category } ført inn for spiller { $player }.
category-crossed-out = { $category } strøket.
category-crossed-out-for-player = { $category } strøket for spiller { $player }.
dice-reconnected = { $count } terninger koblet til igjen.
settings-loaded = Innstillinger lastet ({ $count } spillere)

# Scan status messages
scan-no-devices = Ingen terninger funnet. Søker igjen...
scan-devices-found = { $count } terninger funnet.
scan-device-found = Terning { $color } funnet.
scan-connecting = Kobler til terning { $color }...
scan-dice-connected = Terning { $color } koblet til.
scan-failed = Søk mislyktes. Prøver igjen...
scan-all-connected = Alle 5 terninger koblet til. Klar for spill!
scan-swapping = Bytter terning { $slot }...
scan-auto-swap = Terning { $slot } svarer ikke, automatisk bytte...

# Dice colors
dice-color-black = Svart
dice-color-red = Rød
dice-color-green = Grønn
dice-color-blue = Blå
dice-color-yellow = Gul
dice-color-orange = Oransje

# Dice swap
dice-swap-tooltip = Klikk for å bytte denne terningen

# Scorecard
scorecard-upper-section = Øvre seksjon
scorecard-lower-section = Nedre seksjon
scorecard-bonus = Bonus
scorecard-subtotal = Subtotal
scorecard-yatzy-bonus = Yatzy-bonus
scorecard-total = Totalt

# Categories
category-ones = Enere
category-twos = Toere
category-threes = Treere
category-fours = Firere
category-fives = Femmere
category-sixes = Seksere
category-three-of-a-kind = Tre like
category-four-of-a-kind = Fire like
category-full-house = Hus
category-small-straight = Liten straight
category-large-straight = Stor straight
category-yatzy = Yatzy
category-chance = Sjanse

# Game end screen
game-end-title = Spillet er over!
game-end-new-game = Nytt spill
game-end-winner = { $rank }: { $name }
game-end-score = { $score } poeng

# Reconnection overlay
reconnection-message = Terningtilkobling mistet!
reconnection-retry = Koble til igjen
reconnection-dismiss = Ignorer
reconnection-slot-singular = Terning { $slot } frakoblet
reconnection-slot-plural = Terninger { $slots } frakoblet

# Turn transition overlay
transition-round = Runde { $round }
transition-player = { $name }s tur!
transition-ready = Klar!

# Bonus tracker
bonus-title = Bonusforløp
bonus-subtotal = Øvre seksjon: { $current }/{ $threshold }
bonus-achieved = ✓ Bonus oppnådd: +{ $points } poeng
bonus-reached = Bonus oppnådd!
bonus-remaining = { $remaining } poeng til bonus

# Hint panel
hint-title = Strategiforslag
hint-category = → { $category }
hint-score = { $score } poeng
hint-score-zero = 0 poeng (strøk)
hint-hold-all = Behold alle terninger
hint-hold-none = Kast alle terninger på nytt
hint-hold-some = Behold: { $values }
hint-no-more-rolls = Ingen flere kast tilgjengelige

# Menu
menu-highscore = Highscore
menu-info = Info

# Highscore dialog
highscore-dialog-title = Highscore
highscore-title = Toppresultater
highscore-empty = Ingen highscores ennå.
highscore-score = { $score } poeng
highscore-close = Lukk

# Info dialog
info-dialog-title = Info
info-app-name = Yatzy
info-app-description = Yatzy for GoDice, bygget med GTK 4 og dice-rs
info-github = GitHub-repository
info-docs = docs.rs-dokumentasjon
info-particula = Particula Tech - GoDice
info-license = Lisensiert under MIT-lisensen
info-close = Lukk

# Computer AI
ai-thinking = Datamaskinen tenker...
ai-rolling = Datamaskinen kaster terningene.
ai-holding = Datamaskinen holder { $count } terninger.
ai-scored = Datamaskinen fører inn { $category } ({ $score } poeng).
ai-crossed-out = Datamaskinen stryker { $category }.
