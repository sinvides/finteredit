use std::collections::HashMap;
use std::sync::OnceLock;

use mod_api::{PlayerRef, ServerHost, Text};

static TRANSLATIONS: OnceLock<Translations> = OnceLock::new();
static CURRENT_LANG: OnceLock<String> = OnceLock::new();

struct Translations {
    en_us: HashMap<String, String>,
    ru_ru: HashMap<String, String>,
}

fn current_lang() -> &'static str {
    CURRENT_LANG.get_or_init(|| "en_us".to_string())
}

fn load_lang_file(path: &str) -> HashMap<String, String> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return HashMap::new(),
    };
    serde_json::from_str(&content).unwrap_or_default()
}

fn load_translations() -> Translations {
    let en_us = load_lang_file("assets/finteredit/lang/en_us.json");
    let ru_ru = load_lang_file("assets/finteredit/lang/ru_ru.json");
    Translations { en_us, ru_ru }
}

fn translations() -> &'static Translations {
    TRANSLATIONS.get_or_init(load_translations)
}

fn get_template(key: &str) -> Option<String> {
    let t = translations();
    let lang = current_lang();
    match lang {
        "ru_ru" => t.ru_ru.get(key).cloned().or_else(|| t.en_us.get(key).cloned()),
        _ => t.en_us.get(key).cloned().or_else(|| t.ru_ru.get(key).cloned()),
    }
}

fn format_args(template: &str, args: &[&str]) -> String {
    let mut result = String::new();
    let mut parts = template.splitn(args.len() + 1, "%s");
    for (i, part) in parts.by_ref().enumerate() {
        result.push_str(part);
        if i < args.len() {
            result.push_str(args[i]);
        }
    }
    result
}

fn t(key: &str, args: &[&String]) -> Text {
    let template = get_template(key).unwrap_or_else(|| key.to_string());
    let text = format_args(&template, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    Text::literal(text)
}

pub fn tell(host: &mut dyn ServerHost, player: PlayerRef, mut text: Text) {
    text.style.color = Some("#CC6600".to_owned());
    let _ = player.tell(host, text);
}

pub fn blocks_word(lang: &str, count: i64) -> &'static str {
    if lang == "ru_ru" {
        let n = count % 100;
        if n >= 11 && n <= 14 {
            return "блоков";
        }
        match count % 10 {
            1 => "блок",
            2 | 3 | 4 => "блока",
            _ => "блоков",
        }
    } else {
        if count == 1 {
            "block"
        } else {
            "blocks"
        }
    }
}

pub fn first_point(pos: [i32; 3]) -> Text {
    let count = 1;
    let word = blocks_word(current_lang(), count);
    let pos0 = pos[0].to_string();
    let pos1 = pos[1].to_string();
    let pos2 = pos[2].to_string();
    let cnt_word = format!("{} {}", count, word);
    let args = [&pos0, &pos1, &pos2, &cnt_word];
    t("finteredit.point.first", &args)
}

pub fn second_point(pos: [i32; 3], count: i64) -> Text {
    let word = blocks_word(current_lang(), count);
    let pos0 = pos[0].to_string();
    let pos1 = pos[1].to_string();
    let pos2 = pos[2].to_string();
    let cnt = count.to_string();
    let w = word.to_string();
    let args = [&pos0, &pos1, &pos2, &cnt, &w];
    t("finteredit.point.second", &args)
}

pub fn filled(count: i64) -> Text {
    let word = blocks_word(current_lang(), count);
    let cnt = count.to_string();
    let w = word.to_string();
    let args = [&cnt, &w];
    t("finteredit.fill.done", &args)
}

pub fn undone(count: usize, failed: usize) -> Text {
    let word = blocks_word(current_lang(), count as i64);
    let cnt = count.to_string();
    let fl = failed.to_string();
    let w = word.to_string();
    let args = [&cnt, &w, &fl];
    t("finteredit.undo.done", &args)
}

pub fn unknown_block(name: &str) -> Text {
    let n = name.to_string();
    let args = [&n];
    t("finteredit.error.unknown_block", &args)
}

pub fn no_selection() -> Text {
    t("finteredit.error.no_selection", &[])
}

pub fn no_second_point() -> Text {
    t("finteredit.error.no_second", &[])
}

pub fn no_undo() -> Text {
    t("finteredit.error.no_undo", &[])
}

pub fn too_large(max: i64) -> Text {
    let word = blocks_word(current_lang(), max);
    let mx = max.to_string();
    let w = word.to_string();
    let args = [&mx, &w];
    t("finteredit.error.too_large", &args)
}

pub fn wrong_dimension() -> Text {
    t("finteredit.error.dimension", &[])
}

pub fn copied(count: usize) -> Text {
    let word = blocks_word(current_lang(), count as i64);
    let cnt = count.to_string();
    let w = word.to_string();
    let args = [&cnt, &w];
    t("finteredit.copy.done", &args)
}

pub fn pasted(count: usize) -> Text {
    let word = blocks_word(current_lang(), count as i64);
    let cnt = count.to_string();
    let w = word.to_string();
    let args = [&cnt, &w];
    t("finteredit.paste.done", &args)
}

pub fn no_clipboard() -> Text {
    t("finteredit.error.no_clipboard", &[])
}
