#![allow(missing_docs, dead_code)]
#![cfg_attr(docsrs, feature(rustdoc_internals))]

use static_assertions::{assert_impl_one, assert_not_impl_any};
use variadics_please::all_tuples_with_size;

trait Foo {
    const SIZE: usize;
}

macro_rules! foo {
    ($size: literal, $(#[$meta: meta])* $($t: ident),* $(,)?) => {
        $(#[$meta])*
        impl<$($t),*> Foo for ($($t,)*) {
            const SIZE: usize = $size;
        }
    };
}

all_tuples_with_size!(
    #[doc(fake_variadic)]
    foo,
    0,
    2,
    T
);

all_tuples_with_size!(foo, 4, 5, T);

trait Bar {
    const SIZE: usize;
}

macro_rules! bar {
    ($size: literal, $(#[$meta: meta])* $(($t1: ident, $t2: ident)),* $(,)?) => {
        $(#[$meta])*
        impl<$($t1,)* $($t2),*> Bar for ($(($t1, $t2),)*) {
            const SIZE: usize = $size;
        }
    };
}

all_tuples_with_size!(
    #[doc(fake_variadic)]
    bar,
    0,
    2,
    T,
    U
);

all_tuples_with_size!(bar, 4, 5, T, U);

trait Baz {
    const SIZE: usize;
}

macro_rules! baz {
    ($size: literal, $(#[$meta: meta])* $($t: ident),* $(,)?) => {
        $(#[$meta])*
        impl<$($t),*> Baz for ($($t,)*) {
            const SIZE: usize = $size;
        }
    };
}

// no {1}
all_tuples_with_size!(
    #[doc(fake_variadic)]
    baz,
    2,
    3,
    T
);

#[test]
fn basic_test() {
    assert_impl_one!((): Foo);
    assert_eq!(<() as Foo>::SIZE, 0);
    assert_impl_one!(((),): Foo);
    assert_eq!(<((),) as Foo>::SIZE, 1);
    assert_impl_one!(((), ()): Foo);
    assert_eq!(<((), ()) as Foo>::SIZE, 2);
    assert_not_impl_any!(((), (), ()): Foo);

    assert_impl_one!(((), (), (), ()): Foo);
    assert_eq!(<((), (), (), ()) as Foo>::SIZE, 4);
    assert_impl_one!(((), (), (), (), ()): Foo);
    assert_eq!(<((), (), (), (), ()) as Foo>::SIZE, 5);
    assert_not_impl_any!(((), (), (), (), (), ()): Foo);

    assert_impl_one!((): Bar);
    assert_eq!(<() as Bar>::SIZE, 0);
    assert_impl_one!((((), ()),): Bar);
    assert_eq!(<(((), ()),) as Bar>::SIZE, 1);
    assert_impl_one!((((),()), ((),())): Bar);
    assert_eq!(<(((), ()), ((), ())) as Bar>::SIZE, 2);
    assert_not_impl_any!((((),()), ((),()), ((),())): Bar);

    assert_impl_one!((((), ()), ((), ()), ((), ()), ((), ())): Bar);
    assert_eq!(<(((), ()), ((), ()), ((), ()), ((), ())) as Bar>::SIZE, 4);
    assert_impl_one!((((), ()), ((), ()), ((), ()), ((), ()), ((), ())): Bar);
    assert_eq!(
        <(((), ()), ((), ()), ((), ()), ((), ()), ((), ())) as Bar>::SIZE,
        5
    );
    assert_not_impl_any!((((),()), ((),()), ((),()), ((),()), ((),()), ((),())): Bar);

    // only impl for (T,) with `docsrs`
    #[cfg(docsrs)]
    assert_impl_one!(((),): Baz);
    #[cfg(not(docsrs))]
    assert_not_impl_any!(((),): Baz);
    assert_impl_one!(((), ()): Baz);
    assert_impl_one!(((), (), ()): Baz);
}
