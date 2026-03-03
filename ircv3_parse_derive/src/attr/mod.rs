mod enums;
mod field;
mod unit;

pub use enums::{EnumAttrs, EnumKind, VariantAttrs};
pub use field::{FieldAttrs, FieldDefault, FieldKind, Rename, Source, StructAttrs};
pub use unit::UnitStructAttrs;

use syn::{meta::ParseNestedMeta, token::Eq, Error, LitStr, Result};

use crate::error_msg;

pub const IRC: &str = "irc";
pub const TAG: &str = "tag";
pub const TAG_FLAG: &str = "tag_flag";
pub const SOURCE: &str = "source";
pub const PARAM: &str = "param";
pub const PARAMS: &str = "params";
pub const TRAILING: &str = "trailing";
pub const COMMAND: &str = "command";
pub const WITH: &str = "with";
pub const CRLF: &str = "crlf";
pub const DEFAULT: &str = "default";
pub const PRESENT: &str = "present";
pub const SKIP: &str = "skip";
pub const SKIP_NONE: &str = "skip_none";

const RENAME_ALL: &str = "rename_all";
const VALUE: &str = "value";
pub const PICK: &str = "pick";

fn parse_required_lit_str(meta: &ParseNestedMeta, attr: &str) -> Result<LitStr> {
    if meta.input.peek(Eq) {
        parse_lit_str(meta, attr)
    } else {
        Err(meta.error(error_msg::required_value(attr)))
    }
}

fn parse_lit_str(meta: &ParseNestedMeta, attr: &str) -> Result<LitStr> {
    let lit: LitStr = meta.value()?.parse()?;

    if lit.value().is_empty() {
        return Err(Error::new(lit.span(), error_msg::cannot_be_empty(attr)));
    }

    Ok(lit)
}
