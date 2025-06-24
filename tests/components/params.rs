use std::fmt;

use proptest::{
    prelude::{prop, Just, Strategy},
    prop_compose, prop_oneof,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ParamsType {
    MiddlesOnly(Vec<String>),
    TrailingOnly(String),
    MiddlesTrailing(Vec<String>, String),
    None,
}

impl ParamsType {
    pub fn raw(&self) -> String {
        match self {
            Self::MiddlesOnly(middles) => {
                if middles.is_empty() {
                    String::new()
                } else {
                    middles.join(" ")
                }
            }
            Self::TrailingOnly(trailing) => trailing.to_string(),
            Self::MiddlesTrailing(middles, trailing) => {
                if middles.is_empty() {
                    trailing.to_string()
                } else {
                    format!("{} :{}", middles.join(" "), trailing)
                }
            }
            Self::None => String::new(),
        }
    }
}

impl fmt::Display for ParamsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MiddlesOnly(middles) => {
                if middles.is_empty() {
                    write!(f, "{}", String::new())
                } else {
                    write!(f, " {}", middles.join(" "))
                }
            }
            Self::TrailingOnly(trailing) => write!(f, " :{}", trailing),
            Self::MiddlesTrailing(middles, trailing) => {
                if middles.is_empty() {
                    write!(f, " :{}", trailing)
                } else {
                    write!(f, " {} :{}", middles.join(" "), trailing)
                }
            }
            Self::None => write!(f, "{}", String::new()),
        }
    }
}

prop_compose! {
    pub fn params_strategy()(
        params in prop_oneof![
            middles().prop_map(ParamsType::MiddlesOnly),
            (middles(), trailing()).prop_map(|(m, t)| ParamsType::MiddlesTrailing(m, t)),
            trailing().prop_map(ParamsType::TrailingOnly),
            Just(ParamsType::None),

        ]
    ) -> ParamsType { params }
}

prop_compose! {
    pub fn middles()(
        middles in prop::collection::vec("[a-zA-Z0-9#&+!.-]+", 0..=14)
    )-> Vec<String> { middles }
}

prop_compose! {
    pub fn trailing()(
        trailing in "[^\r\n\0]{0,500}"
    ) -> String { trailing }
}
