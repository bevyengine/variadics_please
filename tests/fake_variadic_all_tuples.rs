#![allow(missing_docs, dead_code)]
#![cfg_attr(docsrs, feature(rustdoc_internals))]

use static_assertions::{assert_impl_one, assert_not_impl_any};
use variadics_please::all_tuples;

trait Foo {}

macro_rules! foo {
    ($(#[$meta: meta])* $($t: ident),* $(,)?) => {
        $(#[$meta])*
        impl<$($t),*> Foo for ($($t,)*) {}
    };
}

all_tuples!(
    #[doc(fake_variadic)]
    foo,
    0,
    2,
    T
);

all_tuples!(/*no attribute here, since it's added before*/ foo, 4, 5, T);

trait Bar {}

macro_rules! bar {
    ($(#[$meta: meta])* $(($t1: ident, $t2: ident)),* $(,)?) => {
        $(#[$meta])*
        impl<$($t1,)* $($t2),*> Bar for ($(($t1, $t2),)*) {}
    };
}

all_tuples!(
    #[doc(fake_variadic)]
    bar,
    0,
    2,
    T,
    U
);

all_tuples!(/*no attribute here, since it's added before*/ bar, 4, 5, T, U);

trait Baz {}

macro_rules! baz {
    ($(#[$meta: meta])* $($t: ident),* $(,)?) => {
        $(#[$meta])*
        impl<$($t),*> Baz for ($($t,)*) {}
    };
}

// no {1}
all_tuples!(
    #[doc(fake_variadic)]
    baz,
    2,
    3,
    T
);

#[test]
fn basic_test() {
    // same as `basic_*.rs`
    assert_impl_one!((): Foo);
    assert_impl_one!(((),): Foo);
    assert_impl_one!(((), ()): Foo);
    assert_not_impl_any!(((), (), ()): Foo);

    assert_impl_one!(((), (), (), ()): Foo);
    assert_impl_one!(((), (), (), (), ()): Foo);
    assert_not_impl_any!(((), (), (), (), (), ()): Foo);

    assert_impl_one!((): Bar);
    assert_impl_one!((((), ()),): Bar);
    assert_impl_one!((((),()), ((),())): Bar);
    assert_not_impl_any!((((),()), ((),()), ((),())): Bar);

    assert_impl_one!((((),()), ((),()), ((),()), ((),())): Bar);
    assert_impl_one!((((),()), ((),()), ((),()), ((),()), ((),())): Bar);
    assert_not_impl_any!((((),()), ((),()), ((),()), ((),()), ((),()), ((),())): Bar);

    // only impl for (T,) with `docsrs`
    #[cfg(docsrs)]
    assert_impl_one!(((),): Baz);
    #[cfg(not(docsrs))]
    assert_not_impl_any!(((),): Baz);
    assert_impl_one!(((), ()): Baz);
    assert_impl_one!(((), (), ()): Baz);
}
