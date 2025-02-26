macro_rules! serde_default_fn {
    ($name: ident, $t: ty) => {
        pub const fn $name<const V: $t>() -> $t {
            V
        }
    };
}
serde_default_fn!(default_i16, i16);
