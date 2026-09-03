# Nessak

Nessak is a new _experimental_ hash function family built on an _experimental_ hash function structure called **Tundra**

More information about the algorithm can be found on the [algorithm's repository](https://github.com/Zffu/Nessak).

This package contains multiples implementations:

- **Tundra Implementation**: The entire implementation of the Tundra structure.
- **Tundra-Powered Nessak**: A Nessak implementation using the Object Oriented Tundra implementation described above.
- **Standalone Nessak**: A "standalone" Nessak that doesn't use the Object Oriented Tundra implementation.

## Nessak Examples

In order to use the Nessak function you can use either the standalone implementation or the Tundra powered implementation. Both are very similar to use intentionally!

### Tundra Nessak

Using the _Tundra_ powered Nessak implementation is pretty straightforward:

```python
from nessak import NessakTundra

nessak = NessakTundra() # The instance of Nessak

digest1 = nessak.k2048_2048([]) # Use an existing standard

digest2 = nessak.produce_hash([], digest_size, lane_size, compression_rounds, descent_compression_rounds, inner_permutation_rounds, outer_permutation_rounds, internal_state_minimum_length_multiplier) # Or make your own!
```

### Standalone Nessak

Using the standalone _Nessak_ implementation is just as straightforward:

```python
from nessak import Nessak

nessak = Nessak()

digest1 = nessak.k2048_2048([]) # Use an existing standard

digest2 = nessak.produce_hash([], digest_size, lane_size, compression_rounds, descent_compression_rounds, inner_permutation_rounds, outer_permutation_rounds, internal_state_minimum_length_multiplier) # Or make your own!
```

## Make a new hash function

In order to create a new hash function based on the _Tundra_ structure you simply need to create a class that extends `Tundra` and implement the correct methods:

```python
from nessak import Tundra

class MyHashFunction(Tundra):
    def __init__(self):
        self.minimum_expansion_len = 56
        self.permutation_mul_size = 50
        self.part_size = 8

    def function_g(self, n: int, o: NDArray[np.uint32], w: int) -> np.uint32:
        pass

    def function_p(self, state: NDArray[np.uint32], harmonizer: NDArray[np.uint32], inner_permutation_rounds: int):
        pass

    def function_c(self, part_a: NDArray[np.uint32], part_b: NDArray[np.uint32], round: int):
        pass

    def function_d(self, lane_a: NDArray[np.uint32], lane_b: NDArray[np.uint32], k: int, p: int, r: int) -> NDArray[np.uint32]:
        pass

```
