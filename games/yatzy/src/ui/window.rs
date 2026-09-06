use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use dice_rs::DiceManager;
use gtk4::glib;
use gtk4::prelude::*;
use tracing::debug;
use tracing::warn;

use crate::i18n;
use crate::models::dice_set::DiceSet;
use crate::models::dice_slot::DiceSlot;
use crate::models::game_state::GameState;
use crate::models::highscore::HighscoreList;
use crate::models::highscore_entry::HighscoreEntry;
use crate::models::hold_mask::HoldMask;
use crate::models::player::Player;
use crate::models::player_color::PlayerColor;
use crate::models::player_index::PlayerIndex;
use crate::models::reconnection_request::ReconnectionRequest;
use crate::models::turn_action::TurnAction;
use crate::services::dice_service::DiceService;
use crate::services::dice_service::DiceServiceEvent;
use crate::services::event_bridge::EventBridge;
use crate::services::event_bridge::GameEvent;
use crate::services::game_controller::ControllerEvent;
use crate::services::game_controller::GameController;
use crate::services::highscore_store::HighscoreStore;
use crate::services::led_effect::LedEffect;
use crate::services::led_service::LedService;
use crate::services::reconnection_manager::ReconnectionManager;
use crate::services::roll_detector::RollDetector;
use crate::services::settings_store::SettingsStore;
use crate::services::slot_mapping::SlotMapping;
use crate::ui::models::game_phase::GamePhase;
use crate::ui::models::roll_button_label::RollButtonLabel;
use crate::ui::models::ui_message::UiMessage;
use crate::ui::widgets::dice_status_widget::DiceStatusWidget;
use crate::ui::widgets::dice_view::DiceView;
use crate::ui::widgets::game_end_screen::GameEndScreen;
use crate::ui::widgets::highscore_dialog::HighscoreDialog;
use crate::ui::widgets::info_dialog::InfoDialog;
use crate::ui::widgets::player_bar::PlayerBar;
use crate::ui::widgets::player_setup::PlayerSetupWidget;
use crate::ui::widgets::reconnection_overlay::ReconnectionOverlay;
use crate::ui::widgets::scorecard_view::ScorecardView;
use crate::ui::widgets::turn_panel::TurnPanel;

/// The main application window.
///
/// Orchestrates all UI widgets and bridges them to the game engine
/// and BLE services. The window has three phases: Setup (scanning
/// for dice), Playing (active gameplay), and GameOver (final scores).
pub struct MainWindow {
    /// The GTK window.
    window: gtk4::ApplicationWindow,
    /// The dice-rs BLE manager (retained for future reconnection logic).
    #[allow(dead_code)]
    manager: Arc<DiceManager>,
    /// Slot mapping for physical dice.
    mapping: SlotMapping,
    /// Dice service for scanning and connecting.
    dice_service: DiceService,
    /// Roll detector for tracking roll completion.
    roll_detector: RollDetector,
    /// Event bridge for BLE → game events.
    event_bridge: EventBridge,
    /// LED service for physical feedback.
    led_service: LedService,
    /// Reconnection manager for tracking disconnected dice.
    reconnection_manager: Rc<ReconnectionManager>,
    /// The game controller (set when a game starts).
    game_controller: Rc<RefCell<Option<GameController>>>,
    /// Settings store for persisting game settings.
    settings_store: Rc<SettingsStore>,
    /// Highscore store for persisting highscore data.
    highscore_store: Rc<HighscoreStore>,
    /// The game state.
    game_state: Rc<RefCell<Option<GameState>>>,
    /// Current UI phase.
    phase: Rc<RefCell<GamePhase>>,
    /// The player bar widget.
    player_bar: Rc<PlayerBar>,
    /// The dice view widget.
    dice_view: Rc<DiceView>,
    /// The scorecard view widget.
    scorecard_view: Rc<ScorecardView>,
    /// The turn panel widget.
    turn_panel: Rc<TurnPanel>,
    /// The game end screen widget.
    game_end_screen: Rc<GameEndScreen>,
    /// The reconnection overlay widget.
    reconnection_overlay: Rc<ReconnectionOverlay>,
    /// The revealer for the reconnection overlay (show/hide).
    reconnection_revealer: gtk4::Revealer,
    /// The player setup widget for configuring players before game start.
    player_setup: Rc<RefCell<PlayerSetupWidget>>,
    /// Current hold mask (tracked from dice view toggles).
    holds: Rc<RefCell<HoldMask>>,
    /// Dice slots that were picked up during Holding phase (for auto-hold).
    picked_up_dice: Rc<RefCell<Vec<DiceSlot>>>,
    /// Timer ID for the pickup collection window.
    pickup_timer: Rc<RefCell<Option<glib::SourceId>>>,
    /// Timer ID for the periodic reconnection attempt.
    reconnection_timer: Rc<RefCell<Option<glib::SourceId>>>,
    /// Auto-swap timers per slot: if no Stable event arrives within the
    /// timeout, the dice is automatically swapped for another.
    auto_swap_timers: Rc<RefCell<[Option<glib::SourceId>; 5]>>,
    /// Tracks which slots have received a Stable event since connection.
    slot_verified: Rc<RefCell<[bool; 5]>>,
    /// The status label.
    status_label: gtk4::Label,
    /// The progress bar for the scanning/connecting phase.
    scan_progress_bar: gtk4::ProgressBar,
    /// The status text label above the progress bar.
    scan_status_label: gtk4::Label,
    /// The dice status widget for the setup screen.
    dice_status_widget: Rc<DiceStatusWidget>,
    /// The scan button (setup phase).
    scan_button: gtk4::Button,
    /// The main content stack.
    content_stack: gtk4::Stack,
    /// Receiver for the start-game channel (set in connect_signals).
    start_game_receiver: Rc<RefCell<Option<std::sync::mpsc::Receiver<Vec<Player>>>>>,
}

impl MainWindow {
    /// Create the main window.
    pub fn new(app: &gtk4::Application, manager: Arc<DiceManager>) -> Self {
        let mapping = SlotMapping::new();
        let dice_service = DiceService::new(manager.clone(), mapping.clone());
        let roll_detector = RollDetector::new();
        let event_bridge = EventBridge::new(mapping.clone(), roll_detector.clone());
        let led_service = LedService::new(mapping.clone());

        // Setup screen
        let scan_button = gtk4::Button::builder()
            .css_classes(vec!["scan-button", "suggested-action"])
            .label(&i18n::get("scan-rescan"))
            .halign(gtk4::Align::Center)
            .build();

        let setup_title = gtk4::Label::builder()
            .label(&i18n::get("window-setup-title"))
            .css_classes(vec!["setup-title"])
            .halign(gtk4::Align::Center)
            .build();

        let scan_status_label = gtk4::Label::builder()
            .label(&i18n::get("scan-status-scanning"))
            .css_classes(vec!["scan-status-label"])
            .halign(gtk4::Align::Center)
            .build();

        let scan_progress_bar = gtk4::ProgressBar::builder()
            .css_classes(vec!["scan-progress"])
            .halign(gtk4::Align::Center)
            .width_request(300)
            .build();

        let status_label = gtk4::Label::builder()
            .label(UiMessage::scanning().as_str())
            .css_classes(vec!["status-label", "dim"])
            .halign(gtk4::Align::Center)
            .build();

        let dice_status_widget = Rc::new(DiceStatusWidget::new());

        let setup_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .css_classes(vec!["setup-screen"])
            .spacing(16)
            .halign(gtk4::Align::Center)
            .valign(gtk4::Align::Center)
            .build();

        let player_setup = Rc::new(RefCell::new(PlayerSetupWidget::new()));
        // Disable start button until all 5 dice are connected
        player_setup.borrow().set_start_button_sensitive(false);

        setup_box.append(&setup_title);
        setup_box.append(&scan_status_label);
        setup_box.append(&scan_progress_bar);
        setup_box.append(dice_status_widget.widget());
        setup_box.append(&scan_button);
        setup_box.append(player_setup.borrow().widget());

        // Playing screen
        let player_bar = Rc::new(PlayerBar::new());
        let dice_view = Rc::new(DiceView::new());
        let scorecard_view = Rc::new(ScorecardView::new());
        let turn_panel = Rc::new(TurnPanel::new());
        let reconnection_overlay = Rc::new(ReconnectionOverlay::new());

        let playing_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(12)
            .margin_start(12)
            .margin_end(12)
            .margin_top(12)
            .margin_bottom(12)
            .build();

        playing_box.append(player_bar.widget());
        playing_box.append(dice_view.widget());
        playing_box.append(turn_panel.widget());
        playing_box.append(scorecard_view.widget());

        // Reconnection overlay (initially hidden, shown on disconnect)
        let reconnection_revealer = gtk4::Revealer::builder()
            .child(reconnection_overlay.widget())
            .reveal_child(false)
            .transition_type(gtk4::RevealerTransitionType::SlideDown)
            .build();
        playing_box.append(&reconnection_revealer);

        // Game over screen
        let game_end_screen = Rc::new(GameEndScreen::new());

        // Content stack
        let content_stack = gtk4::Stack::builder().build();
        content_stack.add_named(&setup_box, Some("setup"));
        content_stack.add_named(&playing_box, Some("playing"));
        content_stack.add_named(game_end_screen.widget(), Some("game-over"));
        content_stack.set_visible_child_name("setup");

        // Header bar
        let header_bar = gtk4::HeaderBar::builder().build();
        let title_label = gtk4::Label::builder().label(&i18n::get("app-title")).css_classes(vec!["header-title"]).build();
        header_bar.set_title_widget(Some(&title_label));

        // Burger menu with Highscore and Info actions
        let menu_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(2)
            .margin_start(6)
            .margin_end(6)
            .margin_top(6)
            .margin_bottom(6)
            .build();
        let highscore_action = gtk4::Button::builder().label(&i18n::get("menu-highscore")).css_classes(vec!["flat"]).build();
        let info_action = gtk4::Button::builder().label(&i18n::get("menu-info")).css_classes(vec!["flat"]).build();
        menu_box.append(&highscore_action);
        menu_box.append(&info_action);

        let popover = gtk4::Popover::builder().child(&menu_box).build();
        let menu_button = gtk4::MenuButton::builder().icon_name("open-menu-symbolic").popover(&popover).build();
        header_bar.pack_end(&menu_button);

        let window = gtk4::ApplicationWindow::builder()
            .application(app)
            .title("Kniffel — Yatzy")
            .default_width(900)
            .default_height(700)
            .child(&content_stack)
            .build();
        window.set_titlebar(Some(&header_bar));

        let reconnection_manager = Rc::new(ReconnectionManager::new());
        let settings_store = Rc::new(SettingsStore::default_path().unwrap_or_else(|_| SettingsStore::new("yatzy_settings.json")));
        let highscore_store = Rc::new(HighscoreStore::default_path().unwrap_or_else(|_| HighscoreStore::new("yatzy_highscore.json")));
        let game_controller: Rc<RefCell<Option<GameController>>> = Rc::new(RefCell::new(None));

        let win = Self {
            window,
            manager,
            mapping,
            dice_service,
            roll_detector,
            event_bridge,
            led_service,
            reconnection_manager,
            game_controller,
            settings_store,
            highscore_store,
            game_state: Rc::new(RefCell::new(None)),
            phase: Rc::new(RefCell::new(GamePhase::Setup)),
            player_bar,
            dice_view,
            scorecard_view,
            turn_panel,
            game_end_screen,
            reconnection_overlay,
            reconnection_revealer,
            player_setup,
            holds: Rc::new(RefCell::new(HoldMask::none())),
            picked_up_dice: Rc::new(RefCell::new(Vec::new())),
            pickup_timer: Rc::new(RefCell::new(None)),
            reconnection_timer: Rc::new(RefCell::new(None)),
            auto_swap_timers: Rc::new(RefCell::new([None, None, None, None, None])),
            slot_verified: Rc::new(RefCell::new([false; 5])),
            status_label,
            scan_progress_bar,
            scan_status_label,
            dice_status_widget,
            scan_button,
            content_stack,
            start_game_receiver: Rc::new(RefCell::new(None)),
        };

        // Connect menu actions
        {
            let window_clone = win.window.clone();
            let highscore_store = win.highscore_store.clone();
            highscore_action.connect_clicked(move |_| {
                let store = highscore_store.clone();
                let window = window_clone.clone();
                glib::spawn_future_local(async move {
                    let list = store
                        .load()
                        .await
                        .unwrap_or_else(|e| {
                            warn!(error = %e, "failed to load highscores");
                            None
                        })
                        .unwrap_or_default();
                    let dialog = HighscoreDialog::new(&window, &list);
                    dialog.present();
                });
            });

            let window_clone = win.window.clone();
            info_action.connect_clicked(move |_| {
                let dialog = InfoDialog::new(&window_clone);
                dialog.present();
            });
        }

        win.connect_signals();
        win.start_event_polling();
        win.load_settings();
        // Auto-start scanning for GoDice devices
        win.dice_service.scan_and_connect();
        win
    }

    /// Connect all signal handlers.
    fn connect_signals(&self) {
        // Channel for communicating player setup → start_game
        let (start_game_sender, start_game_receiver) = std::sync::mpsc::channel::<Vec<Player>>();
        // Store receiver for start_event_polling to pick up
        *self.start_game_receiver.borrow_mut() = Some(start_game_receiver);

        // Scan button
        {
            let dice_service = self.dice_service.clone();
            let status_label = self.status_label.clone();
            self.scan_button.connect_clicked(move |_| {
                status_label.set_label(UiMessage::scanning().as_str());
                dice_service.scan_and_connect();
            });
        }

        // Dice swap: clicking a connected dice slot swaps it for another
        {
            let dice_service = self.dice_service.clone();
            let dice_status_widget = self.dice_status_widget.clone();
            let scan_status_label = self.scan_status_label.clone();
            self.dice_status_widget.set_swap_callback(Box::new(move |slot| {
                dice_status_widget.set_swapping(slot);
                scan_status_label.set_label(&i18n::get_str("scan-swapping", "slot", &slot.get().to_string()));
                let dice_service = dice_service.clone();
                glib::spawn_future_local(async move {
                    let _ = dice_service.swap_dice(slot).await;
                });
            }));
        }

        // Player setup: add player button
        {
            let player_setup = self.player_setup.clone();
            let status_label = self.status_label.clone();
            let add_button = player_setup.borrow().add_button().clone();
            add_button.connect_clicked(move |_| {
                if player_setup.borrow_mut().add_player().is_some() {
                    status_label.set_label(&i18n::get("player-added"));
                } else {
                    status_label.set_label(&i18n::get("player-max-reached"));
                }
            });
        }

        // Player setup: start game button
        {
            let player_setup = self.player_setup.clone();
            let status_label = self.status_label.clone();
            let start_button = player_setup.borrow().start_button().clone();
            let player_setup_clone = player_setup.clone();
            let status_label_clone = status_label.clone();
            let settings_store = self.settings_store.clone();
            start_button.connect_clicked(move |_| {
                match player_setup_clone.borrow().build_setup() {
                    Ok(setup) => {
                        let players = setup.into_players();
                        status_label_clone.set_label(&i18n::get_int("game-starting", "count", players.len() as i64));
                        tracing::info!("Starting game with {} players", players.len());
                        // Persist current player settings
                        if let Ok(settings) = player_setup_clone.borrow().collect_settings() {
                            let store = settings_store.clone();
                            glib::spawn_future_local(async move {
                                if let Err(error) = store.save(&settings).await {
                                    warn!(error = %error, "failed to save settings");
                                }
                            });
                        }
                        // Players will be picked up by the start_game_poller
                        let _ = start_game_sender.send(players);
                    }
                    Err(error) => {
                        status_label_clone.set_label(&i18n::get_str("error-prefix", "error", &error.to_string()));
                    }
                }
            });
        }

        // Roll button → GameController::execute(TurnAction::Roll)
        {
            let roll_detector = self.roll_detector.clone();
            let turn_panel = self.turn_panel.clone();
            let dice_view = self.dice_view.clone();
            let game_controller = self.game_controller.clone();
            let status_label = self.status_label.clone();
            let holds = self.holds.clone();
            self.turn_panel.connect_roll(move || {
                let current_holds = *holds.borrow();
                // Get current faces from controller state (UI labels may be stale)
                let current_faces = game_controller
                    .borrow()
                    .as_ref()
                    .and_then(|c| c.state().ok())
                    .map(|s| {
                        let faces = s.dice_set().faces();
                        [Some(faces[0]), Some(faces[1]), Some(faces[2]), Some(faces[3]), Some(faces[4])]
                    })
                    .unwrap_or([None; 5]);
                // Only show rolling animation for non-held dice, restore held labels
                dice_view.set_rolling_with_holds(current_holds, current_faces);
                turn_panel.set_rolling();
                roll_detector.reset_with_holds(current_holds, current_faces);
                #[allow(clippy::collapsible_if)]
                if let Some(ref controller) = *game_controller.borrow() {
                    if let Err(error) = controller.execute(TurnAction::Roll) {
                        status_label.set_label(&i18n::get_str("error-prefix", "error", &error.to_string()));
                    }
                }
            });
        }

        // Dice view hold toggle → update hold mask + GameController::execute(TurnAction::Hold)
        {
            let status_label = self.status_label.clone();
            let holds = self.holds.clone();
            let game_controller = self.game_controller.clone();
            let dice_view = self.dice_view.clone();
            self.dice_view.connect_hold_toggled(move |slot, held| {
                let message = UiMessage::hold_toggled(slot.get(), held);
                status_label.set_label(message.as_str());
                let mut mask = holds.borrow_mut();
                mask.set(slot, held);
                let mask_copy = *mask;
                drop(mask);
                dice_view.update_holds(mask_copy);
                if let Some(ref controller) = *game_controller.borrow() {
                    let _ = controller.execute(TurnAction::Hold { holds: mask_copy });
                }
            });
        }

        // Scorecard category selection → GameController::execute(EnterScore or CrossOut)
        {
            let status_label = self.status_label.clone();
            let game_controller = self.game_controller.clone();
            let scorecard_view = self.scorecard_view.clone();
            let player_bar = self.player_bar.clone();
            self.scorecard_view.connect_category_selected(move |category| {
                let controller = game_controller.borrow();
                if let Some(controller) = controller.as_ref() {
                    // Try to enter score first; if invalid, try cross-out
                    match controller.potential_score(category) {
                        Ok(score) => {
                            let score_val = score;
                            debug!(category = ?category, score = score_val.get(), "scorecard category selected");
                            match controller.execute(TurnAction::EnterScore { category, score: score_val }) {
                                Ok(events) => {
                                    debug!(category = ?category, "score entered successfully");
                                    status_label.set_label(&format!("{} eingetragen.", category));
                                    // Update scorecard from controller state
                                    if let Ok(state) = controller.state() {
                                        let active = state.current_player_index().get();
                                        scorecard_view.clear_last_entered();
                                        scorecard_view.update_players(state.players(), active);
                                        scorecard_view.set_last_entered(active, category);
                                        player_bar.update_all_scores(state.players());
                                    }
                                    // Process events for UI updates
                                    for event in &events {
                                        match event {
                                            ControllerEvent::TurnStarted { player_index } => {
                                                player_bar.set_active_player(PlayerIndex::new(*player_index));
                                            }
                                            ControllerEvent::PhaseChanged { phase: _ } => {}
                                            ControllerEvent::CrossOutRecommended { recommendation: _ } => {}
                                            _ => {}
                                        }
                                    }
                                }
                                Err(_error) => {
                                    debug!(category = ?category, error = ?_error, "score entry failed, trying cross-out");
                                    // Try cross-out if score entry failed
                                    if let Err(err) = controller.execute(TurnAction::CrossOut { category }) {
                                        debug!(category = ?category, error = ?err, "cross-out failed");
                                        status_label.set_label(&i18n::get_str("error-prefix", "error", &err.to_string()));
                                    } else {
                                        debug!(category = ?category, "crossed out successfully");
                                        status_label.set_label(&i18n::get_str("category-crossed-out", "category", &category.to_string()));
                                        if let Ok(state) = controller.state() {
                                            let active = state.current_player_index().get();
                                            scorecard_view.clear_last_entered();
                                            scorecard_view.update_players(state.players(), active);
                                            scorecard_view.set_last_entered(active, category);
                                            player_bar.update_all_scores(state.players());
                                        }
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            debug!(category = ?category, error = ?error, "potential_score failed");
                            status_label.set_label(&i18n::get_str("error-prefix", "error", &error.to_string()));
                        }
                    }
                }
            });
        }

        // Game end screen: new game button
        {
            let content_stack = self.content_stack.clone();
            let phase = self.phase.clone();
            let dice_view = self.dice_view.clone();
            let scorecard_view = self.scorecard_view.clone();
            let turn_panel = self.turn_panel.clone();
            let slot_verified = self.slot_verified.clone();
            let auto_swap_timers = self.auto_swap_timers.clone();
            let player_setup = self.player_setup.clone();
            self.game_end_screen.new_game_button().connect_clicked(move |_| {
                *phase.borrow_mut() = GamePhase::Setup;
                content_stack.set_visible_child_name("setup");
                dice_view.reset();
                scorecard_view.clear();
                turn_panel.reset();
                // Cancel any pending auto-swap timers
                for timer in auto_swap_timers.borrow_mut().iter_mut() {
                    if let Some(id) = timer.take() {
                        id.remove();
                    }
                }
                // Dice are still connected from the previous game — keep
                // verification state and enable start button if all verified.
                if slot_verified.borrow().iter().all(|&v| v) {
                    player_setup.borrow().set_start_button_sensitive(true);
                } else {
                    player_setup.borrow().set_start_button_sensitive(false);
                }
            });
        }

        // Reconnection overlay: retry button
        {
            let status_label = self.status_label.clone();
            let dice_service = self.dice_service.clone();
            let reconnection_manager = self.reconnection_manager.clone();
            let reconnection_overlay = self.reconnection_overlay.clone();
            let reconnection_revealer = self.reconnection_revealer.clone();
            let event_bridge = self.event_bridge.clone();
            let mapping = self.mapping.clone();
            self.reconnection_overlay.retry_button().connect_clicked(move |_| {
                status_label.set_label(&i18n::get("reconnecting"));
                let dice_service = dice_service.clone();
                let reconnection_manager = reconnection_manager.clone();
                let reconnection_overlay = reconnection_overlay.clone();
                let reconnection_revealer = reconnection_revealer.clone();
                let event_bridge = event_bridge.clone();
                let mapping = mapping.clone();
                let status_label = status_label.clone();
                glib::spawn_future_local(async move {
                    let pending: Vec<(DiceSlot, String)> = reconnection_manager
                        .pending_requests()
                        .iter()
                        .map(|r| (r.slot(), r.device_name().to_string()))
                        .collect();
                    let reconnected = dice_service.reconnect_pending(&pending).await;
                    for slot in &reconnected {
                        reconnection_manager.mark_reconnected(*slot);
                        if let Some(dice) = mapping.get(*slot) {
                            event_bridge.start_listening(*slot, dice);
                        }
                    }
                    if !reconnected.is_empty() {
                        status_label.set_label(&i18n::get_int("reconnection-reconnected", "count", reconnected.len() as i64));
                    }
                    if !reconnection_manager.has_pending() {
                        reconnection_overlay.clear();
                        reconnection_revealer.set_reveal_child(false);
                    }
                });
            });
        }

        // Reconnection overlay: dismiss button
        {
            let reconnection_overlay = self.reconnection_overlay.clone();
            self.reconnection_overlay.dismiss_button().connect_clicked(move |_| {
                reconnection_overlay.clear();
            });
        }
    }

    /// Start polling for events from the dice service and event bridge.
    fn start_event_polling(&self) {
        // Poll start_game channel
        let mut start_receiver = self.start_game_receiver.borrow_mut().take();
        let game_controller = self.game_controller.clone();
        let game_state = self.game_state.clone();
        let player_bar = self.player_bar.clone();
        let dice_view = self.dice_view.clone();
        let scorecard_view = self.scorecard_view.clone();
        let turn_panel = self.turn_panel.clone();
        let reconnection_manager = self.reconnection_manager.clone();
        let reconnection_overlay = self.reconnection_overlay.clone();
        let reconnection_revealer = self.reconnection_revealer.clone();
        let holds = self.holds.clone();
        let led_service = self.led_service.clone();
        let game_end_screen = self.game_end_screen.clone();
        let highscore_store = self.highscore_store.clone();
        let reconnection_timer = self.reconnection_timer.clone();
        let auto_swap_timers = self.auto_swap_timers.clone();
        let slot_verified = self.slot_verified.clone();

        // Poll dice service events
        let mut dice_receiver = self.dice_service.subscribe();
        let status_label = self.status_label.clone();
        let event_bridge = self.event_bridge.clone();
        let mapping = self.mapping.clone();
        let content_stack = self.content_stack.clone();
        let phase = self.phase.clone();
        let dice_status_widget = self.dice_status_widget.clone();
        let scan_progress_bar = self.scan_progress_bar.clone();
        let scan_status_label = self.scan_status_label.clone();
        let dice_service = self.dice_service.clone();

        glib::timeout_add_local(Duration::from_millis(50), move || {
            // Check for pending start_game requests
            if let Some(ref mut receiver) = start_receiver {
                while let Ok(players) = receiver.try_recv() {
                    if let Ok(controller) = GameController::new(players.clone()) {
                        // Subscribe to controller events before storing
                        let controller_receiver = controller.subscribe();
                        *game_controller.borrow_mut() = Some(controller);
                        *game_state.borrow_mut() = GameState::new(players.clone()).ok();
                        player_bar.set_players(&players);
                        player_bar.set_active_player(PlayerIndex::new(0));
                        turn_panel.reset();
                        dice_view.reset();
                        scorecard_view.clear();
                        // Show initial scorecard for all players
                        if let Ok(state) = game_controller.borrow().as_ref().unwrap().state() {
                            scorecard_view.update_players(state.players(), state.current_player_index().get());
                        }
                        reconnection_manager.clear();
                        reconnection_overlay.clear();
                        reconnection_revealer.set_reveal_child(false);
                        if let Some(id) = reconnection_timer.borrow_mut().take() {
                            id.remove();
                        }
                        *holds.borrow_mut() = HoldMask::none();
                        *phase.borrow_mut() = GamePhase::Playing;
                        content_stack.set_visible_child_name("playing");
                        status_label.set_label(&i18n::get("game-started-scanning"));

                        // Auto-scan and connect GoDice devices
                        dice_service.scan_and_connect();

                        // Start controller event polling timer
                        let led_service = led_service.clone();
                        let game_end_screen = game_end_screen.clone();
                        let content_stack = content_stack.clone();
                        let phase = phase.clone();
                        let status_label = status_label.clone();
                        let player_bar = player_bar.clone();
                        let scorecard_view = scorecard_view.clone();
                        let turn_panel = turn_panel.clone();
                        let dice_view = dice_view.clone();
                        let game_controller = game_controller.clone();
                        let holds = holds.clone();
                        let highscore_store = highscore_store.clone();
                        let mut controller_receiver = controller_receiver;
                        #[allow(clippy::collapsible_if)]
                        glib::timeout_add_local(Duration::from_millis(50), move || {
                            while let Ok(event) = controller_receiver.try_recv() {
                                match event {
                                    ControllerEvent::Celebration { effect } => {
                                        let led = led_service.clone();
                                        glib::spawn_future_local(async move {
                                            let _ = led.apply(&effect).await;
                                        });
                                    }
                                    ControllerEvent::GameOver { standings } => {
                                        game_end_screen.update(&standings);
                                        *phase.borrow_mut() = GamePhase::GameOver;
                                        content_stack.set_visible_child_name("game-over");
                                        status_label.set_label(&i18n::get("status-game-over"));
                                        turn_panel.set_game_over();
                                        // Save highscores
                                        let store = highscore_store.clone();
                                        glib::spawn_future_local(async move {
                                            match store.load().await {
                                                Ok(Some(mut list)) => {
                                                    for entry in &standings {
                                                        list.add(HighscoreEntry::new(entry.player().name().as_str(), entry.player().grand_total()));
                                                    }
                                                    if let Err(error) = store.save(&list).await {
                                                        warn!(error = %error, "failed to save highscores");
                                                    }
                                                }
                                                Ok(None) => {
                                                    let mut list = HighscoreList::new();
                                                    for entry in &standings {
                                                        list.add(HighscoreEntry::new(entry.player().name().as_str(), entry.player().grand_total()));
                                                    }
                                                    if let Err(error) = store.save(&list).await {
                                                        warn!(error = %error, "failed to save highscores");
                                                    }
                                                }
                                                Err(error) => {
                                                    warn!(error = %error, "failed to load highscores for saving");
                                                }
                                            }
                                        });
                                    }
                                    ControllerEvent::RollCompleted { dice: _ } => {
                                        // Dice view is already updated by GameEvent::RollComplete.
                                        // Only update the turn panel here.
                                        if let Some(ref controller) = *game_controller.borrow() {
                                            if let Ok(state) = controller.state() {
                                                turn_panel.set_roll_complete(state.rolls_used());
                                            }
                                        }
                                    }
                                    ControllerEvent::ScoreEntered {
                                        player_index,
                                        category,
                                        score: _,
                                    } => {
                                        status_label.set_label(&i18n::get_str_int(
                                            "score-entered-for-player",
                                            "category",
                                            &category.to_string(),
                                            "player",
                                            (player_index + 1) as i64,
                                        ));
                                        scorecard_view.clear_last_entered();
                                        if let Some(ref controller) = *game_controller.borrow() {
                                            if let Ok(state) = controller.state() {
                                                scorecard_view.update_players(state.players(), state.current_player_index().get());
                                                scorecard_view.set_last_entered(player_index, category);
                                                player_bar.update_all_scores(state.players());
                                            }
                                        }
                                    }
                                    ControllerEvent::CategoryCrossedOut { player_index, category } => {
                                        status_label.set_label(&i18n::get_str_int(
                                            "category-crossed-out-for-player",
                                            "category",
                                            &category.to_string(),
                                            "player",
                                            (player_index + 1) as i64,
                                        ));
                                        scorecard_view.clear_last_entered();
                                        if let Some(ref controller) = *game_controller.borrow() {
                                            if let Ok(state) = controller.state() {
                                                scorecard_view.update_players(state.players(), state.current_player_index().get());
                                                scorecard_view.set_last_entered(player_index, category);
                                                player_bar.update_all_scores(state.players());
                                            }
                                        }
                                    }
                                    ControllerEvent::TurnStarted { player_index } => {
                                        player_bar.set_active_player(PlayerIndex::new(player_index));
                                        *holds.borrow_mut() = HoldMask::none();
                                        dice_view.update_holds(HoldMask::none());
                                        // Show scorecard with new active player highlighted
                                        if let Some(ref controller) = *game_controller.borrow()
                                            && let Ok(state) = controller.state()
                                        {
                                            scorecard_view.update_players(state.players(), state.current_player_index().get());
                                        }
                                        status_label.set_label(&i18n::get_int("player-turn", "player", (player_index + 1) as i64));
                                    }
                                    ControllerEvent::PhaseChanged { phase: new_phase } => match new_phase {
                                        crate::models::turn_phase::TurnPhase::Rolling => {
                                            turn_panel.set_rolling();
                                        }
                                        crate::models::turn_phase::TurnPhase::Holding => {
                                            if let Some(ref controller) = *game_controller.borrow() {
                                                if let Ok(state) = controller.state() {
                                                    turn_panel.set_roll_complete(state.rolls_used());
                                                }
                                            }
                                        }
                                        crate::models::turn_phase::TurnPhase::Scoring => {
                                            turn_panel.set_roll_button_label(RollButtonLabel::NoRollsLeft);
                                        }
                                        crate::models::turn_phase::TurnPhase::AwaitingRoll => {
                                            turn_panel.reset();
                                        }
                                        crate::models::turn_phase::TurnPhase::TurnEnd => {}
                                    },
                                    ControllerEvent::StatusChanged { status: _ } => {}
                                    ControllerEvent::CrossOutRecommended { recommendation: _ } => {}
                                    ControllerEvent::TurnTransition { transition: _ } => {}
                                }
                            }
                            glib::ControlFlow::Continue
                        });
                    }
                }
            }

            while let Ok(event) = dice_receiver.try_recv() {
                match event {
                    DiceServiceEvent::ScanStarted => {
                        scan_status_label.set_label(&i18n::get("scan-status-scanning"));
                        scan_progress_bar.set_fraction(0.0);
                    }
                    DiceServiceEvent::NoDevicesFound => {
                        scan_status_label.set_label(&i18n::get("scan-no-devices"));
                    }
                    DiceServiceEvent::DevicesFound(count) => {
                        scan_status_label.set_label(&i18n::get_int("scan-devices-found", "count", count as i64));
                    }
                    DiceServiceEvent::DeviceFound { name: _, color } => {
                        scan_status_label.set_label(&i18n::get_str("scan-device-found", "color", &dice_color_name(color)));
                        // Show in next available slot as discovered (pulsing)
                        if let Some(slot) = dice_status_widget.next_empty_slot() {
                            dice_status_widget.set_discovered(slot, color);
                        }
                    }
                    DiceServiceEvent::Connecting { name: _, color } => {
                        scan_status_label.set_label(&i18n::get_str("scan-connecting", "color", &dice_color_name(color)));
                    }
                    DiceServiceEvent::DiceAssigned { slot, name, color } => {
                        scan_status_label.set_label(&i18n::get_str("scan-dice-connected", "color", &dice_color_name(color)));
                        let connected_count = dice_service.assigned_count();
                        scan_progress_bar.set_fraction(connected_count as f64 / DiceSlot::COUNT as f64);
                        // Update setup screen dice status widget
                        if let Ok(slot) = DiceSlot::new(slot) {
                            dice_status_widget.set_connected(slot, &name, color);
                            // Request battery level
                            if let Some(dice) = mapping.get(slot) {
                                let dice_status_widget = dice_status_widget.clone();
                                glib::spawn_future_local(async move {
                                    if let Ok(battery) = dice.get_battery_level().await {
                                        dice_status_widget.set_battery(slot, battery.get());
                                    }
                                });
                            }
                        }
                        // Set UI color indicator from physical dice color
                        if let Ok(slot) = DiceSlot::new(slot) {
                            dice_view.set_slot_color(slot, color);
                            // Set LED to match physical dice color
                            let led_color = dice_color_to_led(color);
                            let led = led_service.clone();
                            glib::spawn_future_local(async move {
                                let _ = led
                                    .apply(&LedEffect::Solid {
                                        slot,
                                        color: PlayerColor::new(led_color),
                                    })
                                    .await;
                            });
                            // Start listening for this dice's events.
                            // Reset the slot first in case this is a swap
                            // (the old listener task may still be alive).
                            if let Some(dice) = mapping.get(slot) {
                                event_bridge.reset_active_slot(slot);
                                event_bridge.start_listening(slot, dice);
                            }
                            // Start auto-swap timer: if no Stable event arrives
                            // within 10s, the dice is unresponsive and gets swapped.
                            {
                                slot_verified.borrow_mut()[slot.get() as usize] = false;
                                // Cancel any existing timer for this slot
                                if let Some(id) = auto_swap_timers.borrow_mut()[slot.get() as usize].take() {
                                    id.remove();
                                }
                                let dice_service = dice_service.clone();
                                let dice_status_widget = dice_status_widget.clone();
                                let scan_status_label = scan_status_label.clone();
                                let auto_swap_timers_inner = auto_swap_timers.clone();
                                let slot_verified = slot_verified.clone();
                                let slot_idx = slot.get() as usize;
                                let slot_for_cb = slot;
                                let id = glib::timeout_add_local(Duration::from_secs(10), move || {
                                    if slot_verified.borrow()[slot_idx] {
                                        return glib::ControlFlow::Break;
                                    }
                                    debug!(slot = slot_idx, "auto-swap: no Stable event received, swapping");
                                    // Clear timer ID so DiceAssigned won't try to remove it
                                    auto_swap_timers_inner.borrow_mut()[slot_idx] = None;
                                    scan_status_label.set_label(&i18n::get_str("scan-auto-swap", "slot", &slot_idx.to_string()));
                                    dice_status_widget.set_swapping(slot_for_cb);
                                    let dice_service = dice_service.clone();
                                    glib::spawn_future_local(async move {
                                        let _ = dice_service.swap_dice(slot_for_cb).await;
                                    });
                                    glib::ControlFlow::Break
                                });
                                auto_swap_timers.borrow_mut()[slot.get() as usize] = Some(id);
                            }
                        }
                    }
                    DiceServiceEvent::DiceConnectionFailed { name: _, error: _ } => {
                        // Keep scanning, don't change status text much
                    }
                    DiceServiceEvent::ScanFailed(_error) => {
                        scan_status_label.set_label(&i18n::get("scan-failed"));
                    }
                    DiceServiceEvent::AllSlotsAssigned => {
                        scan_status_label.set_label(&i18n::get("scan-all-connected"));
                        scan_progress_bar.set_fraction(1.0);
                        // Don't enable start button yet — wait until all dice
                        // report a Stable event (verified via DiceStable handler).
                        for (slot, dice) in mapping.assigned() {
                            event_bridge.start_listening(slot, dice);
                        }
                        event_bridge.start_roll_detector_forwarding();
                    }
                }
            }
            glib::ControlFlow::Continue
        });

        // Poll game events
        let mut game_receiver = self.event_bridge.subscribe();
        let dice_view = self.dice_view.clone();
        let turn_panel = self.turn_panel.clone();
        let status_label = self.status_label.clone();
        let led_service = self.led_service.clone();
        let reconnection_manager = self.reconnection_manager.clone();
        let reconnection_overlay = self.reconnection_overlay.clone();
        let reconnection_revealer = self.reconnection_revealer.clone();
        let game_controller = self.game_controller.clone();
        let holds = self.holds.clone();
        let picked_up_dice = self.picked_up_dice.clone();
        let pickup_timer = self.pickup_timer.clone();
        let roll_detector = self.roll_detector.clone();
        let dice_service = self.dice_service.clone();
        let event_bridge = self.event_bridge.clone();
        let mapping = self.mapping.clone();
        let reconnection_timer = self.reconnection_timer.clone();
        let dice_status_widget = self.dice_status_widget.clone();
        let phase = self.phase.clone();
        let auto_swap_timers = self.auto_swap_timers.clone();
        let slot_verified = self.slot_verified.clone();
        let player_setup = self.player_setup.clone();

        glib::timeout_add_local(Duration::from_millis(50), move || {
            while let Ok(event) = game_receiver.try_recv() {
                match event {
                    GameEvent::RollStarted => {
                        // Check current phase
                        let current_phase = game_controller.borrow().as_ref().and_then(|c| c.state().ok()).map(|s| s.phase());
                        let is_holding_phase = current_phase.map(|p| p == crate::models::turn_phase::TurnPhase::Holding).unwrap_or(false);
                        let is_awaiting_roll = current_phase.map(|p| p == crate::models::turn_phase::TurnPhase::AwaitingRoll).unwrap_or(false);

                        // First roll of a turn: physical pickup triggers RollStarted
                        // but nobody called execute(Roll) yet. Transition now so
                        // dice_stable will succeed when RollComplete fires.
                        if is_awaiting_roll && let Some(ref controller) = *game_controller.borrow() {
                            debug!("RollStarted in AwaitingRoll, calling execute(Roll)");
                            let _ = controller.execute(TurnAction::Roll);
                        }

                        // During Holding phase, a physical pickup starts a roll
                        // before the auto-hold timer has fired. Mark the roll as
                        // pending holds so it won't complete prematurely.
                        if is_holding_phase {
                            debug!("set_pending_holds(true) — RollStarted during Holding");
                            roll_detector.set_pending_holds(true);
                        }

                        let current_holds = *holds.borrow();
                        if is_holding_phase && current_holds.held_count() == 0 {
                            // Auto-hold timer hasn't fired yet; don't show rolling
                        } else {
                            // Either not in Holding phase (first roll) or holds
                            // already set (re-roll via roll button)
                            let current_faces = game_controller
                                .borrow()
                                .as_ref()
                                .and_then(|c| c.state().ok())
                                .map(|s| {
                                    let faces = s.dice_set().faces();
                                    [Some(faces[0]), Some(faces[1]), Some(faces[2]), Some(faces[3]), Some(faces[4])]
                                })
                                .unwrap_or([None; 5]);
                            dice_view.set_rolling_with_holds(current_holds, current_faces);
                            turn_panel.set_rolling();
                            status_label.set_label(UiMessage::roll_started().as_str());
                        }
                    }
                    GameEvent::RollComplete { faces } => {
                        debug!(
                            faces = ?faces.iter().map(|f| f.map(|v| v.get())).collect::<Vec<_>>(),
                            "RollComplete received"
                        );
                        dice_view.clear_rolling();
                        // Fall back to current controller faces for dice that
                        // didn't report (e.g. hardware/firmware issues).
                        let controller_faces: [Option<dice_rs::FaceValue>; 5] = game_controller
                            .borrow()
                            .as_ref()
                            .and_then(|c| c.state().ok())
                            .map(|s| {
                                let f = s.dice_set().faces();
                                [Some(f[0]), Some(f[1]), Some(f[2]), Some(f[3]), Some(f[4])]
                            })
                            .unwrap_or([None; 5]);
                        let merged = [
                            Some(faces[0].or(controller_faces[0]).unwrap_or(dice_rs::FaceValue::new(1).unwrap())),
                            Some(faces[1].or(controller_faces[1]).unwrap_or(dice_rs::FaceValue::new(1).unwrap())),
                            Some(faces[2].or(controller_faces[2]).unwrap_or(dice_rs::FaceValue::new(1).unwrap())),
                            Some(faces[3].or(controller_faces[3]).unwrap_or(dice_rs::FaceValue::new(1).unwrap())),
                            Some(faces[4].or(controller_faces[4]).unwrap_or(dice_rs::FaceValue::new(1).unwrap())),
                        ];
                        // Warn about dice that didn't report — their values are fallbacks
                        for i in 0..5 {
                            if faces[i].is_none() {
                                let source = if controller_faces[i].is_some() { "controller" } else { "default" };
                                debug!(slot = i, fallback = merged[i].map(|v| v.get()), source, "die did not report, using fallback");
                            }
                        }
                        dice_view.update_faces(&merged);
                        status_label.set_label(UiMessage::roll_complete().as_str());
                        // Forward to game controller
                        #[allow(clippy::collapsible_if)]
                        if let Some(ref controller) = *game_controller.borrow() {
                            if let Some(dice) = faces_to_dice_set(&merged) {
                                let _ = controller.dice_stable(dice);
                            }
                            // Update turn panel immediately — don't wait for controller event
                            if let Ok(state) = controller.state() {
                                turn_panel.set_roll_complete(state.rolls_used());
                            }
                        }
                    }
                    GameEvent::RollTimedOut { faces } => {
                        dice_view.clear_rolling();
                        // Fall back to current controller faces for dice that
                        // didn't report.
                        let controller_faces: [Option<dice_rs::FaceValue>; 5] = game_controller
                            .borrow()
                            .as_ref()
                            .and_then(|c| c.state().ok())
                            .map(|s| {
                                let f = s.dice_set().faces();
                                [Some(f[0]), Some(f[1]), Some(f[2]), Some(f[3]), Some(f[4])]
                            })
                            .unwrap_or([None; 5]);
                        let merged = [
                            Some(faces[0].or(controller_faces[0]).unwrap_or(dice_rs::FaceValue::new(1).unwrap())),
                            Some(faces[1].or(controller_faces[1]).unwrap_or(dice_rs::FaceValue::new(1).unwrap())),
                            Some(faces[2].or(controller_faces[2]).unwrap_or(dice_rs::FaceValue::new(1).unwrap())),
                            Some(faces[3].or(controller_faces[3]).unwrap_or(dice_rs::FaceValue::new(1).unwrap())),
                            Some(faces[4].or(controller_faces[4]).unwrap_or(dice_rs::FaceValue::new(1).unwrap())),
                        ];
                        dice_view.update_faces(&merged);
                        status_label.set_label(UiMessage::roll_timed_out().as_str());
                        // Forward to game controller with partial dice
                        #[allow(clippy::collapsible_if)]
                        if let Some(ref controller) = *game_controller.borrow() {
                            if let Some(dice) = faces_to_dice_set(&merged) {
                                let _ = controller.dice_stable(dice);
                            }
                            // Update turn panel immediately
                            if let Ok(state) = controller.state() {
                                turn_panel.set_roll_complete(state.rolls_used());
                            }
                        }
                    }
                    GameEvent::DiceStable { slot, face } => {
                        // Update face value on the setup screen
                        if *phase.borrow() == GamePhase::Setup {
                            dice_status_widget.set_face_value(slot, face);
                        }
                        // Mark slot as verified (received a Stable event)
                        // and cancel any pending auto-swap timer.
                        slot_verified.borrow_mut()[slot.get() as usize] = true;
                        if let Some(id) = auto_swap_timers.borrow_mut()[slot.get() as usize].take() {
                            id.remove();
                        }
                        // Enable start button when all slots are verified
                        if *phase.borrow() == GamePhase::Setup && slot_verified.borrow().iter().all(|&v| v) {
                            player_setup.borrow().set_start_button_sensitive(true);
                        }
                    }
                    GameEvent::DicePickedUp { slot } => {
                        // Only auto-hold during Holding phase (after a roll, before next roll)
                        let is_holding_phase = game_controller
                            .borrow()
                            .as_ref()
                            .and_then(|c| c.state().ok())
                            .map(|s| s.phase() == crate::models::turn_phase::TurnPhase::Holding)
                            .unwrap_or(false);
                        debug!(slot = slot.get(), is_holding_phase, "DicePickedUp");
                        if !is_holding_phase {
                            continue;
                        }
                        // Add to picked-up set if not already there
                        let mut picked = picked_up_dice.borrow_mut();
                        if !picked.contains(&slot) {
                            picked.push(slot);
                        }
                        drop(picked);
                        // Cancel any existing timer
                        if let Some(id) = pickup_timer.borrow_mut().take() {
                            id.remove();
                        }
                        // Start a new 1s collection window
                        let picked_up_dice = picked_up_dice.clone();
                        let pickup_timer_inner = pickup_timer.clone();
                        let holds = holds.clone();
                        let dice_view = dice_view.clone();
                        let turn_panel = turn_panel.clone();
                        let game_controller = game_controller.clone();
                        let roll_detector = roll_detector.clone();
                        let status_label = status_label.clone();
                        let id = glib::timeout_add_local(Duration::from_secs(1), move || {
                            debug!("auto-hold timer fired");
                            // Auto-hold all dice NOT picked up
                            let picked = picked_up_dice.borrow().clone();
                            debug!(picked_count = picked.len(), "picked up dice");
                            let mut mask = holds.borrow_mut();
                            for slot in DiceSlot::all() {
                                if !picked.contains(&slot) {
                                    mask.set(slot, true);
                                } else {
                                    mask.set(slot, false);
                                }
                            }
                            let mask_copy = *mask;
                            drop(mask);
                            // Update UI
                            dice_view.update_holds(mask_copy);
                            // Update controller holds
                            if let Some(ref controller) = *game_controller.borrow() {
                                let _ = controller.execute(TurnAction::Hold { holds: mask_copy });
                            }
                            // Get current faces from the controller state (not from UI labels,
                            // which may already show "..." from the RollStarted event)
                            let current_faces = game_controller
                                .borrow()
                                .as_ref()
                                .and_then(|c| c.state().ok())
                                .map(|s| {
                                    let faces = s.dice_set().faces();
                                    [Some(faces[0]), Some(faces[1]), Some(faces[2]), Some(faces[3]), Some(faces[4])]
                                })
                                .unwrap_or([None; 5]);
                            // Mark held dice as stable in the roll detector
                            debug!(
                                faces = ?current_faces.iter().map(|f| f.map(|v| v.get())).collect::<Vec<_>>(),
                                held_count = mask_copy.held_count(),
                                "mark_held_stable"
                            );
                            roll_detector.mark_held_stable(mask_copy, current_faces);
                            // Show rolling animation only for non-held dice,
                            // restore face labels for held dice
                            dice_view.set_rolling_with_holds(mask_copy, current_faces);
                            turn_panel.set_rolling();
                            if let Some(ref controller) = *game_controller.borrow() {
                                let phase = controller.state().map(|s| s.phase()).ok();
                                debug!(?phase, "executing Roll action");
                                if let Err(error) = controller.execute(TurnAction::Roll) {
                                    debug!(?error, "Roll action failed");
                                    status_label.set_label(&i18n::get_str("error-prefix", "error", &error.to_string()));
                                }
                            }
                            // Clear picked-up state
                            picked_up_dice.borrow_mut().clear();
                            *pickup_timer_inner.borrow_mut() = None;
                            glib::ControlFlow::Break
                        });
                        *pickup_timer.borrow_mut() = Some(id);
                    }
                    GameEvent::DiceDisconnected { slot, name } => {
                        dice_view.set_disconnected(slot);
                        status_label.set_label(UiMessage::dice_disconnected(slot.get()).as_str());
                        reconnection_manager.register_disconnect(ReconnectionRequest::new(slot, name));
                        reconnection_overlay.show_disconnected(slot);
                        reconnection_revealer.set_reveal_child(true);
                        let led = led_service.clone();
                        glib::spawn_future_local(async move {
                            let _ = led.apply(&LedEffect::all_off()).await;
                        });
                        // Start periodic reconnection timer if not already running
                        if reconnection_timer.borrow().is_none() {
                            let dice_service = dice_service.clone();
                            let reconnection_manager = reconnection_manager.clone();
                            let reconnection_overlay = reconnection_overlay.clone();
                            let reconnection_revealer = reconnection_revealer.clone();
                            let event_bridge = event_bridge.clone();
                            let mapping = mapping.clone();
                            let status_label = status_label.clone();
                            let reconnection_timer_inner = reconnection_timer.clone();
                            let id = glib::timeout_add_local(Duration::from_secs(5), move || {
                                let dice_service = dice_service.clone();
                                let reconnection_manager = reconnection_manager.clone();
                                let reconnection_overlay = reconnection_overlay.clone();
                                let reconnection_revealer = reconnection_revealer.clone();
                                let event_bridge = event_bridge.clone();
                                let mapping = mapping.clone();
                                let status_label = status_label.clone();
                                let timer_for_closure = reconnection_timer_inner.clone();
                                let timer_for_check = reconnection_timer_inner.clone();
                                glib::spawn_future_local(async move {
                                    if !reconnection_manager.has_pending() {
                                        *timer_for_closure.borrow_mut() = None;
                                        return;
                                    }
                                    let pending: Vec<(DiceSlot, String)> = reconnection_manager
                                        .pending_requests()
                                        .iter()
                                        .map(|r| (r.slot(), r.device_name().to_string()))
                                        .collect();
                                    let reconnected = dice_service.reconnect_pending(&pending).await;
                                    for slot in &reconnected {
                                        reconnection_manager.mark_reconnected(*slot);
                                        if let Some(dice) = mapping.get(*slot) {
                                            event_bridge.start_listening(*slot, dice);
                                        }
                                    }
                                    if !reconnected.is_empty() {
                                        status_label.set_label(&i18n::get_int("dice-reconnected", "count", reconnected.len() as i64));
                                    }
                                    if !reconnection_manager.has_pending() {
                                        reconnection_overlay.clear();
                                        reconnection_revealer.set_reveal_child(false);
                                        *timer_for_closure.borrow_mut() = None;
                                    }
                                });
                                // Keep timer running unless cleared
                                if timer_for_check.borrow().is_some() {
                                    glib::ControlFlow::Continue
                                } else {
                                    glib::ControlFlow::Break
                                }
                            });
                            *reconnection_timer.borrow_mut() = Some(id);
                        }
                    }
                    GameEvent::DiceReconnected { slot } => {
                        dice_view.clear_disconnected(slot);
                        reconnection_manager.mark_reconnected(slot);
                        if !reconnection_manager.has_pending() {
                            reconnection_overlay.clear();
                            reconnection_revealer.set_reveal_child(false);
                        }
                    }
                    GameEvent::ApplyLedEffect { effect } => {
                        let led = led_service.clone();
                        glib::spawn_future_local(async move {
                            let _ = led.apply(&effect).await;
                        });
                    }
                    GameEvent::ActivePlayerChanged { player_index: _, color } => {
                        let effect = LedEffect::active_player(color);
                        let led = led_service.clone();
                        glib::spawn_future_local(async move {
                            let _ = led.apply(&effect).await;
                        });
                    }
                }
            }
            glib::ControlFlow::Continue
        });
    }

    /// Start a new game with the given players.
    pub fn start_game(&self, players: Vec<Player>) {
        if let Ok(controller) = GameController::new(players.clone()) {
            *self.game_controller.borrow_mut() = Some(controller);
            *self.game_state.borrow_mut() = GameState::new(players.clone()).ok();
            self.player_bar.set_players(&players);
            self.player_bar.set_active_player(PlayerIndex::new(0));
            self.turn_panel.reset();
            self.dice_view.reset();
            self.scorecard_view.clear();
            self.reconnection_manager.clear();
            self.reconnection_overlay.clear();
            self.reconnection_revealer.set_reveal_child(false);
            if let Some(id) = self.reconnection_timer.borrow_mut().take() {
                id.remove();
            }
            *self.holds.borrow_mut() = HoldMask::none();
            *self.phase.borrow_mut() = GamePhase::Playing;
            self.content_stack.set_visible_child_name("playing");
            self.start_controller_event_polling();

            // Persist settings
            let settings = crate::models::game_settings::GameSettings::default_single_player();
            self.save_settings(&settings);
        }
    }

    /// Show the game end screen with the final standings.
    pub fn show_game_over(&self, standings: &[crate::models::standing::StandingEntry]) {
        self.game_end_screen.update(standings);
        *self.phase.borrow_mut() = GamePhase::GameOver;
        self.content_stack.set_visible_child_name("game-over");
    }

    /// Present the window.
    pub fn present(&self) {
        self.window.present();
    }

    /// Load settings from disk on startup.
    fn load_settings(&self) {
        let store = self.settings_store.clone();
        let status_label = self.status_label.clone();
        let player_setup = self.player_setup.clone();
        glib::spawn_future_local(async move {
            match store.load().await {
                Ok(Some(settings)) => {
                    let msg = i18n::get_int("settings-loaded", "count", settings.player_count() as i64);
                    status_label.set_label(&msg);
                    player_setup.borrow_mut().load_settings(&settings);
                }
                Ok(None) => {
                    // First run — no settings file yet
                }
                Err(error) => {
                    warn!(error = %error, "failed to load settings");
                }
            }
        });
    }

    /// Save current settings to disk.
    fn save_settings(&self, settings: &crate::models::game_settings::GameSettings) {
        let store = self.settings_store.clone();
        let settings = settings.clone();
        glib::spawn_future_local(async move {
            if let Err(error) = store.save(&settings).await {
                warn!(error = %error, "failed to save settings");
            }
        });
    }

    /// Start polling for controller events from the game controller.
    fn start_controller_event_polling(&self) {
        let controller = self.game_controller.borrow();
        let Some(controller) = controller.as_ref() else {
            return;
        };
        let mut receiver = controller.subscribe();

        let led_service = self.led_service.clone();
        let game_end_screen = self.game_end_screen.clone();
        let content_stack = self.content_stack.clone();
        let phase = self.phase.clone();
        let status_label = self.status_label.clone();
        let player_bar = self.player_bar.clone();
        let scorecard_view = self.scorecard_view.clone();
        let turn_panel = self.turn_panel.clone();
        let dice_view = self.dice_view.clone();
        let game_controller = self.game_controller.clone();
        let holds = self.holds.clone();
        let highscore_store = self.highscore_store.clone();

        #[allow(clippy::collapsible_if)]
        glib::timeout_add_local(Duration::from_millis(50), move || {
            while let Ok(event) = receiver.try_recv() {
                match event {
                    ControllerEvent::Celebration { effect } => {
                        let led = led_service.clone();
                        glib::spawn_future_local(async move {
                            let _ = led.apply(&effect).await;
                        });
                    }
                    ControllerEvent::GameOver { standings } => {
                        game_end_screen.update(&standings);
                        *phase.borrow_mut() = GamePhase::GameOver;
                        content_stack.set_visible_child_name("game-over");
                        status_label.set_label(&i18n::get("status-game-over"));
                        turn_panel.set_game_over();
                        // Save highscores
                        let store = highscore_store.clone();
                        glib::spawn_future_local(async move {
                            match store.load().await {
                                Ok(Some(mut list)) => {
                                    for entry in &standings {
                                        list.add(HighscoreEntry::new(entry.player().name().as_str(), entry.player().grand_total()));
                                    }
                                    if let Err(error) = store.save(&list).await {
                                        warn!(error = %error, "failed to save highscores");
                                    }
                                }
                                Ok(None) => {
                                    let mut list = HighscoreList::new();
                                    for entry in &standings {
                                        list.add(HighscoreEntry::new(entry.player().name().as_str(), entry.player().grand_total()));
                                    }
                                    if let Err(error) = store.save(&list).await {
                                        warn!(error = %error, "failed to save highscores");
                                    }
                                }
                                Err(error) => {
                                    warn!(error = %error, "failed to load highscores for saving");
                                }
                            }
                        });
                    }
                    ControllerEvent::RollCompleted { dice: _ } => {
                        // Dice view is already updated by GameEvent::RollComplete.
                        // Only update the turn panel here.
                        if let Some(ref controller) = *game_controller.borrow() {
                            if let Ok(state) = controller.state() {
                                turn_panel.set_roll_complete(state.rolls_used());
                            }
                        }
                    }
                    ControllerEvent::ScoreEntered {
                        player_index,
                        category,
                        score: _,
                    } => {
                        status_label.set_label(&i18n::get_str_int(
                            "score-entered-for-player",
                            "category",
                            &category.to_string(),
                            "player",
                            (player_index + 1) as i64,
                        ));
                        scorecard_view.clear_last_entered();
                        if let Some(ref controller) = *game_controller.borrow() {
                            if let Ok(state) = controller.state() {
                                scorecard_view.update_players(state.players(), state.current_player_index().get());
                                scorecard_view.set_last_entered(player_index, category);
                                player_bar.update_all_scores(state.players());
                            }
                        }
                    }
                    ControllerEvent::CategoryCrossedOut { player_index, category } => {
                        status_label.set_label(&i18n::get_str_int(
                            "category-crossed-out-for-player",
                            "category",
                            &category.to_string(),
                            "player",
                            (player_index + 1) as i64,
                        ));
                        scorecard_view.clear_last_entered();
                        if let Some(ref controller) = *game_controller.borrow() {
                            if let Ok(state) = controller.state() {
                                scorecard_view.update_players(state.players(), state.current_player_index().get());
                                scorecard_view.set_last_entered(player_index, category);
                                player_bar.update_all_scores(state.players());
                            }
                        }
                    }
                    ControllerEvent::TurnStarted { player_index } => {
                        player_bar.set_active_player(PlayerIndex::new(player_index));
                        *holds.borrow_mut() = HoldMask::none();
                        dice_view.update_holds(HoldMask::none());
                        status_label.set_label(&format!("Spieler {} ist an der Reihe.", player_index + 1));
                    }
                    ControllerEvent::PhaseChanged { phase: new_phase } => match new_phase {
                        crate::models::turn_phase::TurnPhase::Rolling => {
                            turn_panel.set_rolling();
                        }
                        crate::models::turn_phase::TurnPhase::Holding => {
                            if let Some(ref controller) = *game_controller.borrow() {
                                if let Ok(state) = controller.state() {
                                    turn_panel.set_roll_complete(state.rolls_used());
                                }
                            }
                        }
                        crate::models::turn_phase::TurnPhase::Scoring => {
                            turn_panel.set_roll_button_label(RollButtonLabel::NoRollsLeft);
                        }
                        crate::models::turn_phase::TurnPhase::AwaitingRoll => {
                            turn_panel.reset();
                        }
                        crate::models::turn_phase::TurnPhase::TurnEnd => {}
                    },
                    ControllerEvent::StatusChanged { status: _ } => {}
                    ControllerEvent::CrossOutRecommended { recommendation: _ } => {}
                    ControllerEvent::TurnTransition { transition: _ } => {}
                }
            }
            glib::ControlFlow::Continue
        });
    }
}

/// Convert face values from a roll event to a `DiceSet`.
///
/// Returns `None` if any face value is missing (dice didn't report).
fn faces_to_dice_set(faces: &[Option<dice_rs::FaceValue>; 5]) -> Option<DiceSet> {
    let values: [u8; 5] = faces.iter().map(|f| f.map(|v| v.get())).collect::<Option<Vec<u8>>>()?.try_into().ok()?;
    DiceSet::from_values(values).ok()
}

/// Convert a physical `DiceColor` to an `LedColor` for LED display.
fn dice_color_to_led(color: dice_rs::DiceColor) -> dice_rs::LedColor {
    match color {
        dice_rs::DiceColor::Black => dice_rs::LedColor::new(51, 51, 51),
        dice_rs::DiceColor::Red => dice_rs::LedColor::RED,
        dice_rs::DiceColor::Green => dice_rs::LedColor::GREEN,
        dice_rs::DiceColor::Blue => dice_rs::LedColor::BLUE,
        dice_rs::DiceColor::Yellow => dice_rs::LedColor::new(255, 200, 0),
        dice_rs::DiceColor::Orange => dice_rs::LedColor::new(255, 100, 0),
    }
}

/// Localized color name for a `DiceColor`, used in status messages.
fn dice_color_name(color: dice_rs::DiceColor) -> String {
    let key = match color {
        dice_rs::DiceColor::Black => "dice-color-black",
        dice_rs::DiceColor::Red => "dice-color-red",
        dice_rs::DiceColor::Green => "dice-color-green",
        dice_rs::DiceColor::Blue => "dice-color-blue",
        dice_rs::DiceColor::Yellow => "dice-color-yellow",
        dice_rs::DiceColor::Orange => "dice-color-orange",
    };
    i18n::get(key)
}
