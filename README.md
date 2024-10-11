# IRCv3 parse

implement [IRCv3 message](https://ircv3.net/specs/extensions/message-tags.html)  
[RFC 1459, section 2.3.1](https://datatracker.ietf.org/doc/html/rfc1459#section-2.3.1)

#### default params middle parse

channel = "foo = #bar", "foo #bar", "#bar"

# Usage

```rust
use ircv3_parse::IRCv3;

fn main(){
  let msg = ":foo!foo@foo.tmi.abcdef.gh PRIVMSG #bar :LLLLLl";
  let ircv3_message = IRCv3::parse(msg);
}
```
