# App
app-title = Yatzy

# Game mode
game-mode-single-player = Enkelt spiller
game-mode-multi-player = Flerspiller

# Game status
game-status-setup = Opsætning
game-status-playing = Spil i gang
game-status-game-over = Spillet slut

# Roll result
roll-result-yatzy = Yatzy!
roll-result-full-house = Hus!
roll-result-large-straight = Stor straight!
roll-result-small-straight = Lille straight!
roll-result-four-of-a-kind = Fire ens!
roll-result-three-of-a-kind = Tre ens
roll-result-normal = Normal

# Winner rank
winner-rank = { $rank }. plads

# Turn phase
turn-phase-awaiting-roll = Venter på kast
turn-phase-rolling = Kaster...
turn-phase-holding = Holder terninger
turn-phase-scoring = Indtaster point
turn-phase-turn-end = Tur slut

# Round number
round-number = Runde { $current }/{ $total }

# Game phase
game-phase-setup = Opsætning
game-phase-playing = Spil i gang
game-phase-game-over = Spillet slut

# Dice view
dice-tooltip-hold = Terning { $slot } — Klik for at holde

# Reconnection
reconnection-reconnected = { $count } terninger genoprettet.

# Setup screen
window-setup-title = Yatzy — Opsætning
setup-player-title = Opsæt spillere
scan-status-scanning = Søger efter terninger...
scan-rescan = Søg igen

# Player setup
player-add = + Tilføj spiller
player-start = Start spil
player-remove-tooltip = Fjern spiller
player-default-name = Spiller { $index }
player-color-tooltip = Vælg spillerfarve
player-type-human = Menneske
player-type-computer = Computer
player-added = Spiller tilføjet.
player-max-reached = Maksimalt antal spillere nået.

# Roll button labels
roll-button-roll = Kast
roll-button-reroll = Kast igen
roll-button-last = Sidste kast
roll-button-none = Ingen kast tilbage
roll-button-rolling = Kaster...
roll-button-game-over = Spillet slut

# Roll count display
roll-count = Kast { $current }/3
roll-count-remaining = Kast { $current }/3
roll-count-zero = Kast 0/3

# UI status messages
status-ready = Klar. Søg efter terninger for at starte.
status-scanning = Søger efter GoDice...
status-no-devices = Ingen GoDice fundet.
status-devices-found = { $count } GoDice fundet, forbinder...
status-dice-connected = Terning { $slot } forbundet: { $name }
status-all-dice-connected = Alle 5 terninger forbundet. Klar til spil!
status-connection-failed = Forbindelse mislykkedes for { $name }: { $error }
status-scan-failed = Søgning mislykkedes: { $error }
status-roll-started = Kaster...
status-roll-complete = Kast fuldført. Vælg en kategori.
status-roll-timed-out = Kastet timed out.
status-dice-disconnected = Terning { $slot } afbrudt.
status-score-entered = { $player }: { $category } = { $score } point
status-game-over = Spillet slut!
status-waiting-for-roll = { $player }s tur. Kast for at starte.
status-hold-toggled-held = Terning { $slot } holdt
status-hold-toggled-released = Terning { $slot } frigivet

# Window status messages
game-starting = Starter spil ({ $count } spillere)...
game-started-scanning = Spil startet! Søger efter GoDice...
error-prefix = Fejl: { $error }
reconnecting = Genopretter forbindelse...
player-turn = Spiller { $player }s tur.
score-entered-for-player = { $category } indtastet for spiller { $player }.
category-crossed-out = { $category } streget ud.
category-crossed-out-for-player = { $category } streget ud for spiller { $player }.
dice-reconnected = { $count } terninger genoprettet.
settings-loaded = Indstillinger indlæst ({ $count } spillere)

# Scan status messages
scan-no-devices = Ingen terninger fundet. Søger igen...
scan-devices-found = { $count } terninger fundet.
scan-device-found = Terning { $color } fundet.
scan-connecting = Forbinder til terning { $color }...
scan-dice-connected = Terning { $color } forbundet.
scan-failed = Søgning mislykkedes. Prøver igen...
scan-all-connected = Alle 5 terninger forbundet. Klar til spil!
scan-swapping = Udskifter terning { $slot }...
scan-auto-swap = Terning { $slot } svarer ikke, automatisk udskiftning...

# Dice colors
dice-color-black = Sort
dice-color-red = Rød
dice-color-green = Grøn
dice-color-blue = Blå
dice-color-yellow = Gul
dice-color-orange = Orange

# Dice swap
dice-swap-tooltip = Klik for at udskifte denne terning

# Scorecard
scorecard-upper-section = Øvre sektion
scorecard-lower-section = Nedre sektion
scorecard-bonus = Bonus
scorecard-subtotal = Subtotal
scorecard-yatzy-bonus = Yatzy Bonus
scorecard-total = Total

# Categories
category-ones = Enere
category-twos = Toere
category-threes = Treere
category-fours = Firere
category-fives = Femere
category-sixes = Seksere
category-three-of-a-kind = Tre ens
category-four-of-a-kind = Fire ens
category-full-house = Hus
category-small-straight = Lille straight
category-large-straight = Stor straight
category-yatzy = Yatzy
category-chance = Chance

# Game end screen
game-end-title = Spillet slut!
game-end-new-game = Nyt spil
game-end-winner = { $rank }: { $name }
game-end-score = { $score } point

# Reconnection overlay
reconnection-message = Terningforbindelse mistet!
reconnection-retry = Genopret
reconnection-dismiss = Ignorer
reconnection-slot-singular = Terning { $slot } afbrudt
reconnection-slot-plural = Terninger { $slots } afbrudt

# Turn transition overlay
transition-round = Runde { $round }
transition-player = { $name }s tur!
transition-ready = Klar!

# Bonus tracker
bonus-title = Bonusstatus
bonus-subtotal = Øvre sektion: { $current }/{ $threshold }
bonus-achieved = ✓ Bonus opnået: +{ $points } point
bonus-reached = Bonus opnået!
bonus-remaining = { $remaining } point til bonus

# Hint panel
hint-title = Strategiforslag
hint-category = → { $category }
hint-score = { $score } point
hint-score-zero = 0 point (stryg ud)
hint-hold-all = Behold alle terninger
hint-hold-none = Kast alle terninger igen
hint-hold-some = Behold: { $values }
hint-no-more-rolls = Ingen flere kast tilgængelige

# Menu
menu-highscore = Highscore
menu-info = Info

# Highscore dialog
highscore-dialog-title = Highscore
highscore-title = Topscorer
highscore-empty = Ingen highscores endnu.
highscore-score = { $score } point
highscore-close = Luk

# Info dialog
info-dialog-title = Info
info-app-name = Yatzy
info-app-description = Yatzy for GoDice, bygget med GTK 4 og dice-rs
info-github = GitHub repository
info-docs = docs.rs dokumentation
info-particula = Particula Tech - GoDice
info-license = Licenseret under MIT-licensen
info-close = Luk

# Computer AI
ai-thinking = Computeren tænker...
ai-rolling = Computeren kaster terningerne.
ai-holding = Computeren holder { $count } terninger.
ai-scored = Computeren indtaster { $category } ({ $score } point).
ai-crossed-out = Computeren streger { $category } ud.
