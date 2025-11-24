# Debugging

## Macro Debugging

- Print the final output of a macro using `cargo rustc --profile=check -- -Zunstable-options --pretty=expanded`
  - Alternatively you could install and use [cargo expand](https://github.com/dtolnay/cargo-expand) which adds syntax highlighting to the terminal output.
    - Additionally get pager by piping to `less` ( on Unix systems ): `cargo expand --color always | less -R`
- Print output during macro compilation using `eprintln!("hi");`

## Preview Doc

```sh
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --example all_tuples --no-deps --open
```
