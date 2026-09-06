# App
app-title = Yatzy

# Game mode
game-mode-single-player = Solo
game-mode-multi-player = Multijoueur

# Game status
game-status-setup = Configuration
game-status-playing = Partie en cours
game-status-game-over = Partie terminée

# Roll result
roll-result-yatzy = Yatzy !
roll-result-full-house = Full !
roll-result-large-straight = Grande suite !
roll-result-small-straight = Petite suite !
roll-result-four-of-a-kind = Carré !
roll-result-three-of-a-kind = Brelan
roll-result-normal = Normal

# Winner rank
winner-rank = { $rank }. place

# Turn phase
turn-phase-awaiting-roll = En attente du lancer
turn-phase-rolling = Lancer...
turn-phase-holding = Dés retenus
turn-phase-scoring = Saisie du score
turn-phase-turn-end = Tour terminé

# Round number
round-number = Manche { $current }/{ $total }

# Game phase
game-phase-setup = Configuration
game-phase-playing = En cours
game-phase-game-over = Partie terminée

# Dice view
dice-tooltip-hold = Dé { $slot } — Cliquez pour retenir

# Reconnection
reconnection-reconnected = { $count } dés reconnectés.

# Setup screen
window-setup-title = Yatzy — Configuration
setup-player-title = Configurer les joueurs
scan-status-scanning = Recherche de dés...
scan-rescan = Rechercher à nouveau

# Player setup
player-add = + Ajouter un joueur
player-start = Lancer la partie
player-remove-tooltip = Retirer le joueur
player-default-name = Joueur { $index }
player-color-tooltip = Choisir la couleur du joueur
player-type-human = Humain
player-type-computer = Ordinateur
player-added = Joueur ajouté.
player-max-reached = Nombre maximum de joueurs atteint.

# Roll button labels
roll-button-roll = Lancer
roll-button-reroll = Relancer
roll-button-last = Dernier lancer
roll-button-none = Plus de lancer
roll-button-rolling = Lancer...
roll-button-game-over = Partie terminée

# Roll count display
roll-count = Lancer { $current }/3
roll-count-remaining = Lancer { $current }/3
roll-count-zero = Lancer 0/3

# UI status messages
status-ready = Prêt. Recherchez des dés pour commencer.
status-scanning = Recherche de GoDice...
status-no-devices = Aucun GoDice trouvé.
status-devices-found = { $count } GoDice trouvés, connexion...
status-dice-connected = Dé { $slot } connecté : { $name }
status-all-dice-connected = Les 5 dés sont connectés. Prêt à jouer !
status-connection-failed = Échec de connexion pour { $name } : { $error }
status-scan-failed = Échec de la recherche : { $error }
status-roll-started = Lancer...
status-roll-complete = Lancer terminé. Choisissez une catégorie.
status-roll-timed-out = Temps de lancer écoulé.
status-dice-disconnected = Dé { $slot } déconnecté.
status-score-entered = { $player } : { $category } = { $score } points
status-game-over = Partie terminée !
status-waiting-for-roll = Tour de { $player }. Lancez pour commencer.
status-hold-toggled-held = Dé { $slot } retenu
status-hold-toggled-released = Dé { $slot } relâché

# Window status messages
game-starting = Démarrage de la partie ({ $count } joueurs)...
game-started-scanning = Partie démarrée ! Recherche de GoDice...
error-prefix = Erreur : { $error }
reconnecting = Reconnexion...
player-turn = Tour du joueur { $player }.
score-entered-for-player = { $category } inscrit pour le joueur { $player }.
category-crossed-out = { $category } barré.
category-crossed-out-for-player = { $category } barré pour le joueur { $player }.
dice-reconnected = { $count } dés reconnectés.
settings-loaded = Paramètres chargés ({ $count } joueurs)

# Scan status messages
scan-no-devices = Aucun dé trouvé. Nouvelle recherche...
scan-devices-found = { $count } dés trouvés.
scan-device-found = Dé { $color } trouvé.
scan-connecting = Connexion au dé { $color }...
scan-dice-connected = Dé { $color } connecté.
scan-failed = Échec de la recherche. Nouvelle tentative...
scan-all-connected = Les 5 dés sont connectés. Prêt à jouer !
scan-swapping = Remplacement du dé { $slot }...
scan-auto-swap = Dé { $slot } sans réponse, remplacement automatique...

# Dice colors
dice-color-black = Noir
dice-color-red = Rouge
dice-color-green = Vert
dice-color-blue = Bleu
dice-color-yellow = Jaune
dice-color-orange = Orange

# Dice swap
dice-swap-tooltip = Cliquez pour remplacer ce dé

# Scorecard
scorecard-upper-section = Section supérieure
scorecard-lower-section = Section inférieure
scorecard-bonus = Bonus
scorecard-subtotal = Sous-total
scorecard-yatzy-bonus = Bonus Yatzy
scorecard-total = Total

# Categories
category-ones = As
category-twos = Deux
category-threes = Trois
category-fours = Quatre
category-fives = Cinq
category-sixes = Six
category-three-of-a-kind = Brelan
category-four-of-a-kind = Carré
category-full-house = Full
category-small-straight = Petite suite
category-large-straight = Grande suite
category-yatzy = Yatzy
category-chance = Chance

# Game end screen
game-end-title = Partie terminée !
game-end-new-game = Nouvelle partie
game-end-winner = { $rank } : { $name }
game-end-score = { $score } points

# Reconnection overlay
reconnection-message = Connexion des dés perdue !
reconnection-retry = Reconnecter
reconnection-dismiss = Ignorer
reconnection-slot-singular = Dé { $slot } déconnecté
reconnection-slot-plural = Dés { $slots } déconnectés

# Turn transition overlay
transition-round = Manche { $round }
transition-player = Tour de { $name } !
transition-ready = Prêt !

# Bonus tracker
bonus-title = Progression du bonus
bonus-subtotal = Section supérieure : { $current }/{ $threshold }
bonus-achieved = ✓ Bonus atteint : +{ $points } points
bonus-reached = Bonus atteint !
bonus-remaining = { $remaining } points jusqu'au bonus

# Hint panel
hint-title = Conseil de stratégie
hint-category = → { $category }
hint-score = { $score } points
hint-score-zero = 0 point (barrer)
hint-hold-all = Garder tous les dés
hint-hold-none = Relancer tous les dés
hint-hold-some = Garder : { $values }
hint-no-more-rolls = Plus de lancer disponible

# Menu
menu-highscore = Meilleurs scores
menu-info = Info

# Highscore dialog
highscore-dialog-title = Meilleurs scores
highscore-title = Classement
highscore-empty = Aucun meilleur score pour l'instant.
highscore-score = { $score } points
highscore-close = Fermer

# Info dialog
info-dialog-title = Info
info-app-name = Yatzy
info-app-description = Yatzy pour GoDice, conçu avec GTK 4 et dice-rs
info-github = Dépôt GitHub
info-docs = Documentation docs.rs
info-particula = Particula Tech - GoDice
info-license = Sous licence MIT
info-close = Fermer

# Computer AI
ai-thinking = L'ordinateur réfléchit...
ai-rolling = L'ordinateur lance les dés.
ai-holding = L'ordinateur retient { $count } dés.
ai-scored = L'ordinateur inscrit { $category } ({ $score } points).
ai-crossed-out = L'ordinateur barre { $category }.
