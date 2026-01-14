#![allow(missing_docs, dead_code)]

use static_assertions::{assert_impl_one, assert_not_impl_any};
use variadics_please::all_tuples_with_size;

trait Foo {
    const SIZE: usize;
}

macro_rules! foo {
    ($size: literal, $($t: ident),* $(,)?) => {
        impl<$($t),*> Foo for ($($t,)*) {
            const SIZE: usize = $size;
        }
    };
}

// [0, 2]
all_tuples_with_size!(foo, 0, 2, T);

// no {3}

// [4, 5]
all_tuples_with_size!(foo, 4, 5, T);

trait Bar {
    const SIZE: usize;
}

macro_rules! bar {
    ($size: literal, $(($t1: ident, $t2: ident)),* $(,)?) => {
        impl<$($t1,)* $($t2),*> Bar for ($(($t1, $t2),)*) {
            const SIZE: usize = $size;
        }
    };
}

// [0, 2]
all_tuples_with_size!(bar, 0, 2, T, U);

// no {3}

// [4, 5]
all_tuples_with_size!(bar, 4, 5, T, U);

#[test]
fn basic_test() {
    // 0
    assert_impl_one!((): Foo);
    assert_eq!(<() as Foo>::SIZE, 0);
    // 1
    assert_impl_one!(((),): Foo);
    assert_eq!(<((),) as Foo>::SIZE, 1);
    // 2
    assert_impl_one!(((), ()): Foo);
    assert_eq!(<((), ()) as Foo>::SIZE, 2);
    // no 3
    assert_not_impl_any!(((), (), ()): Foo);

    // 4
    assert_impl_one!(((), (), (), ()): Foo);
    assert_eq!(<((), (), (), ()) as Foo>::SIZE, 4);
    // 5
    assert_impl_one!(((), (), (), (), ()): Foo);
    assert_eq!(<((), (), (), (), ()) as Foo>::SIZE, 5);
    // no 6
    assert_not_impl_any!(((), (), (), (), (), ()): Foo);

    // ((T, U), ..)
    // 0
    assert_impl_one!((): Bar);
    assert_eq!(<() as Bar>::SIZE, 0);
    // 1
    assert_impl_one!((((), ()),): Bar);
    assert_eq!(<(((), ()),) as Bar>::SIZE, 1);
    // 2
    assert_impl_one!((((),()), ((),())): Bar);
    assert_eq!(<(((), ()), ((), ())) as Bar>::SIZE, 2);
    // no 3
    assert_not_impl_any!((((),()), ((),()), ((),())): Bar);

    // 4
    assert_impl_one!((((), ()), ((), ()), ((), ()), ((), ())): Bar);
    assert_eq!(<(((), ()), ((), ()), ((), ()), ((), ())) as Bar>::SIZE, 4);
    // 5
    assert_impl_one!((((), ()), ((), ()), ((), ()), ((), ()), ((), ())): Bar);
    assert_eq!(
        <(((), ()), ((), ()), ((), ()), ((), ()), ((), ())) as Bar>::SIZE,
        5
    );
    // no 6
    assert_not_impl_any!((((),()), ((),()), ((),()), ((),()), ((),()), ((),())): Bar);
}
