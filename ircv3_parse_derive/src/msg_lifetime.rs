use proc_macro2::Span;
use syn::{GenericParam, Generics, Lifetime, LifetimeParam};

pub fn get_or_create_msg_lifetime(generics: &mut Generics) -> Lifetime {
    generics
        .lifetimes()
        .next()
        .map(|param| param.lifetime.clone())
        .unwrap_or_else(|| {
            let lifetime = Lifetime::new("'_msg", Span::call_site());
            generics.params.insert(
                0,
                GenericParam::Lifetime(LifetimeParam::new(lifetime.clone())),
            );
            lifetime
        })
}
