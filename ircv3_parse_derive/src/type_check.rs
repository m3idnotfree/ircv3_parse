use syn::{GenericArgument, PathArguments, Type};

pub enum TypeKind<'a> {
    Str,
    String,
    Bool,
    Option(&'a Type),
    Vec(&'a Type),
    Other,
}

impl<'a> TypeKind<'a> {
    pub fn classify(ty: &'a Type) -> Self {
        if is_str(ty) {
            return Self::Str;
        }

        if is_string(ty) {
            return Self::String;
        }

        if is_bool(ty) {
            return Self::Bool;
        }

        if let Some(inner) = extract_generic_inner(ty, "Option") {
            return Self::Option(inner);
        }

        if let Some(inner) = extract_generic_inner(ty, "Vec") {
            return Self::Vec(inner);
        }

        Self::Other
    }
}

fn is_str(ty: &Type) -> bool {
    is_borrowed(ty, "str")
}

fn is_string(ty: &Type) -> bool {
    is_type(ty, "String")
}

fn is_bool(ty: &Type) -> bool {
    is_type(ty, "bool")
}

fn is_type(ty: &Type, expect: &str) -> bool {
    match ty {
        Type::Path(type_path) if type_path.qself.is_none() => type_path
            .path
            .segments
            .last()
            .map(|seg| seg.ident == expect)
            .unwrap_or(false),
        _ => false,
    }
}

fn is_borrowed(ty: &Type, expect: &str) -> bool {
    match ty {
        Type::Reference(type_ref) => match type_ref.elem.as_ref() {
            Type::Path(type_path) => type_path
                .path
                .segments
                .last()
                .map(|seg| seg.ident == expect)
                .unwrap_or(false),
            _ => false,
        },
        _ => false,
    }
}

fn extract_generic_inner<'a>(ty: &'a Type, wrapper: &str) -> Option<&'a Type> {
    let type_path = match ty {
        Type::Path(type_path) if type_path.qself.is_none() => type_path,
        _ => return None,
    };

    let last_segment = match type_path.path.segments.last() {
        Some(seg) if seg.ident == wrapper => seg,
        _ => return None,
    };

    match &last_segment.arguments {
        PathArguments::AngleBracketed(args) if args.args.len() == 1 => match args.args.first() {
            Some(GenericArgument::Type(inner_ty)) => Some(inner_ty),
            _ => None,
        },
        _ => None,
    }
}
