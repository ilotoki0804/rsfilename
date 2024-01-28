use std::collections::HashMap;
use std::iter::zip;

pub const NOT_ALLOWED_NAMES_WIN11: [&str; 28] = [
    "CON", "PRN", "AUX", "NUL",
    "COM1", "COM¹", "COM2", "COM²", "COM3", "COM³", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
    "LPT1", "LPT¹", "LPT2", "LPT²", "LPT3", "LPT³", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];
pub const NOT_ALLOWED_NAMES: [&str; 30] = [
    "CON", "PRN", "AUX", "NUL",
    "COM1", "COM¹", "COM2", "COM²", "COM3", "COM³", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
    "LPT1", "LPT¹", "LPT2", "LPT²", "LPT3", "LPT³", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    "COM0", "LPT0",
];
pub const NOT_ALLOWED_CHARS: [char; 41] = [
    '\0', '\u{1}', '\u{2}', '\u{3}', '\u{4}', '\u{5}',
    '\u{6}', '\u{7}', '\u{8}', '\t', '\n', '\u{b}',
    '\u{c}', '\r', '\u{e}', '\u{f}', '\u{10}', '\u{11}',
    '\u{12}', '\u{13}', '\u{14}', '\u{15}', '\u{16}',
    '\u{17}', '\u{18}', '\u{19}', '\u{1a}', '\u{1b}',
    '\u{1c}', '\u{1d}', '\u{1e}', '\u{1f}',
    '\\', '/', ':', '*', '?', '"', '<', '>', '|',
];

/// 허용되지 않는 문자가 왔을 때 어떤 문자로 대체할 것인지를 결정합니다.
/// 
/// Replace일 경우 모든 허용되지 않는 문자를 해당 문자로 치환합니다.
/// 
/// Fullwidth인 경우 몇몇 반각 문자를 전각 문자로 변경합니다.
/// 예를 들어 반각 문자 `?`은 전각 문자인 `？`으로 변경됩니다.
/// 
/// Remove인 경우 허용되지 않는 문자를 전부 삭제합니다.
#[derive(Debug)]
pub enum ReplaceMethod {
    Fullwidth(ReplaceChar),
    Replace(ReplaceChar),
    Remove,
}

/// 직접 만드는 대신 ReplaceMethod.compile()을 이용하세요.
pub struct ReplaceMethodTableConstructor {
    pub replace_method: ReplaceMethod,
    pub table: HashMap<char, char>,
}

impl ReplaceMethodTableConstructor {
    fn new(replace_method: ReplaceMethod) -> Self {
        let construct_table = Self::construct_table(&replace_method);
        Self {
            replace_method,
            table: construct_table,
        }
    }

    /// 기존 문자에 변경될 문자를 각각 대응한 HashMap을 생성합니다.
    fn construct_table(replace_method: &ReplaceMethod) -> HashMap<char, char> {
        match replace_method {
            ReplaceMethod::Fullwidth(replace_char) => {
                let mut table = HashMap::new();
                for i in 0..32 {
                    table.insert(char::from(i), replace_char.get_char());
                }
                for (original, fullwidth_replace) in zip("\\/:*?\"<>|".chars(), "⧵／：＊？＂＜＞∣".chars()) {
                    table.insert(original, fullwidth_replace);
                }
                table
            },
            ReplaceMethod::Replace(replace_char) => {
                let mut table = HashMap::new();
                for i in 0..32 {
                    table.insert(char::from(i), replace_char.get_char());
                }
                for original in "\\/:*?\"<>|".chars() {
                    table.insert(original, replace_char.get_char());
                }
                table
            }
            ReplaceMethod::Remove => Self::construct_table(
                &ReplaceMethod::Replace(ReplaceChar::Charactor('\0'))
            ),
        }
    }
}

impl ReplaceMethod {
    pub fn compile(self) -> ReplaceMethodTableConstructor {
        ReplaceMethodTableConstructor::new(self)
    }
}

/// 이름의 맨 마지막에 붙는 마침표를 어떻게 처리할지를 결정합니다.
/// 
/// Remove는 맨 마지막에 붙는 마침표를 삭제합니다.
/// 
/// Replace는 ReplaceChar로 변경합니다.
/// 
/// NotCorrect는 마침표를 삭제하지 않습니다.
/// 
/// ReplaceWithReplaceMethod는 ReplaceMethod의 방식과 공유합니다.
pub enum DotHandlingPolicy {
    Remove,
    Replace(ReplaceChar),
    ReplaceWithReplaceMethod,
    NotCorrect,
}

/// 대체할 문자를 설정합니다. 그냥 자주 사용될 수 있는 대체 문자를 모아놓은 것이고, 다른 의미는 없습니다.
/// 
/// 임의의 대체 문자를 사용하려면 Charactor(char)를 사용하세요.
#[derive(Debug)]
pub enum ReplaceChar {
    Space,  // ' '
    DubleQuestionMark, // '⁇'
    WhiteQuestionMark, // '❔'
    RedQuestionMark, // '❓'
    Underscore, // '_'
    Charactor(char),
}

impl ReplaceChar {
    pub fn get_char(&self) -> char {
        match self {
            Self::Space => ' ',
            Self::DubleQuestionMark => '⁇',
            Self::WhiteQuestionMark => '❔',
            Self::RedQuestionMark => '❓',
            Self::Underscore => '_',
            Self::Charactor(charactor) => *charactor,
        }
    }
}