# rsfilename

Windows와 macOS, Linux에서 안전한 파일명을 만듭니다.

Python 구현을 보고 싶다면 다음 링크를 참고하세요: [pyfilename](https://github.com/ilotoki0804/pyfilename)

```rust
use rsfilename::*;
assert_eq!("hello？.txt.．", simply_to_safe_name("  hello?.txt..", true));
```

자세한 설명은 [문서](https://docs.rs/rsfilename/latest/rsfilename/)를 참고하세요.
