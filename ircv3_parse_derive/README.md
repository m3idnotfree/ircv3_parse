Derive macros for `FromMessage` and `ToMessage` traits from `ircv3_parse`.

---

## Attributes

When no explicit key is specified, the field name is used as the key, converted to `kebab-case`.

`#[irc(command = "CMD")]` is optional for structs and unit structs.

### Struct

```rust,ignore
#[derive(FromMessage, ToMessage)]
#[irc(command = "PRIVMSG")]
struct PrivMsg<'a> {
    #[irc(tag = "id")]
    id: &'a str,
    #[irc(source)]
    nick: &'a str,
    #[irc(param)]
    channel: &'a str,
    #[irc(trailing)]
    text: &'a str,
}
```

The container attribute `#[irc(command = "CMD")]` checks the command on deserialization and writes it on serialization.

#### Tags

##### Tag

```rust,ignore
#[irc(tag)]
#[irc(tag = "key")]
```

##### Tag Flag

```rust,ignore
#[irc(tag_flag)]
#[irc(tag_flag = "key")]
```

Supports `bool`, `Option<T>`, or a nested type.

#### Source

```rust,ignore
#[irc(source)]              // source.name
#[irc(source = "name")]
#[irc(source = "user")]
#[irc(source = "host")]
```

#### Param

```rust,ignore
#[irc(param)]         // first param (index 0)
#[irc(param = 1)]     // second param
#[irc(params)]        // all params
```

`Vec<T>` is only valid with `#[irc(params)]`.

#### Trailing

```rust,ignore
#[irc(trailing)]
```

- `&str`/`String` never fails - yields an empty string when absent.
- `Option<T>` yields `None` when absent or empty (empty string is treated as `None`).

#### Command

```rust,ignore
#[irc(command)]
```

Does not support `Option`.

#### Nested

A field with no `#[irc(...)]` attribute delegates to that type's own `FromMessage`/`ToMessage` implementation.

---

### Unit Struct

Unit structs validate that a message matches expected values without extracting data, and write those values on serialization.

```rust,ignore
#[derive(FromMessage, ToMessage)]
#[irc(command = "PING", tag = "src", value = "server")]
struct PingFromServer;
```

- `#[irc(command = "CMD")]` - matches/writes the command
- `#[irc(tag = "key")]`, `#[irc(param)]`, etc. - matches/writes a specific component value
- `#[irc(value = "...")]` - overrides the expected value (default: kebab-case of the struct name)

---

### Enum

Enums match a single IRC component value against variant patterns.

```rust,ignore
#[derive(FromMessage, ToMessage)]
#[irc(command)]
enum Command {
    PrivMsg,
    Join,
    Part,
}
```

#### Container Attributes

```rust,ignore
#[irc(command)]
#[irc(tag)]
#[irc(tag = "key")]
#[irc(tag_flag)]
#[irc(tag_flag = "key")]
#[irc(source)]
#[irc(source = "name|user|host")]
#[irc(param)]
#[irc(param = N)]
#[irc(trailing)]
```

- `#[irc(rename_all = "lowercase|UPPERCASE|kebab-case")]` - rename all variants (default: `kebab-case`, except `command` which defaults to `UPPERCASE`)

When no key is specified, the enum name is used as the tag key (always `kebab-case`, independent of `rename_all`).

#### Variant Attributes

- `#[irc(value = "...")]` - override the match/serialization value (can appear multiple times. see [`pick`](#enum-variant) for serialization with multiple values)
- `#[irc(present)]` - for `tag_flag` enums, marks the variant matched when the flag is present

```rust,ignore
#[derive(FromMessage, ToMessage)]
#[irc(tag = "badge-info")]
enum Badge {
    #[irc(value = "subscriber", pick)]
    #[irc(value = "sub")]
    Subscriber,
    Moderator,
}

#[derive(FromMessage, ToMessage)]
#[irc(tag_flag = "mod")]
enum Mod {
    #[irc(present)]
    Moderator,
    Regular,
}
```

`tag_flag` enums require exactly two variants:

- one marked `#[irc(present)]` - matched when the flag is present
- one absent variant - matched when the flag is absent. **must be a unit variant**

#### Enum Fields

Variants can carry fields that extract additional IRC components.

```rust,ignore
#[derive(FromMessage, ToMessage)]
#[irc(command)]
enum Command {
    PrivMsg {
        #[irc(param)]
        channel: String,
        #[irc(trailing)]
        text: String,
    },
    Join(#[irc(param)] String),
    Part,
}
```

When a variant has a single unnamed field with no `#[irc(...)]` attribute, the field carries the matched component value directly.

```rust,ignore
#[derive(FromMessage, ToMessage)]
#[irc(tag = "color")]
enum Color {
    Red,
    Blue,
    Other(String),
}
```

---

## Deserialization (FromMessage)

### Field Options

##### with

Provides a custom deserializer function. The function receives the raw component input.

```rust,ignore
#[irc(tag = "ts", with = "parse_timestamp")]
timestamp: u64,

fn parse_timestamp(value: Option<&str>) -> u64 {
    value.and_then(|s| s.parse().ok()).unwrap_or(0)
}
```

##### default

Uses a fallback value when the component is absent instead of returning an error.

```rust,ignore
#[irc(tag = "color", default)]             // Default::default()
#[irc(tag = "color", default = "red_fn")]  // red_fn()
```

### Enum

- `#[irc(default = "VariantName")]` - catch-all unit variant for unrecognized values

---

## Serialization (ToMessage)

### Container

- `#[irc(crlf)]` - appends `\r\n` and validates the message is complete:
  - command must be set (`#[irc(command = "...")]` or `#[irc(command)]` field)
  - `#[irc(source = "user")]` or `#[irc(source = "host")]` requires `#[irc(source)]` to also be present

### Field Options

#### skip

Always skip this field during serialization.

```rust,ignore
#[irc(tag = "internal", skip)]
```

#### skip_none

Skip the tag entirely when the value is `None`. Only allowed on `#[irc(tag)]` fields with `Option<String>` or `Option<&str>` type.

Without `skip_none`, `None` writes `key=` (tag present with no value). With `skip_none`, `None` omits the tag entirely.

```rust,ignore
#[irc(tag = "msgid", skip_none)]
id: Option<String>,    // None -> tag absent, Some(v) -> @msgid=v

#[irc(tag = "msgid")]
id: Option<String>,    // None -> @msgid=
```

### Enum Variant

- `#[irc(pick)]` - when a variant has multiple `#[irc(value)]`, selects which one to use for serialization

```rust,ignore
#[derive(ToMessage)]
#[irc(command)]
enum Command {
    #[irc(value = "PRIVMSG", pick)]
    #[irc(value = "NOTICE")]
    Message,                          // serializes as PRIVMSG
}
```
