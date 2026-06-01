pub mod ast;

use ast::syntax_tree_types::{Attribute, Document, Node, Value};

fn main() {
    let doc = Document {
        root: Node {
            name: "html".into(),
            attrs: vec![],
            content: None,
            children: vec![
                Node {
                    name: "head".into(),
                    attrs: vec![],
                    content: None,
                    children: vec![Node {
                        name: "title".into(),
                        attrs: vec![],
                        content: Some("My page".into()),
                        children: vec![],
                    }],
                },
                Node {
                    name: "body".into(),
                    attrs: vec![],
                    content: None,
                    children: vec![
                        Node {
                            name: "h1".into(),
                            attrs: vec![],
                            content: Some("Hello".into()),
                            children: vec![],
                        },
                        Node {
                            name: "div".into(),
                            attrs: vec![Attribute {
                                key: "class".into(),
                                value: Value::String("container".into()),
                            }],
                            content: None,
                            children: vec![],
                        },
                        Node {
                            name: "button".into(),
                            attrs: vec![
                                Attribute {
                                    key: "class".into(),
                                    value: Value::String("primary".into()),
                                },
                                Attribute {
                                    key: "disabled".into(),
                                    value: Value::Boolean(true),
                                },
                            ],
                            content: Some("Click me".into()),
                            children: vec![],
                        },
                    ],
                },
            ],
        },
    };

    println!("=== TSF Tree ===\n");
    print!("{doc}");
    println!("\n=== Debug ===\n");
    println!("{doc:#?}");
}
