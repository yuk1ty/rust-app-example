#[macro_export]
macro_rules! dfn_deps {
    ($dep_trait_:ident, $struct_:ident) => {
        impl $dep_trait_ for $struct_ {}
    };
}

macro_rules! dfn_mixin {
    ($source_trait_:ident, $trait_:ident, $struct_:ident) => {
        impl $trait_ for $struct_ {
            type $source_trait_ = Self;
            fn task_repository(&self) -> &Self::$source_trait_ {
                &self
            }
        }
    };
}
