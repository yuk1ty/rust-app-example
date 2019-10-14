extern crate macros;

pub mod app;
pub mod domain;
pub mod infra;

/// Add XXXDependencies trait for the struct you want to inject in.
#[macro_export]
macro_rules! deps {
    ($struct_:ident, $($dep_trait_:ident),*) => {
        $(
            impl $dep_trait_ for $struct_ {}
        )*
    }
}

/// Add concrete implementation to MixInXXX trait.
#[macro_export]
macro_rules! mixin {
    ($struct_:ident, $trait_:ident, $uses_trait_:ident, $func_name_:tt) => {
        impl $uses_trait_ for $struct_ {
            type $trait_ = Self;

            fn $func_name_(&self) -> &Self::$trait_ {
                &self
            }
        }
    };
}
