pub mod locales;

use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Locale {
    EnUs,
    EsEs,
    FrFr,
    DeDE,
    JaJp,
    ZhCn,
    PtBr,
    RuRu,
    KoKr,
    ItIt,
}

impl Locale {
    pub fn code(&self) -> &'static str {
        match self {
            Locale::EnUs => "en-US",
            Locale::EsEs => "es-ES",
            Locale::FrFr => "fr-FR",
            Locale::DeDE => "de-DE",
            Locale::JaJp => "ja-JP",
            Locale::ZhCn => "zh-CN",
            Locale::PtBr => "pt-BR",
            Locale::RuRu => "ru-RU",
            Locale::KoKr => "ko-KR",
            Locale::ItIt => "it-IT",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Locale::EnUs => "English",
            Locale::EsEs => "Español",
            Locale::FrFr => "Français",
            Locale::DeDE => "Deutsch",
            Locale::JaJp => "日本語",
            Locale::ZhCn => "简体中文",
            Locale::PtBr => "Português",
            Locale::RuRu => "Русский",
            Locale::KoKr => "한국어",
            Locale::ItIt => "Italiano",
        }
    }
}

impl Default for Locale {
    fn default() -> Self {
        Locale::EnUs
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct I18n {
    locale: RwSignal<Locale>,
    translations: HashMap<Locale, HashMap<&'static str, &'static str>>,
}

impl I18n {
    pub fn new(locale: Locale) -> Self {
        let mut translations = HashMap::new();
        translations.insert(Locale::EnUs, locales::en_us::translations());
        translations.insert(Locale::EsEs, locales::es_es::translations());
        translations.insert(Locale::FrFr, locales::fr_fr::translations());

        Self {
            locale: RwSignal::new(locale),
            translations,
        }
    }

    /// Translate a key to the current locale
    pub fn t(&self, key: &str) -> String {
        let locale = self.locale.get();
        self.translations
            .get(&locale)
            .and_then(|t| t.get(key))
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                log::warn!("Missing translation for key: {} in locale: {:?}", key, locale);
                key.to_string()
            })
    }

    /// Get the current locale
    pub fn locale(&self) -> Locale {
        self.locale.get()
    }

    /// Set the current locale
    pub fn set_locale(&self, locale: Locale) {
        self.locale.set(locale);
    }

    /// Get a signal for the current locale
    pub fn locale_signal(&self) -> Signal<Locale> {
        self.locale.into()
    }
}

/// Provide i18n in the Leptos context
pub fn provide_i18n(locale: Locale) -> I18n {
    let i18n = I18n::new(locale);
    provide_context(i18n.clone());
    i18n
}

/// Get i18n from Leptos context
pub fn use_i18n() -> I18n {
    expect_context::<I18n>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_code() {
        assert_eq!(Locale::EnUs.code(), "en-US");
        assert_eq!(Locale::EsEs.code(), "es-ES");
        assert_eq!(Locale::FrFr.code(), "fr-FR");
    }

    #[test]
    fn test_locale_name() {
        assert_eq!(Locale::EnUs.name(), "English");
        assert_eq!(Locale::EsEs.name(), "Español");
        assert_eq!(Locale::FrFr.name(), "Français");
    }
}
