//! An example of using `all_tuples!`

use variadics_please::all_tuples;

fn main() {}

/// For demonstration
pub trait Foo {
    /// For demonstration
    const FOO_HARDER: bool;
    /// For demonstration
    fn foo() -> bool;
}

macro_rules! impl_tuple_foo {
    ($(#[$meta:meta])* $($name: ident),*) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        #[allow(clippy::unused_unit)]
        $(#[$meta])*
        impl<$($name: Foo),*> Foo for ($($name,)*) {
            const FOO_HARDER: bool = true $(&& $name::FOO_HARDER)*;

            fn foo() -> bool {
                true
            }
        }
    };
}

all_tuples!(
    #[doc(fake_variadic)]
    impl_tuple_foo,
    0,
    15,
    F
);
