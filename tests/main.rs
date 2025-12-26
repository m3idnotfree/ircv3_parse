mod components;

#[cfg(feature = "serde")]
mod serde;

use components::{
    commands::invalid_command_strategy, escape::escaped_strategy, params::ParamsType, TestMessage,
};
use ircv3_parse::{unescape, validators};
use proptest::{prelude::any, prop_assert, prop_assert_eq, proptest, test_runner::Config};

proptest! {
    #![proptest_config(Config::with_cases(3000))]

    #[test]
    fn base(
        test_message in any::<TestMessage>()
    ){
        let msg = test_message.to_string();
        let TestMessage {tags, source, command, params, line_ending} = test_message;
        let result = ircv3_parse::parse(&msg).unwrap();


        if let Some(tags) = tags {
            let actual_tags = result.tags();

            prop_assert!(actual_tags.is_some());
            let actual_tags = actual_tags.unwrap();

            prop_assert!(actual_tags.validate().is_ok());
            prop_assert_eq!(tags.to_string(), actual_tags.to_string());
            prop_assert_eq!(tags.count(), actual_tags.count());


            for tag in tags.iter(){
                let expected_key = tag.expected_key();
                prop_assert!(validators::tag_key(&expected_key).is_ok());

                let actual_value = actual_tags.get(&expected_key);
                if let Some(value) = actual_value {
                    prop_assert!(validators::tag_value(value.as_str()).is_ok());
                }

                prop_assert_eq!(tag.expected_value(), actual_tags.get(&expected_key).map(|x|x.to_string()));
            }
        }

        if let Some(expected_source) = source {
            let actual_source = result.source();

            prop_assert!(actual_source.is_some());
            let actual_source = result.source().unwrap();

            prop_assert!(actual_source.validate().is_ok());
            prop_assert_eq!(expected_source.to_string(), actual_source.to_string());
            prop_assert_eq!(expected_source.expected_name(), actual_source.name);
            prop_assert_eq!(expected_source.expected_user(), actual_source.user);
            prop_assert_eq!(expected_source.expected_host(), actual_source.host);
        }

        let cmd = result.command();
        prop_assert!(cmd.validate().is_ok());
        prop_assert_eq!(command, cmd.as_str());

        let actual_params = result.params();
        prop_assert_eq!(params.raw(), actual_params.to_string());

        match params {
            ParamsType::MiddlesOnly(middles) => {
                prop_assert!(actual_params.middles.validate().is_ok());
                prop_assert_eq!(middles.len(), actual_params.middles.count());
                prop_assert_eq!(middles, actual_params.middles.to_vec());

                prop_assert!(actual_params.trailing.is_none());
            },
            ParamsType::MiddlesTrailing(middles, trailing) => {
                prop_assert!(actual_params.middles.validate().is_ok());
                prop_assert_eq!(middles.len(), actual_params.middles.count());
                prop_assert_eq!(middles, actual_params.middles.to_vec());

                prop_assert!(actual_params.trailing.is_some());
                prop_assert_eq!(trailing, actual_params.trailing.to_string());
            },
            ParamsType::TrailingOnly(trailing) => {
                prop_assert_eq!(0, actual_params.middles.count());

                prop_assert!(actual_params.trailing.is_some());
                prop_assert_eq!(trailing, actual_params.trailing.to_string());

            },
            ParamsType::None => {
                prop_assert_eq!(0, actual_params.middles.count());
                prop_assert!(actual_params.trailing.is_none());
            },
        };

        let round = format!("{}{}{}{}{}",
            if let Some(t)=result.tags(){
               format!("@{} ",t)
            }else {
                "".to_string()
            },
            if let Some(s) = result.source() {
                format!(":{} ",s)
            } else {
                "".to_string()
            },
            result.command(),
            result.params().message(),
            line_ending
            );
        prop_assert_eq!(msg, round);
    }
}

#[test]
fn test_basic_unescaping() {
    assert_eq!(unescape("hello\\sworld"), "hello world");
    assert_eq!(unescape("semi\\:colon"), "semi;colon");
    assert_eq!(unescape("back\\\\slash"), "back\\slash");
    assert_eq!(unescape("carriage\\rreturn"), "carriage\rreturn");
    assert_eq!(unescape("line\\nfeed"), "line\nfeed");
}

#[test]
fn test_edge_cases() {
    assert_eq!(unescape(""), "");
    assert_eq!(unescape("\\"), "\\");
    assert_eq!(unescape("no_escapes"), "no_escapes");
    assert_eq!(unescape("\\unknown"), "\\unknown");
    assert_eq!(unescape("multiple\\s\\:escapes"), "multiple ;escapes");
}

proptest! {
    #![proptest_config(Config::with_cases(3000))]
    #[test]
    fn escaped_string(
        (input, expected) in escaped_strategy()
    ) {
        prop_assert_eq!(expected, unescape(&input));
    }
}

proptest! {
    #![proptest_config(Config::with_cases(3000))]
    #[test]
    fn invalid_command(
        cmd in invalid_command_strategy()
    ) {
        prop_assert!(validators::command(&cmd).is_err());
    }
}
