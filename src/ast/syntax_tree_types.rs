use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => {
                let escaped: String = s
                    .chars()
                    .flat_map(|c| match c {
                        '"' => "\\\"".chars().collect(),
                        '\\' => "\\\\".chars().collect(),
                        '\n' => "\\n".chars().collect(),
                        '\t' => "\\t".chars().collect(),
                        _ => vec![c],
                    })
                    .collect();
                write!(f, "\"{escaped}\"")
            }
            Value::Integer(n) => write!(f, "{n}"),
            Value::Float(x) => write!(f, "{x}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub key: String,
    pub value: Value,
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub content: Option<String>,
    pub children: Vec<Node>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_tree(f, self, 0)
    }
}

fn write_tree(f: &mut fmt::Formatter<'_>, node: &Node, depth: usize) -> fmt::Result {
    write!(f, "{} ", depth)?;
    write!(f, "{}", node.name)?;

    for attr in &node.attrs {
        write!(f, " {attr}")?;
    }

    if let Some(content) = &node.content {
        let escaped: String = content
            .chars()
            .flat_map(|c| match c {
                '"' => "\\\"".chars().collect(),
                '\\' => "\\\\".chars().collect(),
                '\n' => "\\n".chars().collect(),
                '\t' => "\\t".chars().collect(),
                _ => vec![c],
            })
            .collect();
        write!(f, " \"{escaped}\"")?;
    }

    writeln!(f)?;

    for child in &node.children {
        write_tree(f, child, depth + 1)?;
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub root: Node,
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_value_string() {
        assert_eq!(Value::String("hello".into()).to_string(), "\"hello\"");
    }

    #[test]
    fn display_value_string_escapes() {
        assert_eq!(
            Value::String("hello\n\"world\"".into()).to_string(),
            "\"hello\\n\\\"world\\\"\""
        );
    }

    #[test]
    fn display_value_integer() {
        assert_eq!(Value::Integer(42).to_string(), "42");
    }

    #[test]
    fn display_value_float() {
        assert_eq!(Value::Float(3.14).to_string(), "3.14");
    }

    #[test]
    fn display_value_boolean() {
        assert_eq!(Value::Boolean(true).to_string(), "true");
        assert_eq!(Value::Boolean(false).to_string(), "false");
    }

    #[test]
    fn display_value_null() {
        assert_eq!(Value::Null.to_string(), "null");
    }

    #[test]
    fn display_attribute() {
        let attr = Attribute {
            key: "class".into(),
            value: Value::String("container".into()),
        };
        assert_eq!(attr.to_string(), "class=\"container\"");
    }

    #[test]
    fn display_node_simple() {
        let node = Node {
            name: "html".into(),
            attrs: vec![],
            content: None,
            children: vec![],
        };
        assert_eq!(node.to_string(), "0 html\n");
    }

    #[test]
    fn display_node_with_attrs_and_content() {
        let node = Node {
            name: "div".into(),
            attrs: vec![Attribute {
                key: "class".into(),
                value: Value::String("main".into()),
            }],
            content: Some("hello".into()),
            children: vec![],
        };
        assert_eq!(node.to_string(), "0 div class=\"main\" \"hello\"\n");
    }

    #[test]
    fn display_nested_tree() {
        let tree = Document {
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
                        ],
                    },
                ],
            },
        };
        let expected = "\
0 html
1 head
2 title \"My page\"
1 body
2 h1 \"Hello\"
2 div class=\"container\"
";
        assert_eq!(tree.to_string(), expected);
    }
}
