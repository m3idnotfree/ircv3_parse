use proptest::{
    char, collection,
    prelude::{any, Arbitrary, BoxedStrategy, Just, Strategy},
    prop_compose, prop_oneof,
};

#[derive(Debug, Clone, PartialEq)]
pub enum EscapeString {
    Normal(String),
    UnknownEscaped(String),
    Colon,
    Space,
    Slash,
    Return,
    NewLine,
}
impl EscapeString {
    pub fn escaped_string(&self) -> String {
        match self {
            Self::Normal(s) => s.to_string(),
            Self::UnknownEscaped(s) => s.to_string(),
            Self::Colon => ";".to_string(),
            Self::Space => " ".to_string(),
            Self::Slash => "\\".to_string(),
            Self::Return => "\r".to_string(),
            Self::NewLine => "\n".to_string(),
        }
    }
    pub fn unescaped_string(&self) -> String {
        match self {
            Self::Normal(s) => s.to_string(),
            Self::UnknownEscaped(s) => s.to_string(),
            Self::Colon => "\\:".to_string(),
            Self::Space => "\\s".to_string(),
            Self::Slash => "\\\\".to_string(),
            Self::Return => "\r".to_string(),
            Self::NewLine => "\n".to_string(),
        }
    }
}

impl Arbitrary for EscapeString {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            normal_strategy().prop_map(EscapeString::Normal),
            unknown_escape_strategy().prop_map(EscapeString::UnknownEscaped),
            Just(EscapeString::Colon),
            Just(EscapeString::Space),
            Just(EscapeString::Slash),
            Just(EscapeString::Return),
            Just(EscapeString::NewLine),
        ]
        .boxed()
    }
}

// input, expected
prop_compose! {
    pub fn escaped_strategy()(
        s in collection::vec(any::<EscapeString>(), 0..=20)
    ) -> (String, String) {
        let expected: String = s
            .iter()
            .map(|s| s.escaped_string())
            .collect();

        let unescaped_input: String = s
            .iter()
            .map(|s| s.unescaped_string())
            .collect();

        (unescaped_input, expected)
    }
}

prop_compose! {
    fn normal_strategy()(
        normal in collection::vec(
        // exclude space
            char::range('\x21', '\x7E')
                .prop_filter("exclude special chars", |&c|{
                    c != ';' && c != '\\' && c != '\r' && c != '\n'
                }),
            0..=100
        )
        .prop_map(|chars| chars.into_iter().collect::<String>())
    ) -> String { normal }
}

prop_compose! {
    fn unknown_escape_strategy()(
        name in prop_oneof![
            char::range('a', 'z')
                .prop_filter("exclude known escapes", |&c| {
                    c != 's' && c != 'r' && c != 'n'
                })
                .prop_map(|c| format!("\\{}", c)),

            char::range('0', '9')
                .prop_map(|c| format!("\\{}", c)),

            char::range('!', '/')
                .prop_filter("exclude known escapes", |&c| {
                    c != ':' && c != '\\'
                })
                .prop_map(|c| format!("\\{}", c)),
    ]) -> String { name }
}
