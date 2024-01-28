//! # rsfilename
//! 
//! Windows와 macOS, Linux에서 안전한 파일명을 만듭니다.
//! 
//! Python 구현을 보고 싶다면 다음 링크를 참고하세요: [pyfilename](https://github.com/ilotoki0804/pyfilename)
//! 
//! ```rust
//! use rsfilename::*;
//! assert_eq!("hello？.txt.．", simply_to_safe_name("  hello?.txt..", true));
//! ```

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
    fn get_char(&self) -> char {
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

/// 이름이 Windows에서 예약되었는지를 확인합니다.
/// 
/// Windows 11과 Windows 10은 서로 다른 이름 정책을 사용합니다. Windows 10이 더 restrictive한 정책을 사용합니다.
/// strict_check가 true이면 Windows 10과 Windows 11에 모두 호환되는 예약어 검사가 사용되고,
/// false이면 Windows 11과만 호환되는 검사가 사용됩니다.
pub fn is_name_reserved(name: &String, strict_check: bool) -> bool {
    let reserved_names = if strict_check {
        NOT_ALLOWED_NAMES.to_vec()
    } else {
        NOT_ALLOWED_NAMES_WIN11.to_vec()
    };

    let name = name.to_uppercase();
    let name_vec = name.chars().collect::<Vec<_>>();

    if reserved_names.iter().any(|e| e == &name) {
        return true;
    }

    if !strict_check {
        return false;
    }

    if name.len() >= 3 && reserved_names.iter().any(|e| e == &name_vec[..3].into_iter().collect::<String>()) {
        return &name_vec[3] == &'.';
    }

    if name.len() >= 4 && reserved_names.iter().any(|e| e == &name_vec[..4].into_iter().collect::<String>()) {
        return &name_vec[4] == &'.';
    }

    false
}

/// 이름이 안전하게 변경 없이 사용될 수 있는지 검사합니다.
/// 
/// Windows는 파일명에 대한 다음과 같은 세 가지 정책이 있습니다.
/// 
/// 1. 생성이 불가능한 경우: 오류가 나며 해당 파일명을 가진 폴더나 파일을 만드는 것을 거부합니다.
/// 1. 생성은 되지만 이름이 변경되는 경우: 오류가 나지 않고 생성도 되지만 조용히 다른 이름으로 변경됩니다.
/// 1. 일반적인 생성
/// 
/// only_check_creatable가 true일 경우 1번 경우에 해당하는 경우 false를 리턴하고, 나머지 경우에는 true일 리턴합니다.
/// false일 경우 1번, 2번 경우에 대해 false를 리턴하고, 3번 경우에 true를 리턴합니다.
/// 
/// strict_check는 예약어와 관련 있습니다. is_name_reserved의 문서를 참고하세요.
pub fn is_safe_name(name: &String, only_check_creatable: bool, strict_check: bool) -> bool {
    for not_allowed_char in NOT_ALLOWED_CHARS {
        for char_in_name in name.chars() {
            if not_allowed_char == char_in_name {
                return false;
            }
        }
    }

    if is_name_reserved(name, strict_check) {
        return false;
    }

    if only_check_creatable {
        return true;
    }

    if name.chars().next_back().unwrap_or('.') == '.' {
        return false;
    }

    if name.chars().next_back().unwrap_or(' ') == ' ' {
        return false;
    }

    if strict_check && name.chars().next().unwrap_or(' ') == ' ' {
        return false;
    }

    true
}

/// 안전한 이름으로 변환된 이름을 리턴합니다.
pub fn to_safe_name(
    name: &String,
    replace_method_table: ReplaceMethodTableConstructor,
    dot_handling_policy: DotHandlingPolicy
) -> String {
    let table = replace_method_table.table;
    let replace_method = &replace_method_table.replace_method;
    let mut name_chars: Vec<char> = name.chars().map(|chr| {
        if let Some(replaced) = table.get(&chr) {
            *replaced
        } else {
            chr
        }
    }).filter(|chr| *chr != '\0').collect();

    // Remove following/trailing spaces
    let length = name_chars.len();
    for i in 0..length {
        if name_chars[i] != ' ' {
            name_chars = name_chars[i..].to_vec();
            break;
        }
    }
    let length = name_chars.len();
    for i in (0..length).rev() {
        if name_chars[i] != ' ' {
            name_chars = name_chars[..=i].to_vec();
            break;
        }
    }

    let replace = |replace_char: &ReplaceChar, name_chars: &mut Vec<char>| {
        let chr = replace_char.get_char();
        if let Some(_) = table.get(&chr) {
            remove(name_chars);
        } else {
            let last_element = name_chars.last_mut().unwrap();
            *last_element = chr;
        }
    };

    fn remove(name_chars: &mut Vec<char>) {
        while let Some(last_char) = name_chars.last() {
            if *last_char == '.' {
                name_chars.pop();
            } else {
                break;
            }
        }
    }

    if name_chars.last() == Some(&'.') {
        match dot_handling_policy {
            DotHandlingPolicy::NotCorrect => {},
            DotHandlingPolicy::Replace(replace_char) => match replace_char {
                ReplaceChar::Space => panic!("Cannot replace to space. Use DotHandlingPolicy::Remove instead."),
                _ => replace(&replace_char, &mut name_chars),
            }
            DotHandlingPolicy::Remove => remove(&mut name_chars),
            DotHandlingPolicy::ReplaceWithReplaceMethod => match replace_method {
                ReplaceMethod::Fullwidth(_) => replace(&ReplaceChar::Charactor('．'), &mut name_chars),
                ReplaceMethod::Replace(replace_char) => match replace_char {
                    // ReplaceMethod는 이유 있게 ReplaceChar::Space를 가질 수 있기에
                    // ReplaceWithReplaceMethod는 ReplaceChar::Space를 금지하지 않고 조용히 remove를 사용한다.
                    ReplaceChar::Space => remove(&mut name_chars),
                    _ => replace(replace_char, &mut name_chars),
                },
                ReplaceMethod::Remove => remove(&mut name_chars),
            }
        }
    }

    let mut replace_char = match replace_method {
        ReplaceMethod::Fullwidth(replace_char) => replace_char.get_char(),
        ReplaceMethod::Remove => '_',
        ReplaceMethod::Replace(replace_char) => replace_char.get_char(),
    };
    if replace_char == '.' {
        replace_char = '_';
    }

    if is_name_reserved(&name_chars.clone().into_iter().collect::<String>(), true) {
        name_chars.insert(0, replace_char);
    }

    if name_chars.len() == 0 {
        name_chars.push(replace_char);
    }

    name_chars.clone().into_iter().collect::<String>()
}

/// 간단하게 안전한 파일명을 만듭니다.
/// 
/// to_safe_name은 인자가 많아 간단히 사용하기는 어렵습니다.
/// 간단히 사용할 목적으로 가장 무난한 인자를 선택해 사용하도록 제작된 함수입니다.
/// 
/// fullwidth가 true일 경우 반각에서 전각으로 변환합니다.
/// false일 경우 사용할 수 없는 문자를 underscore(`_`)로 대체합니다.
pub fn simply_to_safe_name(name: &str, fullwidth: bool) -> String {
    if fullwidth {
        to_safe_name(
            &name.to_string(),
            ReplaceMethod::Fullwidth(ReplaceChar::Underscore).compile(),
            DotHandlingPolicy::ReplaceWithReplaceMethod,
        )
    } else {
        to_safe_name(
            &name.to_string(),
            ReplaceMethod::Replace(ReplaceChar::Underscore).compile(),
            DotHandlingPolicy::ReplaceWithReplaceMethod,
        )
    }
}
