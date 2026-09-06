# App
app-title = Yatzy

# Game mode
game-mode-single-player = Um jogador
game-mode-multi-player = Multijogador

# Game status
game-status-setup = Configuração
game-status-playing = Partida em andamento
game-status-game-over = Partida encerrada

# Roll result
roll-result-yatzy = Yatzy!
roll-result-full-house = Full House!
roll-result-large-straight = Sequência maior!
roll-result-small-straight = Sequência menor!
roll-result-four-of-a-kind = Quadra!
roll-result-three-of-a-kind = Trinca
roll-result-normal = Normal

# Winner rank
winner-rank = { $rank }.º lugar

# Turn phase
turn-phase-awaiting-roll = Aguardando lançamento
turn-phase-rolling = Lançando...
turn-phase-holding = Segurando dados
turn-phase-scoring = Registrando pontuação
turn-phase-turn-end = Turno encerrado

# Round number
round-number = Rodada { $current }/{ $total }

# Game phase
game-phase-setup = Configuração
game-phase-playing = Em andamento
game-phase-game-over = Partida encerrada

# Dice view
dice-tooltip-hold = Dado { $slot } — Clique para segurar

# Reconnection
reconnection-reconnected = { $count } dados reconectados.

# Setup screen
window-setup-title = Yatzy — Configuração
setup-player-title = Configurar jogadores
scan-status-scanning = Procurando dados...
scan-rescan = Procurar novamente

# Player setup
player-add = + Adicionar jogador
player-start = Iniciar partida
player-remove-tooltip = Remover jogador
player-default-name = Jogador { $index }
player-color-tooltip = Escolher cor do jogador
player-type-human = Humano
player-type-computer = Computador
player-added = Jogador adicionado.
player-max-reached = Número máximo de jogadores atingido.

# Roll button labels
roll-button-roll = Lançar
roll-button-reroll = Lançar novamente
roll-button-last = Último lançamento
roll-button-none = Sem lançamentos
roll-button-rolling = Lançando...
roll-button-game-over = Partida encerrada

# Roll count display
roll-count = Lançamento { $current }/3
roll-count-remaining = Lançamento { $current }/3
roll-count-zero = Lançamento 0/3

# UI status messages
status-ready = Pronto. Procure dados para começar.
status-scanning = Procurando GoDice...
status-no-devices = Nenhum GoDice encontrado.
status-devices-found = { $count } GoDice encontrados, conectando...
status-dice-connected = Dado { $slot } conectado: { $name }
status-all-dice-connected = Todos os 5 dados conectados. Pronto para jogar!
status-connection-failed = Falha na conexão para { $name }: { $error }
status-scan-failed = Falha na busca: { $error }
status-roll-started = Lançando...
status-roll-complete = Lançamento completo. Escolha uma categoria.
status-roll-timed-out = Tempo de lançamento esgotado.
status-dice-disconnected = Dado { $slot } desconectado.
status-score-entered = { $player }: { $category } = { $score } pontos
status-game-over = Partida encerrada!
status-waiting-for-roll = Vez de { $player }. Lance para começar.
status-hold-toggled-held = Dado { $slot } segurado
status-hold-toggled-released = Dado { $slot } solto

# Window status messages
game-starting = Iniciando partida ({ $count } jogadores)...
game-started-scanning = Partida iniciada! Procurando GoDice...
error-prefix = Erro: { $error }
reconnecting = Reconectando...
player-turn = Vez do jogador { $player }.
score-entered-for-player = { $category } registrado para o jogador { $player }.
category-crossed-out = { $category } riscado.
category-crossed-out-for-player = { $category } riscado para o jogador { $player }.
dice-reconnected = { $count } dados reconectados.
settings-loaded = Configurações carregadas ({ $count } jogadores)

# Scan status messages
scan-no-devices = Nenhum dado encontrado. Buscando novamente...
scan-devices-found = { $count } dados encontrados.
scan-device-found = Dado { $color } encontrado.
scan-connecting = Conectando ao dado { $color }...
scan-dice-connected = Dado { $color } conectado.
scan-failed = Falha na busca. Tentando novamente...
scan-all-connected = Todos os 5 dados conectados. Pronto para jogar!
scan-swapping = Trocando dado { $slot }...
scan-auto-swap = Dado { $slot } sem resposta, troca automática...

# Dice colors
dice-color-black = Preto
dice-color-red = Vermelho
dice-color-green = Verde
dice-color-blue = Azul
dice-color-yellow = Amarelo
dice-color-orange = Laranja

# Dice swap
dice-swap-tooltip = Clique para trocar este dado

# Scorecard
scorecard-upper-section = Seção superior
scorecard-lower-section = Seção inferior
scorecard-bonus = Bônus
scorecard-subtotal = Subtotal
scorecard-yatzy-bonus = Bônus Yatzy
scorecard-total = Total

# Categories
category-ones = Uns
category-twos = Dois
category-threes = Três
category-fours = Quatros
category-fives = Cincos
category-sixes = Seis
category-three-of-a-kind = Trinca
category-four-of-a-kind = Quadra
category-full-house = Full House
category-small-straight = Sequência menor
category-large-straight = Sequência maior
category-yatzy = Yatzy
category-chance = Sorte

# Game end screen
game-end-title = Partida encerrada!
game-end-new-game = Nova partida
game-end-winner = { $rank }: { $name }
game-end-score = { $score } pontos

# Reconnection overlay
reconnection-message = Conexão dos dados perdida!
reconnection-retry = Reconectar
reconnection-dismiss = Ignorar
reconnection-slot-singular = Dado { $slot } desconectado
reconnection-slot-plural = Dados { $slots } desconectados

# Turn transition overlay
transition-round = Rodada { $round }
transition-player = Vez de { $name }!
transition-ready = Pronto!

# Bonus tracker
bonus-title = Progresso do bônus
bonus-subtotal = Seção superior: { $current }/{ $threshold }
bonus-achieved = ✓ Bônus atingido: +{ $points } pontos
bonus-reached = Bônus atingido!
bonus-remaining = { $remaining } pontos até o bônus

# Hint panel
hint-title = Dica de estratégia
hint-category = → { $category }
hint-score = { $score } pontos
hint-score-zero = 0 pontos (riscar)
hint-hold-all = Manter todos os dados
hint-hold-none = Relançar todos os dados
hint-hold-some = Manter: { $values }
hint-no-more-rolls = Sem mais lançamentos disponíveis

# Menu
menu-highscore = Recordes
menu-info = Info

# Highscore dialog
highscore-dialog-title = Recordes
highscore-title = Ranking
highscore-empty = Ainda não há recordes.
highscore-score = { $score } pontos
highscore-close = Fechar

# Info dialog
info-dialog-title = Info
info-app-name = Yatzy
info-app-description = Yatzy para GoDice, construído com GTK 4 e dice-rs
info-github = Repositório GitHub
info-docs = Documentação docs.rs
info-particula = Particula Tech - GoDice
info-license = Licença MIT
info-close = Fechar

# Computer AI
ai-thinking = O computador está pensando...
ai-rolling = O computador lança os dados.
ai-holding = O computador segura { $count } dados.
ai-scored = O computador registra { $category } ({ $score } pontos).
ai-crossed-out = O computador risca { $category }.
