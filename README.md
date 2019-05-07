# anticipate

There is nothing wrong with favoring panics for the purposes of getting a
program to avoid catastrophic actions, but `expect` is a bit rough around the
corners. Namely, a classic `C` programmer will end up writing a lot of code that
looks like this (with respect to Result and Option):

```rust
fn produce_a_result<A>() -> Result<A, &str> {
  // ... produces an Ok or Err
}
fn main() {
    let result = produce_a_result();
    result.unwrap_or_else(|e| {
        eprintln!("foo: {}", e);
        std::process::exit(1);
    });
}
```

Now we have finer control over our exit status and what we print, but this will
quickly get repetitive. `anticipate` is simply an extension trait to simplify
this pattern. The above would be:

```rust
fn produce_a_result<A>() -> Result<A, &str> {
  // ... produces an Ok or Err
}
fn main() {
    let result = produce_a_result();
    result.anticipate_err("foo");
}
```

a la `expect` and `expect_err`.
