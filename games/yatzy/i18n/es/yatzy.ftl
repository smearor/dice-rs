# App
app-title = Yatzy

# Game mode
game-mode-single-player = Un jugador
game-mode-multi-player = Multijugador

# Game status
game-status-setup = Configuración
game-status-playing = Partida en curso
game-status-game-over = Partida terminada

# Roll result
roll-result-yatzy = ¡Yatzy!
roll-result-full-house = ¡Full!
roll-result-large-straight = ¡Escalera mayor!
roll-result-small-straight = ¡Escalera menor!
roll-result-four-of-a-kind = ¡Póker!
roll-result-three-of-a-kind = Trío
roll-result-normal = Normal

# Winner rank
winner-rank = { $rank }. puesto

# Turn phase
turn-phase-awaiting-roll = Esperando tirada
turn-phase-rolling = Tirando...
turn-phase-holding = Manteniendo dados
turn-phase-scoring = Anotando puntos
turn-phase-turn-end = Turno terminado

# Round number
round-number = Ronda { $current }/{ $total }

# Game phase
game-phase-setup = Configuración
game-phase-playing = En curso
game-phase-game-over = Partida terminada

# Dice view
dice-tooltip-hold = Dado { $slot } — Clic para mantener

# Reconnection
reconnection-reconnected = { $count } dados reconectados.

# Setup screen
window-setup-title = Yatzy — Configuración
setup-player-title = Configurar jugadores
scan-status-scanning = Buscando dados...
scan-rescan = Buscar de nuevo

# Player setup
player-add = + Añadir jugador
player-start = Iniciar partida
player-remove-tooltip = Eliminar jugador
player-default-name = Jugador { $index }
player-color-tooltip = Elegir color del jugador
player-type-human = Humano
player-type-computer = Ordenador
player-added = Jugador añadido.
player-max-reached = Número máximo de jugadores alcanzado.

# Roll button labels
roll-button-roll = Tirar
roll-button-reroll = Tirar de nuevo
roll-button-last = Última tirada
roll-button-none = Sin tiradas
roll-button-rolling = Tirando...
roll-button-game-over = Partida terminada

# Roll count display
roll-count = Tirada { $current }/3
roll-count-remaining = Tirada { $current }/3
roll-count-zero = Tirada 0/3

# UI status messages
status-ready = Listo. Busca dados para empezar.
status-scanning = Buscando GoDice...
status-no-devices = No se encontraron GoDice.
status-devices-found = { $count } GoDice encontrados, conectando...
status-dice-connected = Dado { $slot } conectado: { $name }
status-all-dice-connected = Los 5 dados conectados. ¡Listo para jugar!
status-connection-failed = Conexión fallida para { $name }: { $error }
status-scan-failed = Búsqueda fallida: { $error }
status-roll-started = Tirando...
status-roll-complete = Tirada completa. Elige una categoría.
status-roll-timed-out = Tiempo de tirada agotado.
status-dice-disconnected = Dado { $slot } desconectado.
status-score-entered = { $player }: { $category } = { $score } puntos
status-game-over = ¡Partida terminada!
status-waiting-for-roll = Turno de { $player }. Tira para empezar.
status-hold-toggled-held = Dado { $slot } mantenido
status-hold-toggled-released = Dado { $slot } soltado

# Window status messages
game-starting = Iniciando partida ({ $count } jugadores)...
game-started-scanning = ¡Partida iniciada! Buscando GoDice...
error-prefix = Error: { $error }
reconnecting = Reconectando...
player-turn = Turno del jugador { $player }.
score-entered-for-player = { $category } anotado para el jugador { $player }.
category-crossed-out = { $category } tachado.
category-crossed-out-for-player = { $category } tachado para el jugador { $player }.
dice-reconnected = { $count } dados reconectados.
settings-loaded = Ajustes cargados ({ $count } jugadores)

# Scan status messages
scan-no-devices = No se encontraron dados. Buscando de nuevo...
scan-devices-found = { $count } dados encontrados.
scan-device-found = Dado { $color } encontrado.
scan-connecting = Conectando al dado { $color }...
scan-dice-connected = Dado { $color } conectado.
scan-failed = Búsqueda fallida. Reintentando...
scan-all-connected = Los 5 dados conectados. ¡Listo para jugar!
scan-swapping = Cambiando dado { $slot }...
scan-auto-swap = Dado { $slot } sin respuesta, cambio automático...

# Dice colors
dice-color-black = Negro
dice-color-red = Rojo
dice-color-green = Verde
dice-color-blue = Azul
dice-color-yellow = Amarillo
dice-color-orange = Naranja

# Dice swap
dice-swap-tooltip = Clic para cambiar este dado

# Scorecard
scorecard-upper-section = Sección superior
scorecard-lower-section = Sección inferior
scorecard-bonus = Bonus
scorecard-subtotal = Subtotal
scorecard-yatzy-bonus = Bonus Yatzy
scorecard-total = Total

# Categories
category-ones = Unos
category-twos = Doses
category-threes = Treses
category-fours = Cuatros
category-fives = Cincos
category-sixes = Seises
category-three-of-a-kind = Trío
category-four-of-a-kind = Póker
category-full-house = Full
category-small-straight = Escalera menor
category-large-straight = Escalera mayor
category-yatzy = Yatzy
category-chance = Suerte

# Game end screen
game-end-title = ¡Partida terminada!
game-end-new-game = Nueva partida
game-end-winner = { $rank }: { $name }
game-end-score = { $score } puntos

# Reconnection overlay
reconnection-message = ¡Conexión de dados perdida!
reconnection-retry = Reconectar
reconnection-dismiss = Ignorar
reconnection-slot-singular = Dado { $slot } desconectado
reconnection-slot-plural = Dados { $slots } desconectados

# Turn transition overlay
transition-round = Ronda { $round }
transition-player = ¡Turno de { $name }!
transition-ready = ¡Listo!

# Bonus tracker
bonus-title = Progreso del bonus
bonus-subtotal = Sección superior: { $current }/{ $threshold }
bonus-achieved = ✓ Bonus alcanzado: +{ $points } puntos
bonus-reached = ¡Bonus alcanzado!
bonus-remaining = { $remaining } puntos hasta el bonus

# Hint panel
hint-title = Consejo de estrategia
hint-category = → { $category }
hint-score = { $score } puntos
hint-score-zero = 0 puntos (tachar)
hint-hold-all = Mantener todos los dados
hint-hold-none = Relanzar todos los dados
hint-hold-some = Mantener: { $values }
hint-no-more-rolls = No hay más tiradas disponibles

# Menu
menu-highscore = Puntuaciones máximas
menu-info = Info

# Highscore dialog
highscore-dialog-title = Puntuaciones máximas
highscore-title = Ranking
highscore-empty = Aún no hay puntuaciones.
highscore-score = { $score } puntos
highscore-close = Cerrar

# Info dialog
info-dialog-title = Info
info-app-name = Yatzy
info-app-description = Yatzy para GoDice, construido con GTK 4 y dice-rs
info-github = Repositorio de GitHub
info-docs = Documentación de docs.rs
info-particula = Particula Tech - GoDice
info-license = Licencia MIT
info-close = Cerrar

# Computer AI
ai-thinking = El ordenador está pensando...
ai-rolling = El ordenador tira los dados.
ai-holding = El ordenador mantiene { $count } dados.
ai-scored = El ordenador anota { $category } ({ $score } puntos).
ai-crossed-out = El ordenador tacha { $category }.
