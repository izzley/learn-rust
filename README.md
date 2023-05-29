# learn-rust

## 

## useful learnings 

Below list of points which I found helpful

* Build, run, repeat is slow. Speed up with [`cargo check`](https://doc.rust-lang.org/cargo/commands/cargo-check.html) instead of `cargo build` or `cargo run` to save some seconds.
* Lint using the built-in [`clippy`](https://doc.rust-lang.org/nightly/clippy/usage.html): `cargo clippy`
* [Identify outdated dependencies](https://github.com/kbknapp/cargo-outdated) which can't be updated with `cargo update`: `cargo install --locked cargo-outdated`
* If using old crate versions they may have known vulnerabilities. Use [`cargo-audit`](https://crates.io/crates/cargo-audit): `cargo install -f cargo-audit`
* Use map, filter, fold or filter_map wherever possible instead of for loop. Working with vectors is best practice