use std::rc::Rc;

use parser::parse;
use parser::{Document, Node};
use tokenizer::Pos;

const MULTI_SEQ: &str = "---
- 1
---
- 2
";

const MULTI_MAP: &str = "---
a: 1
---
b: 2
";

#[test]
fn test_parse_multi_documents() {
    let pos = Pos {
        filename: Rc::new("<text>".to_string()),
        indent: 0,
        line: 0,
        line_start: false,
        line_offset: 0,
        offset: 0,
    };
    assert_eq!(
        parse(
            Rc::new("<text>".to_string()),
            MULTI_MAP,
            |d| "ok".to_string()
        ).unwrap(),
        "ok".to_string(),
    );
}
