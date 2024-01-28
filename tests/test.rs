use rsfilename::*;
use std::collections::HashMap;

#[test]
fn test_construct_table1() {
    let result = HashMap::from([
        ('\u{7}', '!'),
        ('\u{c}', '!'),
        ('\u{10}', '!'),
        ('\u{1a}', '!'),
        ('\n', '!'),
        ('\u{1d}', '!'),
        ('\u{4}', '!'),
        ('\u{12}', '!'),
        ('|', '!'),
        ('\u{2}', '!'),
        ('\u{1f}', '!'),
        ('?', '!'),
        ('<', '!'),
        ('\u{3}', '!'),
        ('\u{15}', '!'),
        ('\r', '!'),
        ('\u{14}', '!'),
        ('\u{1c}', '!'),
        ('/', '!'),
        ('\u{1e}', '!'),
        ('\\', '!'),
        ('"', '!'),
        ('\u{b}', '!'),
        ('\u{f}', '!'),
        (':', '!'),
        ('>', '!'),
        ('\u{16}', '!'),
        ('\u{17}', '!'),
        ('*', '!'),
        ('\u{8}', '!'),
        ('\u{6}', '!'),
        ('\u{18}', '!'),
        ('\u{13}', '!'),
        ('\u{e}', '!'),
        ('\0', '!'),
        ('\u{1}', '!'),
        ('\u{1b}', '!'),
        ('\t', '!'),
        ('\u{11}', '!'),
        ('\u{5}', '!'),
        ('\u{19}', '!'),
    ]);

    let replace_method = ReplaceMethod::Replace(ReplaceChar::Charactor('!'));
    assert_eq!(result, replace_method.construct_table());
}

#[test]
fn test_construct_table2() {
    let result = HashMap::from([
        ('\u{1f}', '!'),
        ('>', '＞'),
        ('\u{17}', '!'),
        ('\u{2}', '!'),
        ('\u{1d}', '!'),
        ('\0', '!'),
        ('<', '＜'),
        ('\u{14}', '!'),
        ('\u{15}', '!'),
        ('\u{1e}', '!'),
        ('\u{6}', '!'),
        ('\u{12}', '!'),
        ('\t', '!'),
        ('/', '／'),
        ('\u{1}', '!'),
        ('?', '？'),
        ('\u{7}', '!'),
        ('\u{c}', '!'),
        ('\u{f}', '!'),
        ('\u{1a}', '!'),
        ('"', '＂'),
        ('\n', '!'),
        ('\u{8}', '!'),
        ('\u{4}', '!'),
        ('*', '＊'),
        (':', '：'),
        ('\u{18}', '!'),
        ('\u{b}', '!'),
        ('\u{13}', '!'),
        ('\\', '⧵'),
        ('\u{1c}', '!'),
        ('\u{5}', '!'),
        ('\r', '!'),
        ('\u{19}', '!'),
        ('\u{1b}', '!'),
        ('\u{3}', '!'),
        ('\u{11}', '!'),
        ('\u{e}', '!'),
        ('\u{10}', '!'),
        ('\u{16}', '!'),
        ('|', '∣'),
    ]);

    let replace_method = ReplaceMethod::Fullwidth(ReplaceChar::Charactor('!'));
    assert_eq!(result, replace_method.construct_table());
}

#[test]
fn test_construct_table3() {
    let result = HashMap::from([
        ('\u{11}', '\0'),
        ('\u{2}', '\0'),
        ('\u{8}', '\0'),
        ('\u{15}', '\0'),
        ('\u{17}', '\0'),
        ('\u{18}', '\0'),
        ('\u{19}', '\0'),
        ('\u{c}', '\0'),
        ('\u{7}', '\0'),
        ('\u{1c}', '\0'),
        (':', '\0'),
        ('\n', '\0'),
        ('\u{10}', '\0'),
        ('\u{1d}', '\0'),
        ('*', '\0'),
        ('\u{1b}', '\0'),
        ('\\', '\0'),
        ('\u{4}', '\0'),
        ('"', '\0'),
        ('\r', '\0'),
        ('<', '\0'),
        ('\u{1a}', '\0'),
        ('/', '\0'),
        ('>', '\0'),
        ('\t', '\0'),
        ('\u{b}', '\0'),
        ('\u{e}', '\0'),
        ('\u{3}', '\0'),
        ('\u{14}', '\0'),
        ('?', '\0'),
        ('\u{5}', '\0'),
        ('\u{13}', '\0'),
        ('|', '\0'),
        ('\u{f}', '\0'),
        ('\u{12}', '\0'),
        ('\u{1e}', '\0'),
        ('\u{1}', '\0'),
        ('\0', '\0'),
        ('\u{6}', '\0'),
        ('\u{16}', '\0'),
        ('\u{1f}', '\0'),
    ]);

    let replace_method = ReplaceMethod::Remove;
    assert_eq!(result, replace_method.construct_table());
}

#[test]
fn test_is_name_reserved() {
    assert!(!is_name_reserved(&"hello!".to_string(), true));
    assert!(!is_name_reserved(&"nul hello.txt".to_string(), true));
    assert!(!is_name_reserved(&"com1 hello.txt".to_string(), true));

    assert!(is_name_reserved(&"NUL".to_string(), true));
    assert!(is_name_reserved(&"nul".to_string(), true));
    assert!(is_name_reserved(&"nul.txt".to_string(), true));
    assert!(is_name_reserved(&"nul. hello.txt".to_string(), true));

    assert!(is_name_reserved(&"COM1".to_string(), true));
    assert!(is_name_reserved(&"com1".to_string(), true));
    assert!(is_name_reserved(&"com1.txt".to_string(), true));
    assert!(is_name_reserved(&"com1. this is the world .txt".to_string(), true));
}

#[test]
fn test_is_safe_name() {
    // 사용 불가능 문자 검사
    assert!(!is_safe_name(&String::from("hell?o.txt."), true, true));
    assert!(!is_safe_name(&String::from("hell*o.txt."), true, true));

    // 예약어 검사
    assert!(!is_safe_name(&String::from("com2.hello.txt."), true, true));
    assert!(!is_safe_name(&String::from("nul.hello.txt."), true, true));
    assert!(!is_safe_name(&String::from("nul."), true, true));
    assert!(is_safe_name(&String::from("nul is nullish.txt"), true, true));

    // trailing dot 검사
    assert!(is_safe_name(&String::from("hello.txt."), true, true));
    assert!(!is_safe_name(&String::from("hello.txt."), false, true));

    // trailing space 검사
    assert!(is_safe_name(&String::from("hello.txt "), true, true));
    assert!(!is_safe_name(&String::from("hello.txt "), false, true));
    
    // following space 검사
    assert!(is_safe_name(&String::from(" hello.txt"), true, true));
    assert!(is_safe_name(&String::from(" hello.txt"), true, false));
    assert!(!is_safe_name(&String::from(" hello.txt"), false, true));

    // utf-8 검사
    assert!(is_safe_name(&String::from("한글테스트.txt"), false, true));
}

#[test]
fn test_to_safe_name() {
    // 제거할 문자 테스트
    assert_eq!("hel＂l？dk＊o／／.t⧵xt", to_safe_name(
        &"hel\"l?dk*o//.t\\xt".to_string(),
        &ReplaceMethod::Fullwidth(ReplaceChar::Charactor('!')),
        &DotHandlingPolicy::Replace(ReplaceChar::Charactor('!'))
    ));

    // 스페이스 제거 테스트
    assert_eq!("hello.txt", to_safe_name(
        &"    hello.txt      ".to_string(),
        &ReplaceMethod::Fullwidth(ReplaceChar::Charactor('!')),
        &DotHandlingPolicy::Replace(ReplaceChar::Charactor('!'))
    ));

    // 마침표 제거 테스트
    assert_eq!("hello.txt....!", to_safe_name(
        &"hello.txt.....".to_string(),
        &ReplaceMethod::Fullwidth(ReplaceChar::Charactor('!')),
        &DotHandlingPolicy::Replace(ReplaceChar::Charactor('!'))
    ));
    assert_eq!("hello.txt.....", to_safe_name(
        &"hello.txt.....".to_string(),
        &ReplaceMethod::Fullwidth(ReplaceChar::Charactor('!')),
        &DotHandlingPolicy::NotCorrect)
    );
    assert_eq!("hello.txt", to_safe_name(
        &"hello.txt.....".to_string(),
        &ReplaceMethod::Fullwidth(ReplaceChar::Charactor('!')),
        &DotHandlingPolicy::Remove)
    );

    // 빈 문자열 테스트
    assert_eq!("!", to_safe_name(
        &".....".to_string(),
        &ReplaceMethod::Fullwidth(ReplaceChar::Charactor('!')),
        &DotHandlingPolicy::Remove)
    );
    assert_eq!("_", to_safe_name(
        &".....".to_string(),
        &ReplaceMethod::Fullwidth(ReplaceChar::Charactor('.')),
        &DotHandlingPolicy::Remove)
    );
    
    // 예약어 체크
    assert_eq!("!com2.hello.txt!", to_safe_name(
        &"com2.hello.txt.".to_string(),
        &ReplaceMethod::Fullwidth(ReplaceChar::Charactor('!')),
        &DotHandlingPolicy::Replace(ReplaceChar::Charactor('!')))
    );

    // DotHandlingPolicy::ReplaceWithReplaceMethod 검사
    assert_eq!("hel!lo.txt....!", to_safe_name(
        &"hel*lo.txt.....".to_string(),
        &ReplaceMethod::Replace(ReplaceChar::Charactor('!')),
        &DotHandlingPolicy::ReplaceWithReplaceMethod)
    );
    assert_eq!("hel lo.txt", to_safe_name(
        &"hel*lo.txt.....".to_string(),
        &ReplaceMethod::Replace(ReplaceChar::Space),
        &DotHandlingPolicy::ReplaceWithReplaceMethod)
    );
    assert_eq!("hel⁇lo.txt....⁇", to_safe_name(
        &"hel*lo.txt.....".to_string(),
        &ReplaceMethod::Replace(ReplaceChar::DubleQuestionMark),
        &DotHandlingPolicy::ReplaceWithReplaceMethod)
    );
    assert_eq!("hel❓lo.txt....❓", to_safe_name(
        &"hel*lo.txt.....".to_string(),
        &ReplaceMethod::Replace(ReplaceChar::RedQuestionMark),
        &DotHandlingPolicy::ReplaceWithReplaceMethod)
    );
}

#[test]
#[should_panic = "Cannot replace to space. Use DotHandlingPolicy::Remove instead."]
fn test_to_safe_name_panic() {
    // Should panic
    to_safe_name(
        &"hello.txt.....".to_string(),
        &ReplaceMethod::Fullwidth(ReplaceChar::Charactor('!')),
        &DotHandlingPolicy::Replace(ReplaceChar::Space)
    );
}

#[test]
fn test_simply_to_safe_name() {
    assert_eq!("hel＊lo.txt....．", simply_to_safe_name("hel*lo.txt.....", true));
    assert_eq!("hel_lo.txt...._", simply_to_safe_name("hel*lo.txt.....", false));
}