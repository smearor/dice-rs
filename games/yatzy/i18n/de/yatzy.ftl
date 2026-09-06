# App
app-title = Kniffel

# Game mode
game-mode-single-player = Einzelspieler
game-mode-multi-player = Mehrspieler

# Game status
game-status-setup = Setup
game-status-playing = Spiel läuft
game-status-game-over = Spiel beendet

# Roll result
roll-result-yatzy = Yatzy!
roll-result-full-house = Full House!
roll-result-large-straight = Große Straße!
roll-result-small-straight = Kleine Straße!
roll-result-four-of-a-kind = Vierlinge!
roll-result-three-of-a-kind = Drillinge
roll-result-normal = Normal

# Winner rank
winner-rank = { $rank }. Platz

# Turn phase
turn-phase-awaiting-roll = Warte auf Wurf
turn-phase-rolling = Würfeln...
turn-phase-holding = Würfel halten
turn-phase-scoring = Punkte eintragen
turn-phase-turn-end = Zug beendet

# Round number
round-number = Runde { $current }/{ $total }

# Game phase
game-phase-setup = Setup
game-phase-playing = Spiel läuft
game-phase-game-over = Spiel beendet

# Dice view
dice-tooltip-hold = Würfel { $slot } — Klicken zum Halten

# Reconnection
reconnection-reconnected = { $count } Würfel erneut verbunden.

# Setup screen
window-setup-title = Kniffel — Setup
setup-player-title = Spieler einrichten
scan-status-scanning = Scanne nach Würfeln...
scan-rescan = Erneut scannen

# Player setup
player-add = + Spieler hinzufügen
player-start = Spiel starten
player-remove-tooltip = Spieler entfernen
player-default-name = Spieler { $index }
player-color-red = Rot
player-color-green = Grün
player-color-blue = Blau
player-color-yellow = Gelb
player-color-orange = Orange
player-color-purple = Lila
player-type-human = Mensch
player-type-computer = Computer
player-added = Spieler hinzugefügt.
player-max-reached = Maximale Spielerzahl erreicht.

# Roll button labels
roll-button-roll = Würfeln
roll-button-reroll = Nochmal würfeln
roll-button-last = Letzter Wurf
roll-button-none = Keine Würfe übrig
roll-button-rolling = Wurf läuft...
roll-button-game-over = Spiel beendet

# Roll count display
roll-count = Wurf { $current }/3
roll-count-remaining = Wurf { $current }/3 — { $remaining } Würfe übrig
roll-count-zero = Wurf 0/3

# UI status messages
status-ready = Bereit. Würfel scannen, um zu beginnen.
status-scanning = Scanne nach GoDice...
status-no-devices = Keine GoDice gefunden.
status-devices-found = { $count } GoDice gefunden, verbinde...
status-dice-connected = Würfel { $slot } verbunden: { $name }
status-all-dice-connected = Alle 5 Würfel verbunden. Spiel bereit!
status-connection-failed = Verbindung fehlgeschlagen für { $name }: { $error }
status-scan-failed = Scan fehlgeschlagen: { $error }
status-roll-started = Würfeln...
status-roll-complete = Wurf abgeschlossen. Kategorie wählen.
status-roll-timed-out = Zeitüberschreitung beim Würfeln.
status-dice-disconnected = Würfel { $slot } getrennt.
status-score-entered = { $player }: { $category } = { $score } Punkte
status-game-over = Spiel beendet!
status-waiting-for-roll = { $player } ist am Zug. Würfeln zum Starten.
status-hold-toggled-held = Würfel { $slot } gehalten
status-hold-toggled-released = Würfel { $slot } freigegeben

# Window status messages
game-starting = Spiel wird gestartet ({ $count } Spieler)...
game-started-scanning = Spiel gestartet! Suche GoDice-Würfel...
error-prefix = Fehler: { $error }
reconnecting = Verbinde erneut...
player-turn = Spieler { $player } ist an der Reihe.
score-entered-for-player = { $category } eingetragen für Spieler { $player }.
category-crossed-out = { $category } gestrichen.
category-crossed-out-for-player = { $category } gestrichen für Spieler { $player }.
dice-reconnected = { $count } Würfel erneut verbunden.
settings-loaded = Einstellungen geladen ({ $count } Spieler)

# Scan status messages
scan-no-devices = Keine Würfel gefunden. Erneutes Scannen...
scan-devices-found = { $count } Würfel gefunden.
scan-device-found = Würfel { $color } gefunden.
scan-connecting = Verbinde mit Würfel { $color }...
scan-dice-connected = Würfel { $color } verbunden.
scan-failed = Scan fehlgeschlagen. Erneuter Versuch...
scan-all-connected = Alle 5 Würfel verbunden. Spiel bereit!
scan-swapping = Tausche Würfel { $slot }...
scan-auto-swap = Würfel { $slot } reagiert nicht, automatischer Tausch...

# Dice colors
dice-color-black = Schwarz
dice-color-red = Rot
dice-color-green = Grün
dice-color-blue = Blau
dice-color-yellow = Gelb
dice-color-orange = Orange

# Dice swap
dice-swap-tooltip = Klicken zum Austauschen

# Scorecard
scorecard-upper-section = Obere Hälfte
scorecard-lower-section = Untere Hälfte
scorecard-bonus = Bonus
scorecard-subtotal = Zwischensumme
scorecard-yatzy-bonus = Kniffel Bonus
scorecard-total = Gesamt

# Categories
category-ones = Einser
category-twos = Zweier
category-threes = Dreier
category-fours = Vierer
category-fives = Fünfer
category-sixes = Sechser
category-three-of-a-kind = Dreierpasch
category-four-of-a-kind = Viererpasch
category-full-house = Full House
category-small-straight = Kleine Straße
category-large-straight = Große Straße
category-yatzy = Kniffel
category-chance = Chance

# Game end screen
game-end-title = Spiel beendet!
game-end-new-game = Neues Spiel
game-end-winner = { $rank }: { $name }
game-end-score = { $score } Punkte

# Reconnection overlay
reconnection-message = Würfel-Verbindung verloren!
reconnection-retry = Erneut verbinden
reconnection-dismiss = Ignorieren
reconnection-slot-singular = Würfel { $slot } ist getrennt
reconnection-slot-plural = Würfel { $slots } sind getrennt

# Turn transition overlay
transition-round = Runde { $round }
transition-player = { $name } ist dran!
transition-ready = Bereit!

# Bonus tracker
bonus-title = Bonus-Fortschritt
bonus-subtotal = Obere Hälfte: { $current }/{ $threshold }
bonus-achieved = ✓ Bonus erreicht: +{ $points } Punkte
bonus-reached = Bonus erreicht!
bonus-remaining = Noch { $remaining } Punkte bis zum Bonus

# Hint panel
hint-title = Strategie-Hinweis
hint-category = → { $category }
hint-score = { $score } Punkte
hint-score-zero = 0 Punkte (streichen)
hint-hold-all = Alle Würfel behalten
hint-hold-none = Alle Würfel neu würfeln
hint-hold-some = Behalten: { $values }
hint-no-more-rolls = Kein weiterer Wurf möglich

# Menu
menu-highscore = Highscore
menu-info = Info

# Highscore dialog
highscore-dialog-title = Highscore
highscore-title = Beste Ergebnisse
highscore-empty = Noch keine Highscores vorhanden.
highscore-score = { $score } Punkte
highscore-close = Schließen

# Info dialog
info-dialog-title = Info
info-app-name = Kniffel
info-app-description = Kniffel (Yatzy) für GoDice, gebaut mit GTK 4 und dice-rs
info-github = GitHub Repository
info-docs = docs.rs Dokumentation
info-particula = Particula Tech - GoDice
info-license = Lizenziert unter der MIT-Lizenz
info-close = Schließen
