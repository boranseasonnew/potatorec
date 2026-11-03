//! All catalogs are embedded. No translation network calls at runtime.
use potato_core::config::Language;
use std::{borrow::Cow, collections::BTreeMap, sync::OnceLock};

const FILES: [(&str, &str); 23] = [
    ("en", include_str!("../../locales/en.json")),
    ("tr", include_str!("../../locales/tr.json")),
    ("az", include_str!("../../locales/az.json")),
    ("ru", include_str!("../../locales/ru.json")),
    ("de", include_str!("../../locales/de.json")),
    ("fr", include_str!("../../locales/fr.json")),
    ("es", include_str!("../../locales/es.json")),
    ("pt-BR", include_str!("../../locales/pt-BR.json")),
    ("it", include_str!("../../locales/it.json")),
    ("nl", include_str!("../../locales/nl.json")),
    ("pl", include_str!("../../locales/pl.json")),
    ("uk", include_str!("../../locales/uk.json")),
    ("ro", include_str!("../../locales/ro.json")),
    ("hu", include_str!("../../locales/hu.json")),
    ("cs", include_str!("../../locales/cs.json")),
    ("sk", include_str!("../../locales/sk.json")),
    ("bg", include_str!("../../locales/bg.json")),
    ("sr", include_str!("../../locales/sr.json")),
    ("hr", include_str!("../../locales/hr.json")),
    ("el", include_str!("../../locales/el.json")),
    ("sv", include_str!("../../locales/sv.json")),
    ("da", include_str!("../../locales/da.json")),
    ("fi", include_str!("../../locales/fi.json")),
];
type Catalogs = BTreeMap<&'static str, BTreeMap<String, String>>;
static CATALOGS: OnceLock<Catalogs> = OnceLock::new();
fn catalogs() -> &'static Catalogs {
    CATALOGS.get_or_init(|| {
        FILES
            .iter()
            .map(|(code, data)| {
                (
                    *code,
                    serde_json::from_str(data).expect("Embedded catalog validated by tests"),
                )
            })
            .collect()
    })
}
pub fn tr(language: Language, key: &str) -> &str {
    catalogs()
        .get(language.code())
        .and_then(|c| c.get(key))
        .map(String::as_str)
        .unwrap_or(key)
}
pub fn current(ui: &eframe::egui::Ui) -> Language {
    ui.ctx()
        .data(|d| d.get_temp::<Language>(eframe::egui::Id::new("potato-language")))
        .unwrap_or(Language::English)
}
pub fn fill(language: Language, key: &str, values: &[String]) -> String {
    let mut out = tr(language, key).to_owned();
    for (index, value) in values.iter().enumerate() {
        out = out.replace(&format!("{{{index}}}"), value);
    }
    out
}
pub fn message(language: Language, text: &str) -> Cow<'_, str> {
    let translated = tr(language, text);
    if translated != text {
        return Cow::Borrowed(translated);
    }
    if let Some((prefix, details)) = text.split_once(':') {
        let prefix_translation = tr(language, prefix);
        if prefix_translation != prefix {
            return Cow::Owned(format!("{prefix_translation}: {details}"));
        }
    }
    Cow::Borrowed(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_23_catalogs_and_guides_have_complete_keys() {
        assert_eq!(Language::ALL.len(), 23);
        let en = &catalogs()["en"];
        for l in Language::ALL {
            let pack = &catalogs()[l.code()];
            assert_eq!(
                pack.keys().collect::<Vec<_>>(),
                en.keys().collect::<Vec<_>>(),
                "{} key coverage",
                l.code()
            );
            assert!(pack.values().all(|v| !v.trim().is_empty()));
            let guide = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("docs/languages")
                .join(format!("{}.md", l.code()));
            assert!(std::fs::read_to_string(guide).unwrap().len() > 1000);
        }
    }
    #[test]
    fn requested_languages_have_native_navigation() {
        assert_eq!(tr(Language::Azerbaijani, "Settings"), "Parametrlər");
        assert_eq!(tr(Language::Russian, "Settings"), "Настройки");
        assert_eq!(tr(Language::Turkish, "Settings"), "Ayarlar");
    }
    #[test]
    fn region_mapping_and_saved_enum_are_stable() {
        assert_eq!(Language::from_code("az-Latn-AZ"), Language::Azerbaijani);
        assert_eq!(Language::from_code("ru-RU"), Language::Russian);
        assert_eq!(Language::from_code("pt_PT"), Language::Portuguese);
        for l in Language::ALL {
            let saved = serde_json::to_string(l).unwrap();
            assert_eq!(*l, serde_json::from_str::<Language>(&saved).unwrap());
        }
    }
    #[test]
    fn translated_templates_keep_numbered_values() {
        for l in Language::ALL {
            for (key, value) in &catalogs()[l.code()] {
                for i in 0..5 {
                    let marker = format!("{{{i}}}");
                    assert_eq!(
                        key.matches(&marker).count(),
                        value.matches(&marker).count(),
                        "{}: {key}",
                        l.code()
                    );
                }
            }
            let result = fill(*l, "{0} available", &["1.5 GiB".into()]);
            assert!(result.contains("1.5 GiB"));
            assert!(!result.contains("{0}"));
        }
    }
}
