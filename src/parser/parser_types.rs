use std::io::BufRead;
use crate::ast::syntax_tree_types::{Attribute, Document, Node, Value};
use crate::errors::error_types::ParseError;
use crate::lexer::lexer_types::{Lexer, Token};

pub struct Parser<R: BufRead> {
    lex: Lexer<R>,
    lookahead: Option<Token>,
}

impl<R: BufRead> Parser<R> {
    pub fn new(reader: R) -> Self {
        let mut lex = Lexer::new(reader);
        let lookahead = Some(lex.scan());
        Parser { lex, lookahead }
    }

    fn peek(&self) -> Option<&Token> {
        self.lookahead.as_ref()
    }

    fn advance(&mut self) -> Token {
        let tok = self.lookahead.take().unwrap_or(Token::Eof);
        self.lookahead = Some(self.lex.scan());
        tok
    }

    fn expect_depth(&mut self) -> Result<String, ParseError> {
        match self.peek() {
            Some(Token::Depth(_)) => {
                let tok = self.advance();
                match tok {
                    Token::Depth(s) => Ok(s),
                    _ => unreachable!(),
                }
            }
            Some(other) => Err(ParseError::UnexpectedToken {
                expected: "depth value".to_string(),
                found: other.clone(),
            }),
            None => Err(ParseError::UnexpectedToken {
                expected: "depth value".to_string(),
                found: Token::Eof,
            }),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        match self.peek() {
            Some(Token::Identifier(_)) => {
                let tok = self.advance();
                match tok {
                    Token::Identifier(s) => Ok(s),
                    _ => unreachable!(),
                }
            }
            Some(other) => Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: other.clone(),
            }),
            None => Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: Token::Eof,
            }),
        }
    }

    fn expect_assign(&mut self) -> Result<(), ParseError> {
        match self.peek() {
            Some(Token::Assign) => {
                self.advance();
                Ok(())
            }
            Some(other) => Err(ParseError::UnexpectedToken {
                expected: "=".to_string(),
                found: other.clone(),
            }),
            None => Err(ParseError::UnexpectedToken {
                expected: "=".to_string(),
                found: Token::Eof,
            }),
        }
    }

    fn parse_value(&mut self) -> Result<Value, ParseError> {
        let tok = self.advance();
        match tok {
            Token::Text(s) => Ok(Value::String(s)),
            Token::Identifier(s) => {
                if let Ok(n) = s.parse::<i64>() {
                    return Ok(Value::Integer(n));
                }
                if let Ok(f) = s.parse::<f64>() {
                    return Ok(Value::Float(f));
                }
                match s.as_str() {
                    "true" => Ok(Value::Boolean(true)),
                    "false" => Ok(Value::Boolean(false)),
                    "null" => Ok(Value::Null),
                    _ => Err(ParseError::InvalidValue { raw: s }),
                }
            }
            other => Err(ParseError::UnexpectedToken {
                expected: "attribute value (string, number, boolean, or null)".to_string(),
                found: other,
            }),
        }
    }

    fn parse_line(&mut self) -> Result<(u32, Node), ParseError> {
        let depth_str = self.expect_depth()?;
        let depth: u32 = depth_str.parse().expect("depth should be valid digits");
        let name = self.expect_identifier()?;

        let mut attrs = Vec::new();
        let mut content = None;
        let mut content_seen = false;

        loop {
            match self.peek() {
                Some(Token::NewLine) => {
                    self.advance();
                    break;
                }
                Some(Token::Eof) | None => break,
                Some(Token::Identifier(_)) if !content_seen => {
                    let key = self.expect_identifier()?;
                    self.expect_assign()?;
                    let value = self.parse_value()?;
                    attrs.push(Attribute { key, value });
                }
                Some(Token::Text(_)) if !content_seen => {
                    let tok = self.advance();
                    if let Token::Text(s) = tok {
                        content = Some(s);
                        content_seen = true;
                    }
                }
                Some(other) => {
                    let msg = if content_seen {
                        "newline".to_string()
                    } else {
                        "attribute, content, or newline".to_string()
                    };
                    return Err(ParseError::UnexpectedToken {
                        expected: msg,
                        found: other.clone(),
                    });
                }
            }
        }

        Ok((
            depth,
            Node {
                name,
                attrs,
                content,
                children: Vec::new(),
            },
        ))
    }

    pub fn parse(&mut self) -> Result<Document, ParseError> {
        let mut stack: Vec<Node> = Vec::new();

        loop {
            match self.peek() {
                Some(Token::Eof) | None => break,
                _ => {}
            }

            let (depth, node) = self.parse_line()?;

            if stack.is_empty() && depth != 0 {
                return Err(ParseError::DepthJump {
                    current_depth: 0,
                    target_depth: depth,
                });
            }

            if depth == 0 && !stack.is_empty() {
                return Err(ParseError::UnexpectedToken {
                    expected: "depth > 0 (root already defined)".to_string(),
                    found: Token::Depth("0".to_string()),
                });
            }

            if depth > stack.len() as u32 {
                return Err(ParseError::DepthJump {
                    current_depth: stack.len() as u32,
                    target_depth: depth,
                });
            }

            while (stack.len() as u32) > depth {
                if let Some(child) = stack.pop() {
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(child);
                    }
                }
            }

            stack.push(node);
        }

        while stack.len() > 1 {
            if let Some(child) = stack.pop() {
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(child);
                }
            }
        }

        if stack.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        Ok(Document {
            root: stack.into_iter().next().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn parse_input(input: &str) -> Result<Document, ParseError> {
        let reader = BufReader::new(input.as_bytes());
        let mut parser = Parser::new(reader);
        parser.parse()
    }

    #[test]
    fn parse_root_only() {
        let doc = parse_input("0 html\n").unwrap();
        assert_eq!(doc.root.name, "html");
        assert!(doc.root.attrs.is_empty());
        assert!(doc.root.content.is_none());
        assert!(doc.root.children.is_empty());
    }

    #[test]
    fn parse_nested_tree() {
        let input = "\
0 html
1 head
2 title \"My page\"
1 body
2 h1 \"Hello\"
2 div class=\"container\"
";
        let doc = parse_input(input).unwrap();
        assert_eq!(doc.root.name, "html");
        assert_eq!(doc.root.children.len(), 2);

        let head = &doc.root.children[0];
        assert_eq!(head.name, "head");
        assert_eq!(head.children.len(), 1);
        assert_eq!(head.children[0].name, "title");
        assert_eq!(head.children[0].content.as_deref(), Some("My page"));

        let body = &doc.root.children[1];
        assert_eq!(body.name, "body");
        assert_eq!(body.children.len(), 2);
        assert_eq!(body.children[0].name, "h1");
        assert_eq!(body.children[0].content.as_deref(), Some("Hello"));
        assert_eq!(body.children[1].name, "div");
        assert_eq!(body.children[1].attrs.len(), 1);
        assert_eq!(body.children[1].attrs[0].key, "class");
        assert_eq!(body.children[1].attrs[0].value, Value::String("container".into()));
    }

    #[test]
    fn parse_attributes() {
        let input = "0 div class=\"main\" id=\"content\"\n";
        let doc = parse_input(input).unwrap();
        assert_eq!(doc.root.name, "div");
        assert_eq!(doc.root.attrs.len(), 2);
        assert_eq!(doc.root.attrs[0].key, "class");
        assert_eq!(doc.root.attrs[0].value, Value::String("main".into()));
        assert_eq!(doc.root.attrs[1].key, "id");
        assert_eq!(doc.root.attrs[1].value, Value::String("content".into()));
    }

    #[test]
    fn parse_attributes_unquoted() {
        let input = "0 button disabled=true count=42 ratio=3.14\n";
        let doc = parse_input(input).unwrap();
        assert_eq!(doc.root.name, "button");
        assert_eq!(doc.root.attrs.len(), 3);
        assert_eq!(doc.root.attrs[0].value, Value::Boolean(true));
        assert_eq!(doc.root.attrs[1].value, Value::Integer(42));
        assert_eq!(doc.root.attrs[2].value, Value::Float(3.14));
    }

    #[test]
    fn parse_content() {
        let input = "0 title \"My Page\"\n";
        let doc = parse_input(input).unwrap();
        assert_eq!(doc.root.name, "title");
        assert_eq!(doc.root.content.as_deref(), Some("My Page"));
    }

    #[test]
    fn parse_attrs_and_content() {
        let input = "0 div class=\"main\" \"hello\"\n";
        let doc = parse_input(input).unwrap();
        assert_eq!(doc.root.name, "div");
        assert_eq!(doc.root.attrs.len(), 1);
        assert_eq!(doc.root.attrs[0].value, Value::String("main".into()));
        assert_eq!(doc.root.content.as_deref(), Some("hello"));
    }

    #[test]
    fn parse_negative_values() {
        let input = "0 m temp=-42 offset=-3.14\n";
        let doc = parse_input(input).unwrap();
        assert_eq!(doc.root.attrs[0].value, Value::Integer(-42));
        assert_eq!(doc.root.attrs[1].value, Value::Float(-3.14));
    }

    #[test]
    fn parse_null_value() {
        let input = "0 m value=null\n";
        let doc = parse_input(input).unwrap();
        assert_eq!(doc.root.attrs[0].value, Value::Null);
    }

    #[test]
    fn roundtrip_forward_parse_serialize() {
        let input = "\
0 html
1 head
2 title \"My page\"
1 body
2 h1 \"Hello\"
2 div class=\"container\"
";
        println!("=== Forward roundtrip: parse → serialize ===");
        println!("Input TSF:");
        println!("{input}");

        let doc = parse_input(input).unwrap();
        println!("Parsed AST: {doc:#?}");

        let output = doc.to_string();
        println!("Serialized output:");
        println!("{output}");
        println!("Input == Output: {}", input == output);

        assert_eq!(output, input);
    }

    #[test]
    fn roundtrip_reverse_serialize_parse() {
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
                        ],
                    },
                ],
            },
        };
        println!("=== Reverse roundtrip: serialize → parse ===");
        println!("Original AST (built programmatically):");
        println!("{doc:#?}");

        let serialized = doc.to_string();
        println!("Serialized TSF:");
        println!("{serialized}");

        let parsed = parse_input(&serialized).unwrap();
        println!("Parsed back to AST:");
        println!("{parsed:#?}");
        println!("Original == Parsed: {}", parsed == doc);

        assert_eq!(parsed, doc);
    }

    #[test]
    fn parse_error_empty() {
        let err = parse_input("").unwrap_err();
        assert!(matches!(err, ParseError::EmptyInput));
    }

    #[test]
    fn parse_error_depth_nonzero_root() {
        let err = parse_input("2 root\n").unwrap_err();
        assert!(matches!(err, ParseError::DepthJump { current_depth: 0, target_depth: 2 }));
    }

    #[test]
    fn parse_error_depth_jump() {
        let err = parse_input("0 root\n2 child\n").unwrap_err();
        assert!(matches!(err, ParseError::DepthJump { .. }));
    }

    #[test]
    fn parse_error_invalid_value() {
        let err = parse_input("0 n key=baz\n").unwrap_err();
        assert!(matches!(err, ParseError::InvalidValue { .. }));
    }

    #[test]
    fn parse_error_unexpected_token() {
        let err = parse_input("0 =bad\n").unwrap_err();
        assert!(matches!(err, ParseError::UnexpectedToken { .. }));
    }

    #[test]
    fn parse_error_multiple_roots() {
        let err = parse_input("0 a\n0 b\n").unwrap_err();
        assert!(matches!(err, ParseError::UnexpectedToken { .. }));
    }

    #[test]
    fn parse_error_content_then_attr() {
        let err = parse_input("0 div \"text\" class=\"x\"\n").unwrap_err();
        assert!(matches!(err, ParseError::UnexpectedToken { .. }));
    }
}
