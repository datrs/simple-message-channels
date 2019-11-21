# simple-message-channels

Simple streamable state machine that implements a useful channel, message-type, message pattern.

A port of the JavaScript module [simple-message-channels](https://github.com/mafintosh/simple-message-channels) to Rust. Original module by [mafintosh](https://github.com/mafintosh).

## Examples

See [examples/](examples/).

The following sends three messages, transforms them, and prints the results:

```sh
cargo run --example send | cargo run --example echo_upper | cargo run --example recv

```
