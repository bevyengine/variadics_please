//! Provides macros for implementing traits on variadic types.

// FIXME(15321): solve CI failures, then replace with `#![expect()]`.
#![allow(missing_docs, reason = "Not all docs are written yet, see #3492.")]
#![cfg_attr(any(docsrs, docsrs_dep), feature(doc_cfg))]

use proc_macro::TokenStream;
use quote::quote;
use unsynn::{format_ident, TokenStream as TokenStream2, *};

unsynn! {
    keyword KDoc = "doc";
    keyword KFakeVariadic = "fake_variadic";

    // `#[doc(fake_variadic)]`
    struct FakeVariadicAttr {
        _hash: Pound, // #
        _bracket: BracketGroupContaining::<FakeVariadicInner>,
    }

    // `doc(fake_variadic)`
    struct FakeVariadicInner {
        _doc: KDoc, // doc
        _paren: ParenthesisGroupContaining::<KFakeVariadic>,  // (fake_variadic)
    }
}

struct AllTuples {
    fake_variadic: bool,
    macro_ident: Ident,
    start: usize,
    end: usize,
    idents: Vec<Ident>,
}

impl Parser for AllTuples {
    fn parser(tokens: &mut TokenIter) -> Result<Self> {
        // Optional leading `#[doc(fake_variadic)]`
        let fake_variadic = FakeVariadicAttr::parse(tokens).is_ok();

        // macro_ident
        let macro_ident_tok = Ident::parser(tokens)?;
        let macro_ident = Ident::new(&macro_ident_tok.to_string(), Span::call_site());

        // `,`
        Comma::parser(tokens)?;

        // start
        let start_tok = LiteralInteger::parser(tokens)?;
        let start_tt = start_tok.to_token_iter().next();
        let start: usize = start_tok.value().try_into().map_err(|_| {
            Error::other::<usize>(start_tt, tokens, "start out of range".into()).unwrap_err()
        })?;

        // `,`
        Comma::parser(tokens)?;

        // end
        let end_tok = LiteralInteger::parser(tokens)?;
        let end_tt = end_tok.to_token_iter().next();
        let end: usize = end_tok.value().try_into().map_err(|_| {
            Error::other::<usize>(end_tt.clone(), tokens, "end out of range".into()).unwrap_err()
        })?;

        if end < start {
            return Error::other(end_tt, tokens, "`start` should <= `end`".into());
        }

        // `,`
        Comma::parser(tokens)?;

        // one or more idents separated by commas
        let first_tok = Ident::parser(tokens)?;
        let mut idents = vec![Ident::new(&first_tok.to_string(), Span::call_site())];

        while tokens.transaction(|t| Comma::parser(t)).is_ok() {
            let tok = Ident::parser(tokens)?;
            idents.push(Ident::new(&tok.to_string(), Span::call_site()));
        }

        Ok(AllTuples {
            fake_variadic,
            macro_ident,
            start,
            end,
            idents,
        })
    }
}

/// Helper macro to generate tuple pyramids. Useful to generate scaffolding to work around Rust
/// lacking variadics. Invoking `all_tuples!(impl_foo, start, end, P, Q, ..)`
/// invokes `impl_foo` providing ident tuples through arity `start..end`.
/// If you require the length of the tuple, see [`all_tuples_with_size!`].
///
/// # Examples
///
/// ## Single parameter
///
/// ```
/// # use core::marker::PhantomData;
/// # use variadics_please::all_tuples;
/// #
/// struct Foo<T> {
///     // ..
/// #    _phantom: PhantomData<T>
/// }
///
/// trait WrappedInFoo {
///     type Tup;
/// }
///
/// macro_rules! impl_wrapped_in_foo {
///     ($($T:ident),*) => {
///         impl<$($T),*> WrappedInFoo for ($($T,)*) {
///             type Tup = ($(Foo<$T>,)*);
///         }
///     };
/// }
///
/// // start from 0 element to 15 elements
/// all_tuples!(impl_wrapped_in_foo, 0, 15, T);
/// // impl_wrapped_in_foo!();
/// // impl_wrapped_in_foo!(T0);
/// // impl_wrapped_in_foo!(T0, T1);
/// // ..
/// // impl_wrapped_in_foo!(T0 .. T14);
/// ```
///
/// # Multiple parameters
///
/// ```
/// # use variadics_please::all_tuples;
/// #
/// trait Append {
///     type Out<Item>;
///     fn append<Item>(tup: Self, item: Item) -> Self::Out<Item>;
/// }
///
/// impl Append for () {
///     type Out<Item> = (Item,);
///     fn append<Item>(_: Self, item: Item) -> Self::Out<Item> {
///         (item,)
///     }
/// }
///
/// macro_rules! impl_append {
///     ($(($P:ident, $p:ident)),*) => {
///         impl<$($P),*> Append for ($($P,)*) {
///             type Out<Item> = ($($P),*, Item);
///             fn append<Item>(($($p,)*): Self, item: Item) -> Self::Out<Item> {
///                 ($($p),*, item)
///             }
///         }
///     }
/// }
///
/// // start from 1 element to 15 elements
/// all_tuples!(impl_append, 1, 15, P, p);
/// // impl_append!((P0, p0));
/// // impl_append!((P0, p0), (P1, p1));
/// // impl_append!((P0, p0), (P1, p1), (P2, p2));
/// // ..
/// // impl_append!((P0, p0) .. (P14, p14));
///
/// // start from 16 elements to 20
/// all_tuples!(impl_append, 16, 20, P, p);
/// // impl_append!((P0, p0) .. (P15, p15));
/// // ..
/// // impl_append!((P0, p0) .. (P19, p19));
/// ```
///
/// **`#[doc(fake_variadic)]`**
///
/// To improve the readability of your docs when implementing a trait for
/// tuples or fn pointers of varying length you can use the rustdoc-internal `fake_variadic` marker.
/// All your impls are collapsed and shown as a single `impl Trait for (F₁, F₂, …, Fₙ)`.
///
/// The `all_tuples!` macro does most of the work for you, the only change to your implementation macro
/// is that you have to accept attributes using `$(#[$meta:meta])*`.
///
/// Since this feature requires a nightly compiler, it's only enabled on docs.rs by default.
/// Add the following to your lib.rs if not already present:
///
/// ```
/// // `rustdoc_internals` is needed for `#[doc(fake_variadics)]`
/// #![cfg_attr(any(docsrs, docsrs_dep), feature(rustdoc_internals))]
/// ```
///
/// ```
/// # use variadics_please::all_tuples;
/// #
/// trait Variadic {}
///
/// impl Variadic for () {}
///
/// macro_rules! impl_variadic {
///     ($(#[$meta:meta])* $(($P:ident, $p:ident)),*) => {
///         $(#[$meta])*
///         impl<$($P),*> Variadic for ($($P,)*) {}
///     }
/// }
///
/// all_tuples!(#[doc(fake_variadic)] impl_variadic, 1, 15, P, p);
/// ```
#[proc_macro]
pub fn all_tuples(input: TokenStream) -> TokenStream {
    let input = match parse_all_tuples(input) {
        Ok(input) => input,
        Err(err) => {
            let msg = err.to_string();
            return TokenStream::from(quote! { compile_error!(#msg) });
        }
    };
    let ident_tuples = build_ident_tuples(&input);
    let macro_ident = &input.macro_ident;
    let invocations = make_invocation_range(&input).map(|n| {
        let ident_tuples = choose_ident_tuples(&input, &ident_tuples, n);
        let attrs = attrs(&input, n);
        quote! { #macro_ident!(#attrs #ident_tuples); }
    });
    TokenStream::from(quote! { #(#invocations)* })
}

/// A variant of [`all_tuples!`] that enumerates its output.
///
/// In particular, the tuples used by the inner macro will themselves be composed
/// of tuples which contain the index.
///
/// For example, with a single parameter:
/// ```
/// # use variadics_please::all_tuples_enumerated;
///
/// trait Squawk {
///     fn squawk(&self);
/// }
///
/// // If every type in a tuple is `Squawk`, the tuple can squawk by having its
/// // constituents squawk sequentially:
/// macro_rules! impl_squawk {
///     ($(($n:tt, $T:ident)),*) => {
///         impl<$($T: Squawk),*> Squawk for ($($T,)*) {
///             fn squawk(&self) {
///                 $(
///                     self.$n.squawk();
///                 )*
///             }
///         }
///     };
/// }
///
/// all_tuples_enumerated!(impl_squawk, 1, 15, T);
/// // impl_squawk!((0, T0));
/// // impl_squawk!((0, T0), (1, T1));
/// // ..
/// // impl_squawk!((0, T0) .. (14, T14));
///
/// all_tuples_enumerated!(impl_squawk, 16, 20, T);
/// // impl_append!((0, T0) .. (15, T15));
/// // ..
/// // impl_append!((0, T0) .. (19, T19));
/// ```
///
/// With multiple parameters, the result is similar, but with the additional parameters
/// included in each tuple; e.g.:
/// ```ignore
/// all_tuples_enumerated!(impl_squawk, 1, 15, P, p);
/// // impl_squawk!((0, P0, p0));
/// // impl_squawk!((0, P0, p0), (1, P1, p1));
/// // ..
/// // impl_squawk!((0, P0, p0) .. (14, P14, p14));
///
/// all_tuples_enumerated!(impl_append, 16, 20, P, p);
/// // impl_append!((0, P0, p0) .. (15, P15, p15));
/// // ..
/// // impl_append!((0, P0, p0) .. (19, P19, p19));
/// ```
#[proc_macro]
pub fn all_tuples_enumerated(input: TokenStream) -> TokenStream {
    let input = match parse_all_tuples(input) {
        Ok(input) => input,
        Err(err) => {
            let msg = err.to_string();
            return TokenStream::from(quote! { compile_error!(#msg) });
        }
    };
    let ident_tuples = build_ident_tuples_enumerated(&input);
    let macro_ident = &input.macro_ident;
    let invocations = make_invocation_range(&input).map(|n| {
        let ident_tuples = choose_ident_tuples_enumerated(&input, &ident_tuples, n);
        let attrs = attrs(&input, n);
        quote! { #macro_ident!(#attrs #ident_tuples); }
    });
    TokenStream::from(quote! { #(#invocations)* })
}

/// Helper macro to generate tuple pyramids with their length. Useful to generate scaffolding to
/// work around Rust lacking variadics. Invoking `all_tuples_with_size!(impl_foo, start, end, P, Q, ..)`
/// invokes `impl_foo` providing ident tuples through arity `start..end` preceded by their length.
/// If you don't require the length of the tuple, see [`all_tuples!`].
///
/// # Examples
///
/// ## Single parameter
///
/// ```
/// # use core::marker::PhantomData;
/// # use variadics_please::all_tuples_with_size;
/// #
/// struct Foo<T> {
///     // ..
/// #    _phantom: PhantomData<T>
/// }
///
/// trait WrappedInFoo {
///     type Tup;
///     const LENGTH: usize;
/// }
///
/// macro_rules! impl_wrapped_in_foo {
///     ($N:expr, $($T:ident),*) => {
///         impl<$($T),*> WrappedInFoo for ($($T,)*) {
///             type Tup = ($(Foo<$T>,)*);
///             const LENGTH: usize = $N;
///         }
///     };
/// }
///
/// all_tuples_with_size!(impl_wrapped_in_foo, 0, 15, T);
/// // impl_wrapped_in_foo!(0);
/// // impl_wrapped_in_foo!(1, T0);
/// // impl_wrapped_in_foo!(2, T0, T1);
/// // ..
/// // impl_wrapped_in_foo!(15, T0 .. T14);
///
/// all_tuples_with_size!(impl_wrapped_in_foo, 16, 20, T);
/// // impl_wrapped_in_foo!(16, T0 .. T15);
/// // ..
/// // impl_wrapped_in_foo!(20, T0 .. T19);
/// ```
///
/// ## Multiple parameters
///
/// ```
/// # use variadics_please::all_tuples_with_size;
/// #
/// trait Append {
///     type Out<Item>;
///     fn append<Item>(tup: Self, item: Item) -> Self::Out<Item>;
/// }
///
/// impl Append for () {
///     type Out<Item> = (Item,);
///     fn append<Item>(_: Self, item: Item) -> Self::Out<Item> {
///         (item,)
///     }
/// }
///
/// macro_rules! impl_append {
///     ($N:expr, $(($P:ident, $p:ident)),*) => {
///         impl<$($P),*> Append for ($($P,)*) {
///             type Out<Item> = ($($P),*, Item);
///             fn append<Item>(($($p,)*): Self, item: Item) -> Self::Out<Item> {
///                 ($($p),*, item)
///             }
///         }
///     }
/// }
///
/// all_tuples_with_size!(impl_append, 1, 15, P, p);
/// // impl_append!(1, (P0, p0));
/// // impl_append!(2, (P0, p0), (P1, p1));
/// // impl_append!(3, (P0, p0), (P1, p1), (P2, p2));
/// // ..
/// // impl_append!(15, (P0, p0) .. (P14, p14));
///
/// all_tuples_with_size!(impl_append, 16, 20, P, p);
/// // impl_append!(16, (P0, p0) .. (P15, p15));
/// // ..
/// // impl_append!(20, (P0, p0) .. (P19, p19));
/// ```
///
/// **`#[doc(fake_variadic)]`**
///
/// To improve the readability of your docs when implementing a trait for
/// tuples or fn pointers of varying length you can use the rustdoc-internal `fake_variadic` marker.
/// All your impls are collapsed and shown as a single `impl Trait for (F₁, F₂, …, Fₙ)`.
///
/// The `all_tuples!` macro does most of the work for you, the only change to your implementation macro
/// is that you have to accept attributes using `$(#[$meta:meta])*`.
///
/// Since this feature requires a nightly compiler, it's only enabled on docs.rs by default.
/// Add the following to your lib.rs if not already present:
///
/// ```
/// // `rustdoc_internals` is needed for `#[doc(fake_variadics)]`
/// #![cfg_attr(any(docsrs, docsrs_dep), feature(rustdoc_internals))]
/// ```
///
/// ```
/// # use variadics_please::all_tuples_with_size;
/// #
/// trait Variadic {}
///
/// impl Variadic for () {}
///
/// macro_rules! impl_variadic {
///     ($N:expr, $(#[$meta:meta])* $(($P:ident, $p:ident)),*) => {
///         $(#[$meta])*
///         impl<$($P),*> Variadic for ($($P,)*) {}
///     }
/// }
///
/// all_tuples_with_size!(#[doc(fake_variadic)] impl_variadic, 1, 15, P, p);
/// ```
#[proc_macro]
pub fn all_tuples_with_size(input: TokenStream) -> TokenStream {
    let input = match parse_all_tuples(input) {
        Ok(input) => input,
        Err(err) => {
            let msg = err.to_string();
            return TokenStream::from(quote! { compile_error!(#msg) });
        }
    };
    let ident_tuples = build_ident_tuples(&input);
    let macro_ident = &input.macro_ident;
    let invocations = make_invocation_range(&input).map(|n| {
        let ident_tuples = choose_ident_tuples(&input, &ident_tuples, n);
        let attrs = attrs(&input, n);
        quote! { #macro_ident!(#n, #attrs #ident_tuples); }
    });
    TokenStream::from(quote! { #(#invocations)* })
}

fn parse_all_tuples(input: TokenStream) -> Result<AllTuples> {
    let ts: TokenStream2 = input.into();
    let mut iter = ts.to_token_iter();
    AllTuples::parser(&mut iter)
}

fn build_ident_tuples(input: &AllTuples) -> Vec<TokenStream2> {
    (0..input.end)
        .map(|i| {
            let idents = input
                .idents
                .iter()
                .map(|ident| format_ident!("{}{}", ident, i));
            to_ident_tuple(idents, input.idents.len())
        })
        .collect()
}

fn build_ident_tuples_enumerated(input: &AllTuples) -> Vec<TokenStream2> {
    (0..input.end)
        .map(|i| {
            let idents = input
                .idents
                .iter()
                .map(|ident| format_ident!("{}{}", ident, i));
            to_ident_tuple_enumerated(idents, i)
        })
        .collect()
}

/// Returns an iterator over the invocation arities, including the optional fake-variadic `n=1`.
fn make_invocation_range(input: &AllTuples) -> impl Iterator<Item = usize> {
    let base = input.start..=input.end;
    let extra: Vec<usize> = if input.fake_variadic && input.start > 1 {
        vec![1]
    } else {
        vec![]
    };
    base.chain(extra)
}

fn choose_ident_tuples(input: &AllTuples, ident_tuples: &[TokenStream2], n: usize) -> TokenStream2 {
    // `rustdoc` uses the first ident to generate nice
    // idents with subscript numbers e.g. (F₁, F₂, …, Fₙ).
    // We don't want two numbers, so we use the
    // original, unnumbered idents for this case.
    if input.fake_variadic && n == 1 {
        let ident_tuple = to_ident_tuple(input.idents.iter().cloned(), input.idents.len());
        quote! { #ident_tuple }
    } else {
        let ident_tuples = &ident_tuples[..n];
        quote! { #(#ident_tuples),* }
    }
}

fn choose_ident_tuples_enumerated(
    input: &AllTuples,
    ident_tuples: &[TokenStream2],
    n: usize,
) -> TokenStream2 {
    if input.fake_variadic && n == 1 {
        let ident_tuple = to_ident_tuple_enumerated(input.idents.iter().cloned(), 0);
        quote! { #ident_tuple }
    } else {
        let ident_tuples = &ident_tuples[..n];
        quote! { #(#ident_tuples),* }
    }
}

fn to_ident_tuple(idents: impl Iterator<Item = Ident>, generic_num: usize) -> TokenStream2 {
    if generic_num < 2 {
        quote! { #(#idents)* }
    } else {
        quote! { (#(#idents),*) }
    }
}

/// Like `to_ident_tuple`, but it enumerates the identifiers
fn to_ident_tuple_enumerated(idents: impl Iterator<Item = Ident>, idx: usize) -> TokenStream2 {
    let idx = Literal::usize_unsuffixed(idx);
    quote! { (#idx, #(#idents),*) }
}

/// n: number of elements
fn attrs(input: &AllTuples, n: usize) -> TokenStream2 {
    if !input.fake_variadic {
        return TokenStream2::default();
    }
    match n {
        // An empty tuple (i.e. the unit type) is still documented separately,
        // so no `#[doc(hidden)]` here.
        0 => TokenStream2::default(),
        n => {
            let cfg = quote! { any(docsrs, docsrs_dep) };
            // The `#[doc(fake_variadic)]` attr has to be on the first impl block.
            if n == 1 {
                let doc = Literal::string(&format!(
                    "This trait is implemented for tuple{s1} {range} item{s2} long.",
                    range = if input.start == input.end {
                        format!("exactly {}", input.start)
                    } else {
                        format!(
                            "{down}up to {up}",
                            down = if input.start != 0 {
                                format!("down to {} ", input.start)
                            } else {
                                "".to_string()
                            },
                            up = input.end
                        )
                    },
                    s1 = if input.end > input.start { "s" } else { "" },
                    s2 = if input.end >= input.start && input.end > 1 {
                        "s"
                    } else {
                        ""
                    }
                ));
                if input.start <= 1 && input.end >= 1 {
                    // n == 1 and it's included
                    quote! {
                        #[cfg_attr(#cfg, doc(fake_variadic))]
                        #[cfg_attr(#cfg, doc = #doc)]
                    }
                } else {
                    // n == 1 but it's not included,
                    // only generate if #cfg
                    quote! {
                        #[cfg(#cfg)]
                        #[doc(fake_variadic)]
                        #[doc = #doc]
                    }
                }
            } else {
                quote! { #[cfg_attr(#cfg, doc(hidden))] }
            }
        }
    }
}
