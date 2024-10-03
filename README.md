# NanoID Generator

This Rust program generates unique IDs using a base58 alphabet and a specified size.

### Usage

#### Command-Line Arguments

- `-n, --number <INT>`: Number of IDs to generate (default: 1).
- `-s, --size <INT>`: Size of each ID (default: 12).

#### Example

To generate 5 IDs, each of size 16:

```bash
nanoid -n 5 -s 16
```
