# Rust Bitboard Chess Engine

A from-scratch, terminal-based chess engine written in Rust. This project demonstrates low-level systems programming, strict state management, and modular software architecture.

## Technical Architecture

* **Bitboard Representation:** Utilizes 64-bit integers (`u64`) to represent board states, enabling highly efficient bitwise operations for piece movement, ray-casting, and collision detection.
* **Modular Design:** 
  * `moves.rs`: Pure mathematical move generation and bit-shifting logic.
  * `board.rs`: State management, piece occupancy mapping, and edge-case handling (Castling, En Passant, Promotion).
  * `main.rs`: I/O handling and the interactive game loop.
* **Custom SAN Parser:** A filtering algorithm that translates context-dependent Standard Algebraic Notation (e.g., `Nf3`, `O-O`, `exd5`) into exact bitwise transformations.
* **Game State Validation:** Implements a "clone-and-make" simulation strategy to validate move legality and dynamically detect Checkmate/Stalemate conditions.

## Relevance to Systems Engineering

Building a bitboard engine requires hyper-optimized memory management, algorithmic efficiency, and precise data structuring. The techniques used here—such as bitwise masking, strict state validation, and safe state mutation in Rust—demonstrate core competencies directly applicable to high-performance computing, data science pipelines, and cloud architecture.

## How to Run

```bash
git clone [https://github.com/RejeeshKoshy/chess-engine.git](https://github.com/RejeeshKoshy/chess-engine.git)
cd chess-engine
cargo run
```

Type `help` for commands or enter standard algebraic moves(e.g., `e4`) to play.