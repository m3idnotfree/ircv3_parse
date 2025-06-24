use proptest::{
    collection,
    prelude::{Just, Strategy},
    prop_compose, prop_oneof,
};

prop_compose! {
    pub fn rfc1123_strategy()(
        key in prop_oneof![
            rfc1123_label_strategy(),
            collection::vec(rfc1123_label_strategy(), 2..=10)
                .prop_map(|labels|fold_labels(labels, 253))
    ]) -> String { key }
}

prop_compose! {
    fn rfc1123_label_strategy()(
        first in "[a-zA-Z0-9]",
        middle in prop_oneof![
            Just("".to_string()),
            "[a-zA-Z0-9-]{0,61}[a-zA-Z0-9]".prop_map(|s| s),
            "[a-zA-Z0-9-]{61}[a-zA-Z0-9]".prop_map(|s| s),
        ]
    ) -> String {
        if middle.is_empty() {
            first
        } else {
            format!("{}{}", first, middle)
        }
    }
}

fn fold_labels(labels: Vec<String>, max_total_len: usize) -> String {
    labels
        .into_iter()
        .scan(0, |total_len, label| {
            let needed_len = if *total_len == 0 {
                label.len()
            } else {
                *total_len + 1 + label.len()
            };

            if needed_len <= max_total_len {
                *total_len = needed_len;
                Some(label)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(".")
}
