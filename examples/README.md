# Example C Bindings to the Namada Light SDK

---
This repository contains a proof-of-concept of binding the Namada light sdk (a Rust crate) to language that 
are compatible with C ABI. In this example, we use C++.

The `src/` subdirectory contains Rust ffi glue code that produces a dynamic library and C header file, built
with cbindgen. The header file produced can be found  in this sub-directory at `namada_light_sdk.h`.

The `example.cpp` file can be built into a program once linked against the `.so` (or system equivalent) file
produced by compiling the Rust ffi code. In this small example, a `RevealPk` transaction is 
    
- built in Rust and returned the C++ program
- The C++ then requests the necessary bytes to be signed
- A signature was produced offline and hardcoded into the C++ program
- The program attaches the signature to the tx.
- Then a query is sent to see if this public key has already been revealed.
- If it has not, the tx is broadcast to the ledger.

## Building and running

To first build the shared library and C header file, simply run ```cargo build```.

Afterwords, you can place the generated `namada_light_sdk.h` and `./target/debug/libnamada_light_sdk.so` 
(or system equivalent) files in this sub-directory. Then run in this sub-directory
```bash
g++ example.cpp L. -l namada_light_sdk_ffi.so
```
Don't forget to add the sub-directory to the `LD_LIBRARY_PATH`. The resulting `a.out` file can then be executed.