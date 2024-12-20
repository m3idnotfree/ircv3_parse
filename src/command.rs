use nom::{
    branch::alt,
    character::complete::{alpha1, digit1},
    IResult,
};

pub fn command_parse(msg: &str) -> IResult<&str, &str> {
    alt((alpha1, digit1))(msg)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::command::command_parse;

    #[test]
    fn command_privmsg() {
        let msg = "PRIVMSG #bar :bleedPurple";
        let (remain, command) = command_parse(msg).unwrap();

        assert_eq!("PRIVMSG", command);
        assert_eq!(" #bar :bleedPurple", remain);
    }

    #[test]
    fn command_digit() {
        let msg = "857 #bar :bleedPurple";
        let (remain, command) = command_parse(msg).unwrap();

        assert_eq!("857", command);
        assert_eq!(" #bar :bleedPurple", remain);
    }
}
