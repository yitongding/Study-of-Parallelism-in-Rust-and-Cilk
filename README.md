# Study-of-Parallelism-in-Rust-and-Cilk
Implemented several algorithms in both Cilk/C++ and Rust/Rayon in order to compare their performance

##To build cilk:
g++-5 matrixMult.cpp -fcilkplus -std=c++11

##To build Rust:
cargo build --release

##To run Rust:
cargo run --release