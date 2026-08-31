# Nessak

Nessak is a new _experimental_ hash function family built on an _experimental_ hash function structure called **Tundra**

More information about the algorithm can be found on the [algorithm's repository](https://github.com/Zffu/Nessak).

## Feature Flags

In order to allow customizability over what you need from this crate, multiple feature flags are used. Here below is the list of feature flags:

- `safety-checks`: This performs assertions on the given parameters (mostly sizes) to ensure that they aren't zero or that the lane sizes are properly aligned. Is done anyways when the `const` feature is enabled.
- `tundra`: Bundles the implementation for the _Tundra_ structure. Allowing you to create your own hash function implementations based on it.
- `tundra-nessak`: Bundles the _Nessak_ implementation using the _Tundra_ structure implementation.
- `standalone`: Bundles a standalone implementation of _Nessak_ that doesn't rely on the _Tundra_ structure implementation but still uses it internally.
- `const`: Enforces every parameter to be selected at compile time, removing the capacity of setting parameters at runtime but allowing for potentially higher performance while trading for file size (if using multiple parameter sets / standards).
- `nessak_impls`: Bundles the Nessak "implementation" functions but not the full Nessak function. These functions are used by every Nessak implementation.

## Nessak Examples

### Tundra Nessak

> Requires the `tundra-nessak` crate feature.

Using the _Nessak_ Tundra implementation is pretty straightforward:

> Requires the `const` crate feature to be inactive.

```rust
use nessak::tundra::nessak::NessakTundraImplementation;

let hash = NessakTundraImplementation::k256_256(&[]); // Use a common standard

let hash = NessakTundraImplementation::produce_hash(&[], digest_size, lane_size, compression_rounds, descent_compression_rounds, inner_permutation_rounds, outer_permutation_rounds, internal_state_minimum_length_multiplier); // Make your own standard!
```

And with the const variant:

> Requires the `const` crate feature.

```rust
use nessak::presets::{NessakK256_256, TundraPreset};
use nessak::tundra::nessak::NessakTundraImplementation;

let hash = NessakTundraImplementation::produce_hash::<NessakK256_256>(&[]); // Use a common standard

pub struct MyPreset;
impl TundraPreset for MyPreset {
	const DIGEST_SIZE: usize = 128;
	const LANE_SIZE: usize = 128;
	const COMPRESSION_ROUNDS: usize = 64;
	const DESCENT_COMPRESSION_ROUNDS: usize  16;
	const INNER_PERMUTATION_ROUNDS: usize = 24;
	const OUTER_PERMUTATION_ROUNDS: usize = 16;
	const INTERNAL_STATE_MINIMUM_LENGTH_MULTIPLIER: usize = 2;
} // Or make your own!

let hash = NessakTundraImplementation::produce_hash::<MyPreset>(&[]); // And use it like any other preset
```

### Standalone Nessak

> Requires the `standalone` crate feature.

Using the standalone _Nessak_ implementation is pretty straightforward:

> Requires the `const` crate feature to be inactive.

```rust
use nessak::standalone::Nessak;

let hash = Nessak::k256_256(&[]); // Use a common standard

let hash = Nessak::produce_hash(&[], digest_size, lane_size, compression_rounds, descent_compression_rounds, inner_permutation_rounds, outer_permutation_rounds, internal_state_minimum_length_multiplier); // Make your own standard!
```

And with the const variant:

> Requires the `const` crate feature.

```rust
use nessak::standalone::Nessak;

use nessak::presets::{NessakK256_256, TundraPreset};

let hash = Nessak::produce_hash::<NessakK256_256>(&[]); // Use a common standard

pub struct MyPreset;
impl TundraPreset for MyPreset {
	const DIGEST_SIZE: usize = 128;
	const LANE_SIZE: usize = 128;
	const COMPRESSION_ROUNDS: usize = 64;
	const DESCENT_COMPRESSION_ROUNDS: usize  16;
	const INNER_PERMUTATION_ROUNDS: usize = 24;
	const OUTER_PERMUTATION_ROUNDS: usize = 16;
	const INTERNAL_STATE_MINIMUM_LENGTH_MULTIPLIER: usize = 2;
} // Or make your own!

let hash = Nessak::produce_hash::<MyPreset>(&[]); // And use it like any other preset
```
