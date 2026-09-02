<h1 style="text-align:center;">Nessak & Tundra structure</h1>
<h2 style="text-align:center;">Zffu - August 2026</h2>

This introduces a new _experimental_ hash function family called **Nessak** as well as an _experimental_ hash function structure called **Tundra**.

> [!NOTE]
> More info [here](https://zffu.dev/works/nessak/Nessak.pdf)

> This function family or structure has not went trough a professional cryptanalysis process, only through common and brute force tests. Thus, any real security cannot be claimed as these are strictly experimental for now.

**Definition of Hashing**
Hashing is a one-directional mathematical operation which is quick to calculate, yet hard to reverse. So [password](https://en.wikipedia.org/wiki/Password "Password") storage and [digital signatures](https://en.wikipedia.org/wiki/Digital_signatures "Digital signatures") benefit from hashes. Even a small change in the input results in a very different hash. So it is useful to check if two copies of data or software match. Typically the operation works on a block of input data; the hash output is then hashed with the next block, creating a new hash reflecting everything to that point; again and again until the final hash reflects everything through the final block.

A hash function is a function that performs hashing.

## The Nessak family

The _Nessak_ hash family is used as the primary way of testing and improving the _Tundra_ structure.

Here are the following recognized standards of the _Nessak_ family:

- `nessak-k2048-2048`
- `nessak-k1024-1024`
- `nessak-k512-512`
- `nessak-k256-256`
- `nessak-k256-128`
- `nessak-k256-64`
- `nessak-k256-32`
- `nessak-k256-16`

Furthermore, here are expanded standards of the _Nessak_ family:

- `nessak-k4096-2048`
- `nessak-k2048-1024`
- `nessak-k1024-512`
- `nessak-k512-256`
- `nessak-k512-128`
- `nessak-k512-64`
- `nessak-k512-32`
- `nessak-k512-16`

### Misc characteristics

| **Standard Name**   | **Internal State Minimum Length Multiplier** | **Digest size (in bits)** | **Lane size (in bits)** |
| ------------------- | -------------------------------------------- | ------------------------- | ----------------------- |
| `nessak-k2048-2048` | $1$                                          | $2048$                    | $2048$                  |
| `nessak-k1024-1024` | $1$                                          | $1024$                    | $1024$                  |
| `nessak-k512-512`   | $1$                                          | $512$                     | $512$                   |
| `nessak-k256-256`   | $1$                                          | $256$                     | $256$                   |
| `nessak-k256-128`   | $1$                                          | $128$                     | $256$                   |
| `nessak-k256-64`    | $1$                                          | $64$                      | $256$                   |
| `nessak-k256-32`    | $1$                                          | $32$                      | $256$                   |
| `nessak-k256-16`    | $1$                                          | $16$                      | $256$                   |
| `nessak-k4096-2048` | $2$                                          | $2048$                    | $4096$                  |
| `nessak-k2048-1024` | $2$                                          | $1024$                    | $2048$                  |
| `nessak-k1024-512`  | $2$                                          | $512$                     | $1024$                  |
| `nessak-k512-256`   | $2$                                          | $256$                     | $512$                   |
| `nessak-k512-128`   | $2$                                          | $128$                     | $512$                   |
| `nessak-k512-64`    | $2$                                          | $64$                      | $512$                   |
| `nessak-k512-32`    | $2$                                          | $32$                      | $512$                   |
| `nessak-k512-16`    | $2$                                          | $16$                      | $512$                   |

### Compression Characteristics

| **Standard Name**   | **Descent Compression Rounds** | **Compression Rounds** |
| ------------------- | ------------------------------ | ---------------------- |
| `nessak-k2048-2048` | $8$                            | $64$                   |
| `nessak-k1024-1024` | $8$                            | $64$                   |
| `nessak-k512-512`   | $8$                            | $64$                   |
| `nessak-k256-256`   | $8$                            | $64$                   |
| `nessak-k256-128`   | $8$                            | $64$                   |
| `nessak-k256-64`    | $8$                            | $64$                   |
| `nessak-k256-32`    | $8$                            | $64$                   |
| `nessak-k256-16`    | $8$                            | $64$                   |
| `nessak-k4096-2048` | $16$                           | $64$                   |
| `nessak-k2048-1024` | $16$                           | $64$                   |
| `nessak-k1024-512`  | $16$                           | $64$                   |
| `nessak-k512-256`   | $16$                           | $64$                   |
| `nessak-k512-128`   | $16$                           | $64$                   |
| `nessak-k512-64`    | $16$                           | $64$                   |
| `nessak-k512-32`    | $16$                           | $64$                   |
| `nessak-k512-16`    | $16$                           | $64$                   |

### Permutation Characteristics

| **Standard Name**   | **Inner Permutation Rounds** | **Outer Permutation Rounds** |
| ------------------- | ---------------------------- | ---------------------------- |
| `nessak-k2048-2048` | $24$                         | $4$                          |
| `nessak-k1024-1024` | $24$                         | $4$                          |
| `nessak-k512-512`   | $24$                         | $4$                          |
| `nessak-k256-256`   | $24$                         | $4$                          |
| `nessak-k256-128`   | $24$                         | $4$                          |
| `nessak-k256-64`    | $24$                         | $4$                          |
| `nessak-k256-32`    | $24$                         | $4$                          |
| `nessak-k256-16`    | $24$                         | $4$                          |
| `nessak-k4096-2048` | $24$                         | $16$                         |
| `nessak-k2048-1024` | $24$                         | $16$                         |
| `nessak-k1024-512`  | $24$                         | $16$                         |
| `nessak-k512-256`   | $24$                         | $16$                         |
| `nessak-k512-128`   | $24$                         | $16$                         |
| `nessak-k512-64`    | $24$                         | $16$                         |
| `nessak-k512-32`    | $24$                         | $16$                         |
| `nessak-k512-16`    | $24$                         | $16$                         |

Furthermore, due to the nature of the **Tundra** structure, new variants with modified parameters (including digest size) can easily be constructed by simply changing the configuration.

The **Nessak** family also inherits a few benefits from the structure, such as:

- _Theoretical immunity to length extension attacks_
- _Modularity_
- _Customizability of parameters without (theoretically) loss of strength_
