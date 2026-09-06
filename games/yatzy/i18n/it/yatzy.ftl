# App
app-title = Yatzy

# Game mode
game-mode-single-player = Giocatore singolo
game-mode-multi-player = Multigiocatore

# Game status
game-status-setup = Configurazione
game-status-playing = Partita in corso
game-status-game-over = Partita terminata

# Roll result
roll-result-yatzy = Yatzy!
roll-result-full-house = Full!
roll-result-large-straight = Scala massima!
roll-result-small-straight = Scala minima!
roll-result-four-of-a-kind = Poker!
roll-result-three-of-a-kind = Tris
roll-result-normal = Normale

# Winner rank
winner-rank = { $rank }. posto

# Turn phase
turn-phase-awaiting-roll = In attesa del lancio
turn-phase-rolling = Lancio...
turn-phase-holding = Dadi trattenuti
turn-phase-scoring = Inserimento punteggio
turn-phase-turn-end = Turno terminato

# Round number
round-number = Turno { $current }/{ $total }

# Game phase
game-phase-setup = Configurazione
game-phase-playing = In corso
game-phase-game-over = Partita terminata

# Dice view
dice-tooltip-hold = Dado { $slot } — Clic per trattenere

# Reconnection
reconnection-reconnected = { $count } dadi ricollegati.

# Setup screen
window-setup-title = Yatzy — Configurazione
setup-player-title = Configura giocatori
scan-status-scanning = Ricerca di dadi...
scan-rescan = Cerca di nuovo

# Player setup
player-add = + Aggiungi giocatore
player-start = Avvia partita
player-remove-tooltip = Rimuovi giocatore
player-default-name = Giocatore { $index }
player-color-tooltip = Scegli colore giocatore
player-type-human = Umano
player-type-computer = Computer
player-added = Giocatore aggiunto.
player-max-reached = Numero massimo di giocatori raggiunto.

# Roll button labels
roll-button-roll = Lancia
roll-button-reroll = Lancia di nuovo
roll-button-last = Ultimo lancio
roll-button-none = Nessun lancio rimasto
roll-button-rolling = Lancio...
roll-button-game-over = Partita terminata

# Roll count display
roll-count = Lancio { $current }/3
roll-count-remaining = Lancio { $current }/3
roll-count-zero = Lancio 0/3

# UI status messages
status-ready = Pronto. Cerca dadi per iniziare.
status-scanning = Ricerca di GoDice...
status-no-devices = Nessun GoDice trovato.
status-devices-found = { $count } GoDice trovati, connessione...
status-dice-connected = Dado { $slot } connesso: { $name }
status-all-dice-connected = Tutti i 5 dadi connessi. Pronto per giocare!
status-connection-failed = Connessione fallita per { $name }: { $error }
status-scan-failed = Ricerca fallita: { $error }
status-roll-started = Lancio...
status-roll-complete = Lancio completato. Scegli una categoria.
status-roll-timed-out = Tempo di lancio scaduto.
status-dice-disconnected = Dado { $slot } disconnesso.
status-score-entered = { $player }: { $category } = { $score } punti
status-game-over = Partita terminata!
status-waiting-for-roll = Turno di { $player }. Lancia per iniziare.
status-hold-toggled-held = Dado { $slot } trattenuto
status-hold-toggled-released = Dado { $slot } rilasciato

# Window status messages
game-starting = Avvio partita ({ $count } giocatori)...
game-started-scanning = Partita avviata! Ricerca di GoDice...
error-prefix = Errore: { $error }
reconnecting = Riconnessione...
player-turn = Turno del giocatore { $player }.
score-entered-for-player = { $category } inserito per il giocatore { $player }.
category-crossed-out = { $category } barrato.
category-crossed-out-for-player = { $category } barrato per il giocatore { $player }.
dice-reconnected = { $count } dadi ricollegati.
settings-loaded = Impostazioni caricate ({ $count } giocatori)

# Scan status messages
scan-no-devices = Nessun dado trovato. Nuova ricerca...
scan-devices-found = { $count } dadi trovati.
scan-device-found = Dado { $color } trovato.
scan-connecting = Connessione al dado { $color }...
scan-dice-connected = Dado { $color } connesso.
scan-failed = Ricerca fallita. Riprovo...
scan-all-connected = Tutti i 5 dadi connessi. Pronto per giocare!
scan-swapping = Sostituzione dado { $slot }...
scan-auto-swap = Dado { $slot } non risponde, sostituzione automatica...

# Dice colors
dice-color-black = Nero
dice-color-red = Rosso
dice-color-green = Verde
dice-color-blue = Blu
dice-color-yellow = Giallo
dice-color-orange = Arancione

# Dice swap
dice-swap-tooltip = Clic per sostituire questo dado

# Scorecard
scorecard-upper-section = Sezione superiore
scorecard-lower-section = Sezione inferiore
scorecard-bonus = Bonus
scorecard-subtotal = Subtotale
scorecard-yatzy-bonus = Bonus Yatzy
scorecard-total = Totale

# Categories
category-ones = Uni
category-twos = Due
category-threes = Tre
category-fours = Quattro
category-fives = Cinque
category-sixes = Sei
category-three-of-a-kind = Tris
category-four-of-a-kind = Poker
category-full-house = Full
category-small-straight = Scala minima
category-large-straight = Scala massima
category-yatzy = Yatzy
category-chance = Sorte

# Game end screen
game-end-title = Partita terminata!
game-end-new-game = Nuova partita
game-end-winner = { $rank }: { $name }
game-end-score = { $score } punti

# Reconnection overlay
reconnection-message = Connessione dei dadi persa!
reconnection-retry = Riconnetti
reconnection-dismiss = Ignora
reconnection-slot-singular = Dado { $slot } disconnesso
reconnection-slot-plural = Dadi { $slots } disconnessi

# Turn transition overlay
transition-round = Turno { $round }
transition-player = Tocca a { $name }!
transition-ready = Pronto!

# Bonus tracker
bonus-title = Progresso bonus
bonus-subtotal = Sezione superiore: { $current }/{ $threshold }
bonus-achieved = ✓ Bonus raggiunto: +{ $points } punti
bonus-reached = Bonus raggiunto!
bonus-remaining = { $remaining } punti al bonus

# Hint panel
hint-title = Suggerimento strategico
hint-category = → { $category }
hint-score = { $score } punti
hint-score-zero = 0 punti (barrare)
hint-hold-all = Mantieni tutti i dadi
hint-hold-none = Rilancia tutti i dadi
hint-hold-some = Mantieni: { $values }
hint-no-more-rolls = Nessun altro lancio disponibile

# Menu
menu-highscore = Punteggi migliori
menu-info = Info

# Highscore dialog
highscore-dialog-title = Punteggi migliori
highscore-title = Classifica
highscore-empty = Nessun punteggio ancora.
highscore-score = { $score } punti
highscore-close = Chiudi

# Info dialog
info-dialog-title = Info
info-app-name = Yatzy
info-app-description = Yatzy per GoDice, costruito con GTK 4 e dice-rs
info-github = Repository GitHub
info-docs = Documentazione docs.rs
info-particula = Particula Tech - GoDice
info-license = Licenza MIT
info-close = Chiudi

# Computer AI
ai-thinking = Il computer sta pensando...
ai-rolling = Il computer lancia i dadi.
ai-holding = Il computer trattiene { $count } dadi.
ai-scored = Il computer inserisce { $category } ({ $score } punti).
ai-crossed-out = Il computer barra { $category }.
