/// Unescapes an IRCv3 tag value according to the specification.
///
/// The following sequences are unescaped:
/// - `\:` → `;` (backslash + colon → semicolon)
/// - `\s` → ` ` (backslash + s → space)
/// - `\\` → `\` (backslash + backslash → backslash)
/// - `\r` → CR (backslash + r → carriage return)
/// - `\n` → LF (backslash + n → line feed)
///
/// # Examples
///
/// ```
/// use ircv3_parse::unescaped_to_escaped;
///
/// assert_eq!(unescaped_to_escaped("hello\\sworld"), "hello world");
/// assert_eq!(unescaped_to_escaped("semi\\:colon"), "semi;colon");
/// assert_eq!(unescaped_to_escaped("back\\\\slash"), "back\\slash");
/// ```
pub fn unescaped_to_escaped(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some(':') => result.push(';'),
                Some('s') => result.push(' '),
                Some('\\') => result.push('\\'),
                Some('r') => result.push('\r'),
                Some('n') => result.push('\n'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => {
                    result.push('\\');
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}
