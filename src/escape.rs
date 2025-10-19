use crate::compat::{String, Vec};

use crate::{COLON, CR, LF, SEMICOLON, SPACE};

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
    const BACKSLASH: u8 = b'\\';

    let bytes = value.as_bytes();
    let mut result = Vec::with_capacity(value.len());
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == BACKSLASH && i + 1 < bytes.len() {
            match bytes[i + 1] {
                COLON => {
                    result.push(SEMICOLON);
                    i += 2;
                }
                b's' => {
                    result.push(SPACE);
                    i += 2;
                }
                BACKSLASH => {
                    result.push(BACKSLASH);
                    i += 2;
                }
                b'r' => {
                    result.push(CR);
                    i += 2;
                }
                b'n' => {
                    result.push(LF);
                    i += 2;
                }
                other => {
                    result.push(BACKSLASH);
                    result.push(other);
                    i += 2;
                }
            }
        } else if bytes[i] == BACKSLASH {
            result.push(BACKSLASH);
            i += 1;
        } else {
            result.push(bytes[i]);
            i += 1;
        }
    }

    String::from_utf8(result).expect("Invalid UTF-8 in result")
}
