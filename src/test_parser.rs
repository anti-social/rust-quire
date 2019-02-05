use std::rc::Rc;

use parser::{parse, parse_all};
use parser::{Document, Node};
use tokenizer::Pos;

const MULTI_SEQ: &'static str = "---
- 1

---
- 2
- 3
";

const MULTI_MAP: &'static str = "---
a: 1
---
a: 2
b: 3
";

#[test]
fn test_parse_multi_documents_seq() {
    let mut doc_num = 0;
    let res = parse_all(
        Rc::new("<text>".to_string()),
        MULTI_SEQ,
        move |d| {
            match doc_num {
                0 => {
                    assert_eq!(
                        format!("{:?}", &d),
                        "Document { directives: [], root: <Sequence [<Scalar 1>]> }"
                    );
                    ()
                },
                1 => {
                    assert_eq!(
                        format!("{:?}", &d),
                        "Document { directives: [], root: <Sequence [<Scalar 2>, <Scalar 3>]> }"
                    );
                    ()
                },
                _ => panic!("Too many documents"),
            }
            doc_num += 1;
            ()
        }
    ).unwrap();
    assert_eq!(res, vec![(), ()]);
}

#[test]
fn test_parse_multi_documents_map() {
    let mut doc_num = 0;
    let res = parse_all(
        Rc::new("<text>".to_string()),
        MULTI_MAP,
        move |d| {
            match doc_num {
                0 => {
                    assert_eq!(
                        format!("{:?}", &d),
                        "Document { directives: [], root: <Map {<Scalar a>: <Scalar 1>}> }"
                    );
                    ()
                },
                1 => {
                    assert_eq!(
                        format!("{:?}", &d),
                        "Document { directives: [], root: <Map {<Scalar a>: <Scalar 2>, <Scalar b>: <Scalar 3>}> }"
                    );
                    ()
                },
                _ => panic!("Too many documents"),
            }
            doc_num += 1;
            ()
        }
    ).unwrap();
    assert_eq!(res, vec![(), ()]);
}
