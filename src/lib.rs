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

pub mod components;

use crate::components::*;

/// 이름이 Windows에서 예약되었는지를 확인합니다.
/// 
/// Windows 11과 Windows 10은 서로 다른 이름 정책을 사용합니다. Windows 10이 더 restrictive한 정책을 사용합니다.
/// strict_check가 true이면 Windows 10과 Windows 11에 모두 호환되는 예약어 검사가 사용되고,
/// false이면 Windows 11과만 호환되는 검사가 사용됩니다.
pub fn is_name_reserved(name: &String, strict: bool) -> bool {
    let reserved_names = if strict {
        NOT_ALLOWED_NAMES.to_vec()
    } else {
        NOT_ALLOWED_NAMES_WIN11.to_vec()
    };

    let name = name.to_uppercase();
    let name_vec = name.chars().collect::<Vec<_>>();

    if reserved_names.iter().any(|e| e == &name) {
        return true;
    }

    if !strict {
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
pub fn is_safe_name(name: &String, only_check_creatable: bool, strict: bool) -> bool {
    for not_allowed_char in NOT_ALLOWED_CHARS {
        for char_in_name in name.chars() {
            if not_allowed_char == char_in_name {
                return false;
            }
        }
    }

    if is_name_reserved(name, strict) {
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

    if strict && name.chars().next().unwrap_or(' ') == ' ' {
        return false;
    }

    true
}

/// 안전한 이름으로 변환된 이름을 리턴합니다.
pub fn to_safe_name(
    name: &String,
    compiled_replace_method: ReplaceMethodTableConstructor,
    dot_handling_policy: DotHandlingPolicy
) -> String {
    let table = compiled_replace_method.table;
    let replace_method = &compiled_replace_method.replace_method;
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
            if *last_char == '.' || *last_char == ' ' {
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
