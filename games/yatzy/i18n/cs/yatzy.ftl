# App
app-title = Yatzy

# Game mode
game-mode-single-player = Hra jednoho hráče
game-mode-multi-player = Hra více hráčů

# Game status
game-status-setup = Nastavení
game-status-playing = Hra probíhá
game-status-game-over = Konec hry

# Roll result
roll-result-yatzy = Yatzy!
roll-result-full-house = Full House!
roll-result-large-straight = Velká postupka!
roll-result-small-straight = Malá postupka!
roll-result-four-of-a-kind = Čtyřka!
roll-result-three-of-a-kind = Trojka
roll-result-normal = Normální

# Winner rank
winner-rank = { $rank }. místo

# Turn phase
turn-phase-awaiting-roll = Čeká na hod
turn-phase-rolling = Házení...
turn-phase-holding = Držení kostek
turn-phase-scoring = Zápis skóre
turn-phase-turn-end = Kolo ukončeno

# Round number
round-number = Kolo { $current }/{ $total }

# Game phase
game-phase-setup = Nastavení
game-phase-playing = Hra probíhá
game-phase-game-over = Konec hry

# Dice view
dice-tooltip-hold = Kostka { $slot } — Klikněte pro podržení

# Reconnection
reconnection-reconnected = { $count } kostek znovu připojeno.

# Setup screen
window-setup-title = Yatzy — Nastavení
setup-player-title = Nastavit hráče
scan-status-scanning = Hledání kostek...
scan-rescan = Hledat znovu

# Player setup
player-add = + Přidat hráče
player-start = Zahájit hru
player-remove-tooltip = Odebrat hráče
player-default-name = Hráč { $index }
player-color-tooltip = Vybrat barvu hráče
player-type-human = Člověk
player-type-computer = Počítač
player-added = Hráč přidán.
player-max-reached = Dosažen maximální počet hráčů.

# Roll button labels
roll-button-roll = Hodit
roll-button-reroll = Hodit znovu
roll-button-last = Poslední hod
roll-button-none = Žádné hody
roll-button-rolling = Házení...
roll-button-game-over = Konec hry

# Roll count display
roll-count = Hod { $current }/3
roll-count-remaining = Hod { $current }/3
roll-count-zero = Hod 0/3

# UI status messages
status-ready = Připraveno. Najděte kostky pro začátek.
status-scanning = Hledání GoDice...
status-no-devices = Nebyly nalezeny žádné GoDice.
status-devices-found = Nalezeno { $count } GoDice, připojování...
status-dice-connected = Kostka { $slot } připojena: { $name }
status-all-dice-connected = Všech 5 kostek připojeno. Připraveno hře!
status-connection-failed = Připojení selhalo pro { $name }: { $error }
status-scan-failed = Hledání selhalo: { $error }
status-roll-started = Házení...
status-roll-complete = Hod dokončen. Vyberte kategorii.
status-roll-timed-out = Čas hodu vypršel.
status-dice-disconnected = Kostka { $slot } odpojena.
status-score-entered = { $player }: { $category } = { $score } bodů
status-game-over = Konec hry!
status-waiting-for-roll = Na tahu je { $player }. Hoďte pro začátek.
status-hold-toggled-held = Kostka { $slot } podržena
status-hold-toggled-released = Kostka { $slot } uvolněna

# Window status messages
game-starting = Spouštění hry ({ $count } hráčů)...
game-started-scanning = Hra spuštěna! Hledání GoDice...
error-prefix = Chyba: { $error }
reconnecting = Znovu připojování...
player-turn = Na tahu je hráč { $player }.
score-entered-for-player = { $category } zapsáno pro hráče { $player }.
category-crossed-out = { $category } škrtnuto.
category-crossed-out-for-player = { $category } škrtnuto pro hráče { $player }.
dice-reconnected = { $count } kostek znovu připojeno.
settings-loaded = Nastavení načtena ({ $count } hráčů)

# Scan status messages
scan-no-devices = Nebyly nalezeny žádné kostky. Hledání znovu...
scan-devices-found = Nalezeno { $count } kostek.
scan-device-found = Kostka { $color } nalezena.
scan-connecting = Připojování ke kostce { $color }...
scan-dice-connected = Kostka { $color } připojena.
scan-failed = Hledání selhalo. Zkusit znovu...
scan-all-connected = Všech 5 kostek připojeno. Připraveno hře!
scan-swapping = Výměna kostky { $slot }...
scan-auto-swap = Kostka { $slot } nereaguje, automatická výměna...

# Dice colors
dice-color-black = Černá
dice-color-red = Červená
dice-color-green = Zelená
dice-color-blue = Modrá
dice-color-yellow = Žlutá
dice-color-orange = Oranžová

# Dice swap
dice-swap-tooltip = Klikněte pro výměnu této kostky

# Scorecard
scorecard-upper-section = Horní sekce
scorecard-lower-section = Dolní sekce
scorecard-bonus = Bonus
scorecard-subtotal = Mezisoučet
scorecard-yatzy-bonus = Bonus Yatzy
scorecard-total = Celkem

# Categories
category-ones = Jedničky
category-twos = Dvojky
category-threes = Trojky
category-fours = Čtyřky
category-fives = Pětice
category-sixes = Šestky
category-three-of-a-kind = Trojka
category-four-of-a-kind = Čtyřka
category-full-house = Full House
category-small-straight = Malá postupka
category-large-straight = Velká postupka
category-yatzy = Yatzy
category-chance = Šance

# Game end screen
game-end-title = Konec hry!
game-end-new-game = Nová hra
game-end-winner = { $rank }: { $name }
game-end-score = { $score } bodů

# Reconnection overlay
reconnection-message = Připojení kostek ztraceno!
reconnection-retry = Znovu připojit
reconnection-dismiss = Ignorovat
reconnection-slot-singular = Kostka { $slot } odpojena
reconnection-slot-plural = Kostky { $slots } odpojeny

# Turn transition overlay
transition-round = Kolo { $round }
transition-player = Na tahu je { $name }!
transition-ready = Připraveno!

# Bonus tracker
bonus-title = Pokrok bonusu
bonus-subtotal = Horní sekce: { $current }/{ $threshold }
bonus-achieved = ✓ Bonus dosažen: +{ $points } bodů
bonus-reached = Bonus dosažen!
bonus-remaining = { $remaining } bodů do bonusu

# Hint panel
hint-title = Strategická rada
hint-category = → { $category }
hint-score = { $score } bodů
hint-score-zero = 0 bodů (škrtnout)
hint-hold-all = Ponechat všechny kostky
hint-hold-none = Přehodit všechny kostky
hint-hold-some = Ponechat: { $values }
hint-no-more-rolls = Žádné další hody k dispozici

# Menu
menu-highscore = Nejlepší skóre
menu-info = Info

# Highscore dialog
highscore-dialog-title = Nejlepší skóre
highscore-title = Žebříček
highscore-empty = Zatím žádné nejlepší skóre.
highscore-score = { $score } bodů
highscore-close = Zavřít

# Info dialog
info-dialog-title = Info
info-app-name = Yatzy
info-app-description = Yatzy pro GoDice, postavené s GTK 4 a dice-rs
info-github = GitHub repozitář
info-docs = Dokumentace docs.rs
info-particula = Particula Tech - GoDice
info-license = Licence MIT
info-close = Zavřít

# Computer AI
ai-thinking = Počítač přemýšlí...
ai-rolling = Počítač hází kostkami.
ai-holding = Počítač podržuje { $count } kostek.
ai-scored = Počítač zapisuje { $category } ({ $score } bodů).
ai-crossed-out = Počítač škrtá { $category }.
