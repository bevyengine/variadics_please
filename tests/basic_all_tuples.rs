#![allow(missing_docs, dead_code)]

use static_assertions::{assert_impl_one, assert_not_impl_any};
use variadics_please::all_tuples;

trait Foo {}

macro_rules! foo {
    ($($t: ident),* $(,)?) => {
        impl<$($t),*> Foo for ($($t,)*) {}
    };
}

// [0, 2]
all_tuples!(foo, 0, 2, T);

// no {3}

// [4, 5]
all_tuples!(foo, 4, 5, T);

trait Bar {}

macro_rules! bar {
    ($(($t1: ident, $t2: ident)),* $(,)?) => {
        impl<$($t1,)* $($t2),*> Bar for ($(($t1, $t2),)*) {}
    };
}

// [0, 2]
all_tuples!(bar, 0, 2, T, U);

// [4, 5]
all_tuples!(bar, 4, 5, T, U);

// no {3}

#[test]
fn basic_test() {
    // 0
    assert_impl_one!((): Foo);
    // 1
    assert_impl_one!(((),): Foo);
    // 2
    assert_impl_one!(((), ()): Foo);
    // no 3
    assert_not_impl_any!(((), (), ()): Foo);

    // 4
    assert_impl_one!(((), (), (), ()): Foo);
    // 5
    assert_impl_one!(((), (), (), (), ()): Foo);
    // no 6
    assert_not_impl_any!(((), (), (), (), (), ()): Foo);

    // ((T, U), ..)
    // 0
    assert_impl_one!((): Bar);
    // 1
    assert_impl_one!((((), ()),): Bar);
    // 2
    assert_impl_one!((((),()), ((),())): Bar);
    // no 3
    assert_not_impl_any!((((),()), ((),()), ((),())): Bar);

    // 4
    assert_impl_one!((((),()), ((),()), ((),()), ((),())): Bar);
    // 5
    assert_impl_one!((((),()), ((),()), ((),()), ((),()), ((),())): Bar);
    // no 6
    assert_not_impl_any!((((),()), ((),()), ((),()), ((),()), ((),()), ((),())): Bar);
}
