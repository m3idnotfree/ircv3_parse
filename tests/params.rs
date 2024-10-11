use ircv3_parse::params_parse;
use pretty_assertions::assert_eq;

#[test]
fn base_params_only_channel() {
    let msg = " #<channel>";
    let (remain, mut params) = params_parse(msg).unwrap();

    assert_eq!(remain, "");
    assert_eq!(1, params.middle.len());
    assert_eq!(None, params.message);
    assert_eq!(Some("#<channel>".to_string()), params.middle.pop());
}

#[test]
fn base_params_only_channel_rn() {
    let msg = " #<channel>\r\n";
    let (remain, mut params) = params_parse(msg).unwrap();

    assert_eq!(remain, "");
    assert_eq!(Some("#<channel>".to_string()), params.middle.pop());
}

#[test]
fn base_params_middle() {
    let msg = " bar = #twitchdev :bar";
    let (remain, mut params) = params_parse(msg).unwrap();

    assert_eq!(Some("bar".to_string()), params.message);
    assert_eq!(3, params.middle.len());
    assert_eq!(Some("#twitchdev".to_string()), params.middle.pop());
    assert_eq!(Some("=".to_string()), params.middle.pop());
    assert_eq!(Some("bar".to_string()), params.middle.pop());
    assert_eq!(remain, "");
}

#[test]
fn base_params_channel_message() {
    let msg = " #barbar :This room is already in unique-chat mode.";
    let (remain, mut params) = params_parse(msg).unwrap();

    assert_eq!(Some("#barbar".to_string()), params.middle.pop());
    assert_eq!(
        Some("This room is already in unique-chat mode.".to_string()),
        params.message
    );
    assert_eq!(remain, "");
}

#[test]
fn base_paramsspace_empty() {
    let msg = " ";
    let (remain, params) = params_parse(msg).unwrap();

    assert_eq!(1, params.middle.len());
    assert_eq!(None, params.message);
    assert_eq!(remain, "");
}

#[test]
fn base_params_space_empty_rn() {
    let msg = " \r\n";
    let (remain, params) = params_parse(msg).unwrap();

    assert_eq!(1, params.middle.len());
    assert_eq!(None, params.message);
    assert_eq!(remain, "");
}

#[test]
fn base_paramse_empty() {
    let msg = "";
    let (remain, params) = params_parse(msg).unwrap();

    assert_eq!(0, params.middle.len());
    assert_eq!("", remain);
}

#[test]
fn base_params_empty_rn() {
    let msg = "\r\n";
    let (remain, params) = params_parse(msg).unwrap();

    assert_eq!(0, params.middle.len());
    assert_eq!("", remain);
}

#[test]
fn base_params_channel_message_base() {
    let msg = " #ronni :Kappa Keepo Kappa";
    let (remain, mut params) = params_parse(msg).unwrap();

    assert_eq!(Some("#ronni".to_string()), params.middle.pop(),);
    assert_eq!(Some("Kappa Keepo Kappa".to_string()), params.message,);
    assert_eq!(remain, "");
}
#[test]
fn base_params_channel_message_rn() {
    let msg = " #ronni :Kappa Keepo Kappa\r\n";
    let (remain, mut params) = params_parse(msg).unwrap();

    assert_eq!(Some("#ronni".to_string()), params.middle.pop());
    assert_eq!(Some("Kappa Keepo Kappa".to_string()), params.message,);
    assert_eq!(remain, "");
}
