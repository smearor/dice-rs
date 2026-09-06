use crate::i18n;
use crate::models::highscore::HighscoreList;
use gtk4::prelude::*;

/// Dialog displaying the highscore list.
///
/// Shows a ranked list of the top scores achieved across all game
/// sessions. The list is loaded from disk via `HighscoreStore`.
pub struct HighscoreDialog {
    /// The GTK window.
    dialog: gtk4::Window,
}

impl HighscoreDialog {
    /// Create a new highscore dialog.
    ///
    /// If `list` contains entries, they are displayed in a ranked table.
    /// If the list is empty, a placeholder message is shown instead.
    pub fn new(parent: &gtk4::ApplicationWindow, list: &HighscoreList) -> Self {
        let dialog = gtk4::Window::builder()
            .title(&i18n::get("highscore-dialog-title"))
            .modal(true)
            .transient_for(parent)
            .default_width(400)
            .default_height(500)
            .build();

        let content = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(16)
            .margin_start(24)
            .margin_end(24)
            .margin_top(24)
            .margin_bottom(24)
            .build();

        let title = gtk4::Label::builder()
            .label(&i18n::get("highscore-title"))
            .css_classes(vec!["highscore-dialog-title"])
            .halign(gtk4::Align::Center)
            .build();
        content.append(&title);

        if list.is_empty() {
            let empty_label = gtk4::Label::builder()
                .label(&i18n::get("highscore-empty"))
                .css_classes(vec!["dim-label", "highscore-empty"])
                .halign(gtk4::Align::Center)
                .valign(gtk4::Align::Center)
                .vexpand(true)
                .build();
            content.append(&empty_label);
        } else {
            let scrolled = gtk4::ScrolledWindow::builder().hexpand(true).vexpand(true).build();

            let standings_list = gtk4::ListBox::builder()
                .css_classes(vec!["highscore-list"])
                .halign(gtk4::Align::Center)
                .hexpand(true)
                .build();

            for (index, entry) in list.entries().iter().enumerate() {
                let row = gtk4::Box::builder()
                    .orientation(gtk4::Orientation::Horizontal)
                    .spacing(16)
                    .css_classes(vec!["highscore-row"])
                    .halign(gtk4::Align::Center)
                    .build();

                let rank_label = gtk4::Label::builder()
                    .label(format!("{}.", index + 1))
                    .css_classes(vec!["highscore-rank"])
                    .build();

                let name_label = gtk4::Label::builder()
                    .label(entry.player_name())
                    .css_classes(vec!["highscore-name"])
                    .hexpand(true)
                    .halign(gtk4::Align::Start)
                    .build();

                let score_label = gtk4::Label::builder()
                    .label(i18n::get_int("highscore-score", "score", entry.score().get() as i64))
                    .css_classes(vec!["highscore-score"])
                    .build();

                row.append(&rank_label);
                row.append(&name_label);
                row.append(&score_label);

                let list_row = gtk4::ListBoxRow::builder().child(&row).build();
                standings_list.append(&list_row);
            }

            scrolled.set_child(Some(&standings_list));
            content.append(&scrolled);
        }

        let close_button = gtk4::Button::builder()
            .label(&i18n::get("highscore-close"))
            .css_classes(vec!["suggested-action", "highscore-close-button"])
            .halign(gtk4::Align::Center)
            .build();
        content.append(&close_button);

        {
            let dialog_clone = dialog.clone();
            close_button.connect_clicked(move |_| {
                dialog_clone.close();
            });
        }

        dialog.set_child(Some(&content));

        Self { dialog }
    }

    /// Present the dialog.
    pub fn present(&self) {
        self.dialog.present();
    }
}
