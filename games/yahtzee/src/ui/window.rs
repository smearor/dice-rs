use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use dice_rs::DiceManager;
use gtk4::glib;
use gtk4::prelude::*;
use tracing::warn;

use crate::models::dice_set::DiceSet;
use crate::models::game_state::GameState;
use crate::models::player::Player;
use crate::models::player_index::PlayerIndex;
use crate::models::reconnection_request::ReconnectionRequest;
use crate::services::dice_service::DiceService;
use crate::services::dice_service::DiceServiceEvent;
use crate::services::event_bridge::EventBridge;
use crate::services::event_bridge::GameEvent;
use crate::services::game_controller::ControllerEvent;
use crate::services::game_controller::GameController;
use crate::services::led_effect::LedEffect;
use crate::services::led_service::LedService;
use crate::services::reconnection_manager::ReconnectionManager;
use crate::services::roll_detector::RollDetector;
use crate::services::settings_store::SettingsStore;
use crate::services::slot_mapping::SlotMapping;
use crate::ui::models::game_phase::GamePhase;
use crate::ui::models::ui_message::UiMessage;
use crate::ui::widgets::dice_view::DiceView;
use crate::ui::widgets::game_end_screen::GameEndScreen;
use crate::ui::widgets::player_bar::PlayerBar;
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
    /// The status label.
    status_label: gtk4::Label,
    /// The scan button (setup phase).
    scan_button: gtk4::Button,
    /// The main content stack.
    content_stack: gtk4::Stack,
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
            .label("Nach GoDice scannen")
            .halign(gtk4::Align::Center)
            .build();

        let setup_title = gtk4::Label::builder()
            .label("Kniffel — Setup")
            .css_classes(vec!["setup-title"])
            .halign(gtk4::Align::Center)
            .build();

        let setup_instruction = gtk4::Label::builder()
            .label("Verbinde 5 GoDice, um das Spiel zu starten.")
            .css_classes(vec!["setup-instruction"])
            .halign(gtk4::Align::Center)
            .build();

        let status_label = gtk4::Label::builder()
            .label(UiMessage::ready().as_str())
            .css_classes(vec!["status-label", "dim"])
            .halign(gtk4::Align::Center)
            .build();

        let setup_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .css_classes(vec!["setup-screen"])
            .spacing(16)
            .halign(gtk4::Align::Center)
            .valign(gtk4::Align::Center)
            .build();

        setup_box.append(&setup_title);
        setup_box.append(&setup_instruction);
        setup_box.append(&scan_button);
        setup_box.append(&status_label);

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

        let scorecard_scrolled = gtk4::ScrolledWindow::builder()
            .child(scorecard_view.widget())
            .vexpand(true)
            .hexpand(true)
            .build();

        playing_box.append(&scorecard_scrolled);

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
        let title_label = gtk4::Label::builder().label("Kniffel").css_classes(vec!["header-title"]).build();
        header_bar.set_title_widget(Some(&title_label));

        let window = gtk4::ApplicationWindow::builder()
            .application(app)
            .title("Kniffel — Yahtzee")
            .default_width(900)
            .default_height(700)
            .child(&content_stack)
            .build();
        window.set_titlebar(Some(&header_bar));

        let reconnection_manager = Rc::new(ReconnectionManager::new());
        let settings_store = Rc::new(SettingsStore::default_path().unwrap_or_else(|_| SettingsStore::new("yahtzee_settings.json")));
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
            game_state: Rc::new(RefCell::new(None)),
            phase: Rc::new(RefCell::new(GamePhase::Setup)),
            player_bar,
            dice_view,
            scorecard_view,
            turn_panel,
            game_end_screen,
            reconnection_overlay,
            reconnection_revealer,
            status_label,
            scan_button,
            content_stack,
        };

        win.connect_signals();
        win.start_event_polling();
        win.load_settings();
        win
    }

    /// Connect all signal handlers.
    fn connect_signals(&self) {
        // Scan button
        {
            let dice_service = self.dice_service.clone();
            let status_label = self.status_label.clone();
            self.scan_button.connect_clicked(move |_| {
                status_label.set_label(UiMessage::scanning().as_str());
                dice_service.scan_and_connect();
            });
        }

        // Roll button
        {
            let roll_detector = self.roll_detector.clone();
            let turn_panel = self.turn_panel.clone();
            let dice_view = self.dice_view.clone();
            self.turn_panel.connect_roll(move || {
                roll_detector.reset();
                dice_view.set_rolling();
                turn_panel.set_rolling();
            });
        }

        // Dice view hold toggle
        {
            let status_label = self.status_label.clone();
            self.dice_view.connect_hold_toggled(move |slot, held| {
                let message = UiMessage::hold_toggled(slot.get(), held);
                status_label.set_label(message.as_str());
            });
        }

        // Scorecard category selection
        {
            let status_label = self.status_label.clone();
            self.scorecard_view.connect_category_selected(move |category| {
                let message = UiMessage::new(format!("Kategorie {category} ausgewählt."));
                status_label.set_label(message.as_str());
            });
        }

        // Game end screen: new game button
        {
            let content_stack = self.content_stack.clone();
            let phase = self.phase.clone();
            self.game_end_screen.new_game_button().connect_clicked(move |_| {
                *phase.borrow_mut() = GamePhase::Setup;
                content_stack.set_visible_child_name("setup");
            });
        }

        // Reconnection overlay: retry button
        {
            let status_label = self.status_label.clone();
            self.reconnection_overlay.retry_button().connect_clicked(move |_| {
                status_label.set_label("Verbinde erneut...");
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
        // Poll dice service events
        let mut dice_receiver = self.dice_service.subscribe();
        let status_label = self.status_label.clone();
        let event_bridge = self.event_bridge.clone();
        let mapping = self.mapping.clone();
        let content_stack = self.content_stack.clone();
        let phase = self.phase.clone();

        glib::timeout_add_local(Duration::from_millis(50), move || {
            while let Ok(event) = dice_receiver.try_recv() {
                match event {
                    DiceServiceEvent::ScanStarted => {}
                    DiceServiceEvent::NoDevicesFound => {
                        status_label.set_label(UiMessage::no_devices().as_str());
                    }
                    DiceServiceEvent::DevicesFound(count) => {
                        status_label.set_label(UiMessage::devices_found(count).as_str());
                    }
                    DiceServiceEvent::DiceAssigned { slot, name } => {
                        status_label.set_label(UiMessage::dice_connected(slot, &name).as_str());
                    }
                    DiceServiceEvent::DiceConnectionFailed { name, error } => {
                        status_label.set_label(UiMessage::connection_failed(&name, &error).as_str());
                    }
                    DiceServiceEvent::ScanFailed(error) => {
                        status_label.set_label(UiMessage::scan_failed(&error).as_str());
                    }
                    DiceServiceEvent::AllSlotsAssigned => {
                        status_label.set_label(UiMessage::all_dice_connected().as_str());
                        for (slot, dice) in mapping.assigned() {
                            event_bridge.start_listening(slot, dice);
                        }
                        event_bridge.start_roll_detector_forwarding();
                        *phase.borrow_mut() = GamePhase::Playing;
                        content_stack.set_visible_child_name("playing");
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

        glib::timeout_add_local(Duration::from_millis(50), move || {
            while let Ok(event) = game_receiver.try_recv() {
                match event {
                    GameEvent::RollStarted => {
                        dice_view.set_rolling();
                        turn_panel.set_rolling();
                        status_label.set_label(UiMessage::roll_started().as_str());
                    }
                    GameEvent::RollComplete { faces } => {
                        dice_view.clear_rolling();
                        dice_view.update_faces(&faces);
                        status_label.set_label(UiMessage::roll_complete().as_str());
                        // Forward to game controller
                        #[allow(clippy::collapsible_if)]
                        if let Some(ref controller) = *game_controller.borrow() {
                            if let Some(dice) = faces_to_dice_set(&faces) {
                                let _ = controller.dice_stable(dice);
                            }
                        }
                    }
                    GameEvent::RollTimedOut { faces } => {
                        dice_view.clear_rolling();
                        dice_view.update_faces(&faces);
                        status_label.set_label(UiMessage::roll_timed_out().as_str());
                        // Forward to game controller with partial dice
                        #[allow(clippy::collapsible_if)]
                        if let Some(ref controller) = *game_controller.borrow() {
                            if let Some(dice) = faces_to_dice_set(&faces) {
                                let _ = controller.dice_stable(dice);
                            }
                        }
                    }
                    GameEvent::DiceStable { slot: _, face: _ } => {
                        // Individual die became stable — we update on RollComplete
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
        glib::spawn_future_local(async move {
            match store.load().await {
                Ok(Some(settings)) => {
                    let msg = format!("Einstellungen geladen ({} Spieler)", settings.player_count());
                    status_label.set_label(&msg);
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
                        status_label.set_label("Spiel beendet!");
                    }
                    ControllerEvent::StatusChanged { status: _ } => {}
                    ControllerEvent::PhaseChanged { phase: _ } => {}
                    ControllerEvent::RollCompleted { dice: _ } => {}
                    ControllerEvent::ScoreEntered {
                        player_index: _,
                        category: _,
                        score: _,
                    } => {}
                    ControllerEvent::TurnStarted { player_index: _ } => {}
                    ControllerEvent::CrossOutRecommended { recommendation: _ } => {}
                    ControllerEvent::TurnTransition { transition: _ } => {}
                    ControllerEvent::CategoryCrossedOut { player_index: _, category: _ } => {}
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
