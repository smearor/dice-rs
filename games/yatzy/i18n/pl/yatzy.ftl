# App
app-title = Yatzy

# Game mode
game-mode-single-player = Gra pojedyncza
game-mode-multi-player = Wieloosobowa

# Game status
game-status-setup = Konfiguracja
game-status-playing = Gra w toku
game-status-game-over = Koniec gry

# Roll result
roll-result-yatzy = Yatzy!
roll-result-full-house = Ful!
roll-result-large-straight = Duży strit!
roll-result-small-straight = Mały strit!
roll-result-four-of-a-kind = Kareta!
roll-result-three-of-a-kind = Trójka
roll-result-normal = Normalny

# Winner rank
winner-rank = { $rank }. miejsce

# Turn phase
turn-phase-awaiting-roll = Oczekiwanie na rzut
turn-phase-rolling = Rzucanie...
turn-phase-holding = Przytrzymywanie kości
turn-phase-scoring = Wprowadzanie wyniku
turn-phase-turn-end = Tura zakończona

# Round number
round-number = Runda { $current }/{ $total }

# Game phase
game-phase-setup = Konfiguracja
game-phase-playing = Gra w toku
game-phase-game-over = Koniec gry

# Dice view
dice-tooltip-hold = Kość { $slot } — Kliknij, aby przytrzymać

# Reconnection
reconnection-reconnected = { $count } kości ponownie połączonych.

# Setup screen
window-setup-title = Yatzy — Konfiguracja
setup-player-title = Skonfiguruj graczy
scan-status-scanning = Szukanie kości...
scan-rescan = Szukaj ponownie

# Player setup
player-add = + Dodaj gracza
player-start = Rozpocznij grę
player-remove-tooltip = Usuń gracza
player-default-name = Gracz { $index }
player-color-tooltip = Wybierz kolor gracza
player-type-human = Człowiek
player-type-computer = Komputer
player-added = Gracz dodany.
player-max-reached = Osiągnięto maksymalną liczbę graczy.

# Roll button labels
roll-button-roll = Rzuć
roll-button-reroll = Rzuć ponownie
roll-button-last = Ostatni rzut
roll-button-none = Brak rzutów
roll-button-rolling = Rzucanie...
roll-button-game-over = Koniec gry

# Roll count display
roll-count = Rzut { $current }/3
roll-count-remaining = Rzut { $current }/3
roll-count-zero = Rzut 0/3

# UI status messages
status-ready = Gotowe. Szukaj kości, aby rozpocząć.
status-scanning = Szukanie GoDice...
status-no-devices = Nie znaleziono GoDice.
status-devices-found = Znaleziono { $count } GoDice, łączenie...
status-dice-connected = Kość { $slot } połączona: { $name }
status-all-dice-connected = Wszystkie 5 kości połączonych. Gotowe do gry!
status-connection-failed = Błąd połączenia dla { $name }: { $error }
status-scan-failed = Błąd wyszukiwania: { $error }
status-roll-started = Rzucanie...
status-roll-complete = Rzut zakończony. Wybierz kategorię.
status-roll-timed-out = Przekroczono czas rzutu.
status-dice-disconnected = Kość { $slot } rozłączona.
status-score-entered = { $player }: { $category } = { $score } punktów
status-game-over = Koniec gry!
status-waiting-for-roll = Tura gracza { $player }. Rzuć, aby rozpocząć.
status-hold-toggled-held = Kość { $slot } przytrzymana
status-hold-toggled-released = Kość { $slot } zwolniona

# Window status messages
game-starting = Rozpoczynanie gry ({ $count } graczy)...
game-started-scanning = Gra rozpoczęta! Szukanie GoDice...
error-prefix = Błąd: { $error }
reconnecting = Ponowne łączenie...
player-turn = Tura gracza { $player }.
score-entered-for-player = { $category } wpisana dla gracza { $player }.
category-crossed-out = { $category } skreślona.
category-crossed-out-for-player = { $category } skreślona dla gracza { $player }.
dice-reconnected = { $count } kości ponownie połączonych.
settings-loaded = Ustawienia załadowane ({ $count } graczy)

# Scan status messages
scan-no-devices = Nie znaleziono kości. Ponowne szukanie...
scan-devices-found = Znaleziono { $count } kości.
scan-device-found = Kość { $color } znaleziona.
scan-connecting = Łączenie z kością { $color }...
scan-dice-connected = Kość { $color } połączona.
scan-failed = Błąd wyszukiwania. Ponowna próba...
scan-all-connected = Wszystkie 5 kości połączonych. Gotowe do gry!
scan-swapping = Wymiana kości { $slot }...
scan-auto-swap = Kość { $slot } nie odpowiada, automatyczna wymiana...

# Dice colors
dice-color-black = Czarny
dice-color-red = Czerwony
dice-color-green = Zielony
dice-color-blue = Niebieski
dice-color-yellow = Żółty
dice-color-orange = Pomarańczowy

# Dice swap
dice-swap-tooltip = Kliknij, aby wymienić tę kość

# Scorecard
scorecard-upper-section = Sekcja górna
scorecard-lower-section = Sekcja dolna
scorecard-bonus = Bonus
scorecard-subtotal = Suma częściowa
scorecard-yatzy-bonus = Bonus Yatzy
scorecard-total = Razem

# Categories
category-ones = Jedynki
category-twos = Dwójki
category-threes = Trójki
category-fours = Czwórki
category-fives = Piątki
category-sixes = Szóstki
category-three-of-a-kind = Trójka
category-four-of-a-kind = Kareta
category-full-house = Ful
category-small-straight = Mały strit
category-large-straight = Duży strit
category-yatzy = Yatzy
category-chance = Szansa

# Game end screen
game-end-title = Koniec gry!
game-end-new-game = Nowa gra
game-end-winner = { $rank }: { $name }
game-end-score = { $score } punktów

# Reconnection overlay
reconnection-message = Utracono połączenie z kośćmi!
reconnection-retry = Połącz ponownie
reconnection-dismiss = Ignoruj
reconnection-slot-singular = Kość { $slot } rozłączona
reconnection-slot-plural = Kości { $slots } rozłączone

# Turn transition overlay
transition-round = Runda { $round }
transition-player = Tura gracza { $name }!
transition-ready = Gotowe!

# Bonus tracker
bonus-title = Postęp bonusu
bonus-subtotal = Sekcja górna: { $current }/{ $threshold }
bonus-achieved = ✓ Bonus osiągnięty: +{ $points } punktów
bonus-reached = Bonus osiągnięty!
bonus-remaining = { $remaining } punktów do bonusu

# Hint panel
hint-title = Wskazówka strategiczna
hint-category = → { $category }
hint-score = { $score } punktów
hint-score-zero = 0 punktów (skreśl)
hint-hold-all = Zachowaj wszystkie kości
hint-hold-none = Przerzuć wszystkie kości
hint-hold-some = Zachowaj: { $values }
hint-no-more-rolls = Brak dostępnych rzutów

# Menu
menu-highscore = Najlepsze wyniki
menu-info = Info

# Highscore dialog
highscore-dialog-title = Najlepsze wyniki
highscore-title = Ranking
highscore-empty = Brak najlepszych wyników.
highscore-score = { $score } punktów
highscore-close = Zamknij

# Info dialog
info-dialog-title = Info
info-app-name = Yatzy
info-app-description = Yatzy dla GoDice, zbudowane z GTK 4 i dice-rs
info-github = Repozytorium GitHub
info-docs = Dokumentacja docs.rs
info-particula = Particula Tech - GoDice
info-license = Licencja MIT
info-close = Zamknij

# Computer AI
ai-thinking = Komputer myśli...
ai-rolling = Komputer rzuca kośćmi.
ai-holding = Komputer przytrzymuje { $count } kości.
ai-scored = Komputer wpisuje { $category } ({ $score } punktów).
ai-crossed-out = Komputer skreśla { $category }.
