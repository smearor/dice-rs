# App
app-title = Yatzy

# Game mode
game-mode-single-player = Один гравець
game-mode-multi-player = Мультиплеєр

# Game status
game-status-setup = Налаштування
game-status-playing = Гра триває
game-status-game-over = Гра завершена

# Roll result
roll-result-yatzy = Ятзи!
roll-result-full-house = Фул-хаус!
roll-result-large-straight = Великий стріт!
roll-result-small-straight = Малий стріт!
roll-result-four-of-a-kind = Каре!
roll-result-three-of-a-kind = Сет
roll-result-normal = Нормальний

# Winner rank
winner-rank = { $rank }. місце

# Turn phase
turn-phase-awaiting-roll = Очікування кидка
turn-phase-rolling = Кидання...
turn-phase-holding = Утримання кубиків
turn-phase-scoring = Запис очок
turn-phase-turn-end = Хід завершено

# Round number
round-number = Раунд { $current }/{ $total }

# Game phase
game-phase-setup = Налаштування
game-phase-playing = Гра триває
game-phase-game-over = Гра завершена

# Dice view
dice-tooltip-hold = Кубик { $slot } — Натисніть, щоб утримати

# Reconnection
reconnection-reconnected = { $count } кубиків перепідключено.

# Setup screen
window-setup-title = Yatzy — Налаштування
setup-player-title = Налаштування гравців
scan-status-scanning = Пошук кубиків...
scan-rescan = Шукати знову

# Player setup
player-add = + Додати гравця
player-start = Почати гру
player-remove-tooltip = Видалити гравця
player-default-name = Гравець { $index }
player-color-tooltip = Вибрати колір гравця
player-type-human = Людина
player-type-computer = Комп'ютер
player-added = Гравця додано.
player-max-reached = Досягнуто максимальної кількості гравців.

# Roll button labels
roll-button-roll = Кинути
roll-button-reroll = Кинути знову
roll-button-last = Останній кидок
roll-button-none = Кидків не залишилося
roll-button-rolling = Кидання...
roll-button-game-over = Гра завершена

# Roll count display
roll-count = Кидок { $current }/3
roll-count-remaining = Кидок { $current }/3
roll-count-zero = Кидок 0/3

# UI status messages
status-ready = Готово. Знайдіть кубики, щоб почати.
status-scanning = Пошук GoDice...
status-no-devices = GoDice не знайдено.
status-devices-found = Знайдено { $count } GoDice, підключення...
status-dice-connected = Кубик { $slot } підключено: { $name }
status-all-dice-connected = Усі 5 кубиків підключено. Готово до гри!
status-connection-failed = Помилка підключення для { $name }: { $error }
status-scan-failed = Помилка пошуку: { $error }
status-roll-started = Кидання...
status-roll-complete = Кидок завершено. Виберіть категорію.
status-roll-timed-out = Час кидка вичерпано.
status-dice-disconnected = Кубик { $slot } відключено.
status-score-entered = { $player }: { $category } = { $score } очок
status-game-over = Гра завершена!
status-waiting-for-roll = Хід гравця { $player }. Киньте, щоб почати.
status-hold-toggled-held = Кубик { $slot } утримано
status-hold-toggled-released = Кубик { $slot } звільнено

# Window status messages
game-starting = Запуск гри ({ $count } гравців)...
game-started-scanning = Гру розпочато! Пошук GoDice...
error-prefix = Помилка: { $error }
reconnecting = Перепідключення...
player-turn = Хід гравця { $player }.
score-entered-for-player = { $category } записано для гравця { $player }.
category-crossed-out = { $category } закреслено.
category-crossed-out-for-player = { $category } закреслено для гравця { $player }.
dice-reconnected = { $count } кубиків перепідключено.
settings-loaded = Налаштування завантажено ({ $count } гравців)

# Scan status messages
scan-no-devices = Кубиків не знайдено. Пошук знову...
scan-devices-found = Знайдено { $count } кубиків.
scan-device-found = Кубик { $color } знайдено.
scan-connecting = Підключення до кубика { $color }...
scan-dice-connected = Кубик { $color } підключено.
scan-failed = Помилка пошуку. Спроба знову...
scan-all-connected = Усі 5 кубиків підключено. Готово до гри!
scan-swapping = Заміна кубика { $slot }...
scan-auto-swap = Кубик { $slot } не відповідає, автоматична заміна...

# Dice colors
dice-color-black = Чорний
dice-color-red = Червоний
dice-color-green = Зелений
dice-color-blue = Синій
dice-color-yellow = Жовтий
dice-color-orange = Помаранчевий

# Dice swap
dice-swap-tooltip = Натисніть, щоб замінити цей кубик

# Scorecard
scorecard-upper-section = Верхня секція
scorecard-lower-section = Нижня секція
scorecard-bonus = Бонус
scorecard-subtotal = Проміжна сума
scorecard-yatzy-bonus = Бонус Ятзи
scorecard-total = Разом

# Categories
category-ones = Одиниці
category-twos = Двійки
category-threes = Трійки
category-fours = Четвірки
category-fives = П'ятірки
category-sixes = Шістки
category-three-of-a-kind = Сет
category-four-of-a-kind = Каре
category-full-house = Фул-хаус
category-small-straight = Малий стріт
category-large-straight = Великий стріт
category-yatzy = Ятзи
category-chance = Шанс

# Game end screen
game-end-title = Гра завершена!
game-end-new-game = Нова гра
game-end-winner = { $rank }: { $name }
game-end-score = { $score } очок

# Reconnection overlay
reconnection-message = Зв'язок з кубиками втрачено!
reconnection-retry = Перепідключити
reconnection-dismiss = Ігнорувати
reconnection-slot-singular = Кубик { $slot } відключено
reconnection-slot-plural = Кубики { $slots } відключено

# Turn transition overlay
transition-round = Раунд { $round }
transition-player = Хід гравця { $name }!
transition-ready = Готово!

# Bonus tracker
bonus-title = Прогрес бонусу
bonus-subtotal = Верхня секція: { $current }/{ $threshold }
bonus-achieved = ✓ Бонус досягнуто: +{ $points } очок
bonus-reached = Бонус досягнуто!
bonus-remaining = { $remaining } очок до бонусу

# Hint panel
hint-title = Стратегічна підказка
hint-category = → { $category }
hint-score = { $score } очок
hint-score-zero = 0 очок (закреслити)
hint-hold-all = Утримати всі кубики
hint-hold-none = Перекинути всі кубики
hint-hold-some = Утримати: { $values }
hint-no-more-rolls = Більше кидків немає

# Menu
menu-highscore = Рекорди
menu-info = Інфо

# Highscore dialog
highscore-dialog-title = Рекорди
highscore-title = Найкращі результати
highscore-empty = Рекордів ще немає.
highscore-score = { $score } очок
highscore-close = Закрити

# Info dialog
info-dialog-title = Інфо
info-app-name = Yatzy
info-app-description = Yatzy для GoDice, створено з GTK 4 та dice-rs
info-github = Репозиторій GitHub
info-docs = Документація docs.rs
info-particula = Particula Tech - GoDice
info-license = Ліцензія MIT
info-close = Закрити

# Computer AI
ai-thinking = Комп'ютер думає...
ai-rolling = Комп'ютер кидає кубики.
ai-holding = Комп'ютер утримує { $count } кубиків.
ai-scored = Комп'ютер записує { $category } ({ $score } очок).
ai-crossed-out = Комп'ютер закреслює { $category }.
