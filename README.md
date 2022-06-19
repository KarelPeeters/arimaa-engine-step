# Step based Arimaa Engine

[![docs.rs][docs-badge]][docs-url]
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/arimaa-engine-step.svg
[crates-url]: https://crates.io/crates/arimaa-engine-step
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/JamesMHarmon/arimaa-engine-step/blob/master/LICENSE
[docs-badge]: https://docs.rs/arimaa-engine-step/badge.svg
[docs-url]: https://docs.rs/arimaa-engine-step/

## Overview

A game engine for the board game [Arimaa](http://arimaa.com/arimaa/). This library
provides the functionalities:

* Generate a set of valid steps from any state.
* Generate a set of valid moves from any state.
* Supports Zobrist hashing for transpositions.
* Parsers for boards and actions.

## [Documentation][docs-url]

## Example

### Parse a game state and take an action and generate a set of valid actions from that state.

```rust
use arimaa_engine_step::{action, board};

let game_state = board!(
    "2g
     +-----------------+
    8| h c d m e d c h |
    7| r r r r r r r r |
    6|     x     x     |
    5|                 |
    4|                 |
    3|     x     x     |
    2| R R R R R R R R |
    1| H C D M E D C H |
     +-----------------+
       a b c d e f g h"
);

let action = action!(d2n);
let game_state = game_state.take_action(&action);

println!("{:?}", game_state.valid_actions());
```

### [Additional Examples][docs-url]

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/tokio-rs/tokio/blob/master/LICENSE