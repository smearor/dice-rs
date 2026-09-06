# App
app-title = Yatzy

# Game mode
game-mode-single-player = Yksinpelaaja
game-mode-multi-player = Moninpeli

# Game status
game-status-setup = Asetukset
game-status-playing = Peli käynnissä
game-status-game-over = Peli ohi

# Roll result
roll-result-yatzy = Yatzy!
roll-result-full-house = Täyskäsi!
roll-result-large-straight = Iso suora!
roll-result-small-straight = Pieni suora!
roll-result-four-of-a-kind = Neloset!
roll-result-three-of-a-kind = Kolmoset
roll-result-normal = Normaali

# Winner rank
winner-rank = { $rank }. sija

# Turn phase
turn-phase-awaiting-roll = Odotetaan heittoa
turn-phase-rolling = Heitetään...
turn-phase-holding = Sidotaan noppia
turn-phase-scoring = Kirjataan pisteet
turn-phase-turn-end = Vuoro ohi

# Round number
round-number = Kierros { $current }/{ $total }

# Game phase
game-phase-setup = Asetukset
game-phase-playing = Peli käynnissä
game-phase-game-over = Peli ohi

# Dice view
dice-tooltip-hold = Noppa { $slot } — Klikkaa sidottavaksi

# Reconnection
reconnection-reconnected = { $count } noppaa yhdistetty uudelleen.

# Setup screen
window-setup-title = Yatzy — Asetukset
setup-player-title = Määritä pelaajat
scan-status-scanning = Etsitään noppia...
scan-rescan = Etsi uudelleen

# Player setup
player-add = + Lisää pelaaja
player-start = Aloita peli
player-remove-tooltip = Poista pelaaja
player-default-name = Pelaaja { $index }
player-color-tooltip = Valitse pelaajan väri
player-type-human = Ihminen
player-type-computer = Tietokone
player-added = Pelaaja lisätty.
player-max-reached = Maksimimäärä pelaajia saavutettu.

# Roll button labels
roll-button-roll = Heitä
roll-button-reroll = Heitä uudelleen
roll-button-last = Viimeinen heitto
roll-button-none = Ei heittoja jäljellä
roll-button-rolling = Heitetään...
roll-button-game-over = Peli ohi

# Roll count display
roll-count = Heitto { $current }/3
roll-count-remaining = Heitto { $current }/3
roll-count-zero = Heitto 0/3

# UI status messages
status-ready = Valmis. Etsi noppia aloittaaksesi.
status-scanning = Etsitään GoDice...
status-no-devices = GoDice ei löytynyt.
status-devices-found = { $count } GoDice löytyi, yhdistetään...
status-dice-connected = Noppa { $slot } yhdistetty: { $name }
status-all-dice-connected = Kaikki 5 noppaa yhdistetty. Valmis pelaamaan!
status-connection-failed = Yhdistäminen epäonnistui: { $name }: { $error }
status-scan-failed = Haku epäonnistui: { $error }
status-roll-started = Heitetään...
status-roll-complete = Heitto valmis. Valitse kategoria.
status-roll-timed-out = Heitto aikakatkaistiin.
status-dice-disconnected = Noppa { $slot } katkaistu.
status-score-entered = { $player }: { $category } = { $score } pistettä
status-game-over = Peli ohi!
status-waiting-for-roll = Pelaajan { $player } vuoro. Heitä aloittaaksesi.
status-hold-toggled-held = Noppa { $slot } sidottu
status-hold-toggled-released = Noppa { $slot } vapautettu

# Window status messages
game-starting = Aloitetaan peli ({ $count } pelaajaa)...
game-started-scanning = Peli aloitettu! Etsitään GoDice...
error-prefix = Virhe: { $error }
reconnecting = Yhdistetään uudelleen...
player-turn = Pelaajan { $player } vuoro.
score-entered-for-player = { $category } kirjattu pelaajalle { $player }.
category-crossed-out = { $category } yliviivattu.
category-crossed-out-for-player = { $category } yliviivattu pelaajalle { $player }.
dice-reconnected = { $count } noppaa yhdistetty uudelleen.
settings-loaded = Asetukset ladattu ({ $count } pelaajaa)

# Scan status messages
scan-no-devices = Noppia ei löytynyt. Etsitään uudelleen...
scan-devices-found = { $count } noppaa löytyi.
scan-device-found = Noppa { $color } löytyi.
scan-connecting = Yhdistetään noppaan { $color }...
scan-dice-connected = Noppa { $color } yhdistetty.
scan-failed = Haku epäonnistui. Yritetään uudelleen...
scan-all-connected = Kaikki 5 noppaa yhdistetty. Valmis pelaamaan!
scan-swapping = Vaihdetaan noppa { $slot }...
scan-auto-swap = Noppa { $slot } ei vastaa, automaattinen vaihto...

# Dice colors
dice-color-black = Musta
dice-color-red = Punainen
dice-color-green = Vihreä
dice-color-blue = Sininen
dice-color-yellow = Keltainen
dice-color-orange = Oranssi

# Dice swap
dice-swap-tooltip = Klikkaa vaihtaaksesi tämän nopan

# Scorecard
scorecard-upper-section = Yläosio
scorecard-lower-section = Alaosio
scorecard-bonus = Bonus
scorecard-subtotal = Välisumma
scorecard-yatzy-bonus = Yatzy-bonus
scorecard-total = Yhteensä

# Categories
category-ones = Ykköset
category-twos = Kakkoset
category-threes = Kolmoset
category-fours = Neloset
category-fives = Viitoset
category-sixes = Kuutoset
category-three-of-a-kind = Kolmoset
category-four-of-a-kind = Neloset
category-full-house = Täyskäsi
category-small-straight = Pieni suora
category-large-straight = Iso suora
category-yatzy = Yatzy
category-chance = Sattuma

# Game end screen
game-end-title = Peli ohi!
game-end-new-game = Uusi peli
game-end-winner = { $rank }: { $name }
game-end-score = { $score } pistettä

# Reconnection overlay
reconnection-message = Noppien yhteys menetetty!
reconnection-retry = Yhdistä uudelleen
reconnection-dismiss = Ohita
reconnection-slot-singular = Noppa { $slot } katkaistu
reconnection-slot-plural = Nopat { $slots } katkaistu

# Turn transition overlay
transition-round = Kierros { $round }
transition-player = { $name }n vuoro!
transition-ready = Valmis!

# Bonus tracker
bonus-title = Bonuksen edistyminen
bonus-subtotal = Yläosio: { $current }/{ $threshold }
bonus-achieved = ✓ Bonus saavutettu: +{ $points } pistettä
bonus-reached = Bonus saavutettu!
bonus-remaining = { $remaining } pistettä bonusseen

# Hint panel
hint-title = Strategiavinkki
hint-category = → { $category }
hint-score = { $score } pistettä
hint-score-zero = 0 pistettä (yliviivaa)
hint-hold-all = Säilytä kaikki nopat
hint-hold-none = Heitä kaikki nopat uudelleen
hint-hold-some = Säilytä: { $values }
hint-no-more-rolls = Ei enää heittoja saatavilla

# Menu
menu-highscore = Ennätykset
menu-info = Tietoja

# Highscore dialog
highscore-dialog-title = Ennätykset
highscore-title = Parhaat tulokset
highscore-empty = Ei vielä ennätyksiä.
highscore-score = { $score } pistettä
highscore-close = Sulje

# Info dialog
info-dialog-title = Tietoja
info-app-name = Yatzy
info-app-description = Yatzy GoDicelle, rakennettu GTK 4:llä ja dice-rs:llä
info-github = GitHub-repositorio
info-docs = docs.rs-dokumentaatio
info-particula = Particula Tech - GoDice
info-license = MIT-lisenssi
info-close = Sulje

# Computer AI
ai-thinking = Tietokone miettii...
ai-rolling = Tietokone heittää noppia.
ai-holding = Tietokone sitoo { $count } noppaa.
ai-scored = Tietokone kirjaa { $category } ({ $score } pistettä).
ai-crossed-out = Tietokone yliviivaa { $category }.
