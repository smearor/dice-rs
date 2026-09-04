//! Internationalization support using the Fluent ecosystem.
//!
//! Embeds `.ftl` translation files at compile time via `rust-embed`
//! and loads the system locale at runtime via `i18n-embed`.

use std::sync::OnceLock;

use i18n_embed::DesktopLanguageRequester;
use i18n_embed::fluent::FluentLanguageLoader;
use i18n_embed::fluent::fluent_language_loader;
use rust_embed::RustEmbed;

/// Embeds the `i18n/` directory containing `.ftl` files into the binary.
#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

/// The global fluent language loader.
///
/// Initialized once at first access with the system's preferred language,
/// falling back to German (`de`) as the default/fallback language.
static LOADER: OnceLock<FluentLanguageLoader> = OnceLock::new();

/// Get the global language loader, initializing it on first access.
pub fn loader() -> &'static FluentLanguageLoader {
    LOADER.get_or_init(|| {
        let loader = fluent_language_loader!();

        let requested = DesktopLanguageRequester::requested_languages();
        let languages = i18n_embed::select(&loader, &Localizations, &requested);

        if let Err(error) = languages {
            tracing::warn!(error = %error, "failed to load requested languages, falling back to default");
        }

        loader
    })
}

/// Convenience macro wrapping `i18n_embed_fl::fl!` with the static loader.
///
/// Usage:
/// ```
/// use yahtzee::fl;
///
/// let label = fl!("roll-button-roll");
/// ```
#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::loader(), $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::loader(), $message_id, $($args), *)
    }};
}

/// Convenience macro for writing a translated message to a formatter.
///
/// Wraps `write!(f, "{}", fl!("key"))` into `fl_write!(f, "key")`.
/// Supports optional fluent arguments: `fl_write!(f, "key", arg = value)`.
///
/// Usage:
/// ```
/// use std::fmt::Display;
/// use yahtzee::fl_write;
///
/// struct MyType;
/// impl Display for MyType {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         fl_write!(f, "roll-button-roll")
///     }
/// }
/// ```
#[macro_export]
macro_rules! fl_write {
    ($f:expr, $message_id:literal) => {
        write!($f, "{}", $crate::fl!($message_id))
    };

    ($f:expr, $message_id:literal, $($args:expr),*) => {
        write!($f, "{}", $crate::fl!($message_id, $($args), *))
    };
}

/// Get a translated message by ID without compile-time checks.
///
/// Use this for dynamic message IDs (e.g. category names looked up by key).
/// For static messages, prefer the `fl!` macro.
pub fn get(message_id: &str) -> String {
    loader().get(message_id)
}

/// Trait for types that map to a Fluent translation key.
///
/// Implementors provide `fluent_key()` and get `localized()` for free.
/// This ensures a consistent pattern across all localizable enum types.
pub trait Localized {
    /// Returns the Fluent message key for this value.
    fn fluent_key(&self) -> &'static str;

    /// Returns the localized display name by looking up the Fluent key.
    fn localized(&self) -> String {
        get(self.fluent_key())
    }
}

/// Implements `std::fmt::Display` for a type implementing `Localized`.
///
/// Usage:
/// ```
/// use yahtzee::i18n::Localized;
/// use yahtzee::impl_display_localized;
///
/// struct MyType;
/// impl Localized for MyType {
///     fn fluent_key(&self) -> &'static str { "my-type" }
/// }
/// impl_display_localized!(MyType);
/// ```
#[macro_export]
macro_rules! impl_display_localized {
    ($type_name:ty) => {
        impl std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", $crate::i18n::Localized::localized(self))
            }
        }
    };
}

/// Get a translated message with a single integer argument.
pub fn get_int(message_id: &str, arg_name: &str, value: i64) -> String {
    let mut args = fluent_bundle::FluentArgs::with_capacity(1);
    args.set(arg_name, value);
    loader().get_args_fluent(message_id, Some(&args))
}

/// Get a translated message with a single string argument.
pub fn get_str(message_id: &str, arg_name: &str, value: &str) -> String {
    let mut args = fluent_bundle::FluentArgs::with_capacity(1);
    args.set(arg_name, value);
    loader().get_args_fluent(message_id, Some(&args))
}

/// Get a translated message with two string arguments.
pub fn get_str_str(message_id: &str, arg1: &str, val1: &str, arg2: &str, val2: &str) -> String {
    let mut args = fluent_bundle::FluentArgs::with_capacity(2);
    args.set(arg1, val1);
    args.set(arg2, val2);
    loader().get_args_fluent(message_id, Some(&args))
}

/// Get a translated message with a string and an integer argument.
pub fn get_str_int(message_id: &str, arg1: &str, val1: &str, arg2: &str, val2: i64) -> String {
    let mut args = fluent_bundle::FluentArgs::with_capacity(2);
    args.set(arg1, val1);
    args.set(arg2, val2);
    loader().get_args_fluent(message_id, Some(&args))
}

/// Get a translated message with an integer and a string argument.
pub fn get_int_str(message_id: &str, arg1: &str, val1: i64, arg2: &str, val2: &str) -> String {
    let mut args = fluent_bundle::FluentArgs::with_capacity(2);
    args.set(arg1, val1);
    args.set(arg2, val2);
    loader().get_args_fluent(message_id, Some(&args))
}

/// Get a translated message with two string arguments and one integer argument.
pub fn get_str_str_int(message_id: &str, arg1: &str, val1: &str, arg2: &str, val2: &str, arg3: &str, val3: i64) -> String {
    let mut args = fluent_bundle::FluentArgs::with_capacity(3);
    args.set(arg1, val1);
    args.set(arg2, val2);
    args.set(arg3, val3);
    loader().get_args_fluent(message_id, Some(&args))
}

/// Get a translated message with two integer arguments.
pub fn get_int_int(message_id: &str, arg1: &str, val1: i64, arg2: &str, val2: i64) -> String {
    let mut args = fluent_bundle::FluentArgs::with_capacity(2);
    args.set(arg1, val1);
    args.set(arg2, val2);
    loader().get_args_fluent(message_id, Some(&args))
}

/// Initialize the i18n system.
///
/// This forces the lazy loader to be initialized with the system locale.
/// Call this early in `main()` to ensure translations are ready before
/// any UI code runs.
pub fn init() {
    let _ = loader();
}
