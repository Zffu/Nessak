# Nessak

Nessak is a new _experimental_ hash function family built on an _experimental_ hash function structure called **Tundra**

More information about the algorithm can be found on the [algorithm's repository](https://github.com/Zffu/Nessak).

## Feature Flags

In order to allow customizability over what you need from this crate, multiple feature flags are used. Here below is the list of feature flags:

- `safety-checks`: This performs assertions on the given parameters (mostly sizes) to ensure that they aren't zero or that the lane sizes are properly aligned.
- `tundra`: Bundles the implementation for the _Tundra_ structure. Allowing you to create your own hash function implementationsw based on it.
- `tundra-nessak`: Bundles the _Nessak_ implementation using the _Tundra_ structure implementation.
- `standalone`: Bundles a standalone implementation of _Nessak_ that doesn't rely on the _Tundra_ structure implementation but still uses it internally.
- `const`: Enforces every parameter to be selected at compile time, removing the capacity of setting parameters at runtime but allowing for potentially higher performance while trading for file size (if using multiple parameter sets / standards).

## Nessak Examples

### Tundra Nessak

Using the _Nessak_ Tundra implementation is pretty straightforward:

```rust
use nessak::tundra::nessak::NessakTundraImplementation;

let hash = NessakTundraImplementation::k256_256(&[]); // Use a common standard

let hash = NessakTundraImplementation::produce_hash(&[], digest_size, lane_size, compression_rounds, descent_compression_rounds, inner_permutation_rounds, outer_permutation_rounds, internal_state_minimum_length_multiplier); // Make your own standard!
```
