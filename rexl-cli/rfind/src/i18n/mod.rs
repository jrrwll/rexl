use std::env;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Lang {
    English,
    SimplifiedChinese,
}

pub static mut LANG: Lang = Lang::English;

pub const USAGE_EN: &'static str = include_str!("./usage_en.txt");
pub const USAGE_ZH_CN: &'static str = include_str!("./usage_zh_CN.txt");

pub const MESSAGE_EN: &'static str = include_str!("./message_en.properties");
pub const MESSAGE_ZH_CN: &'static str = include_str!("./message_zh_CN.properties");

pub unsafe fn load_i18n_config() -> (&'static str, &'static str) {
    // detect lang
    if let Ok(lang) = env::var("LANG") {
        if lang.contains("zh_CN") {
            LANG = Lang::SimplifiedChinese;
        }
    }

    match LANG {
        Lang::English => {
            (USAGE_EN, MESSAGE_EN)
        }
        Lang::SimplifiedChinese => {
            (USAGE_ZH_CN, MESSAGE_ZH_CN)
        }
    }
}
