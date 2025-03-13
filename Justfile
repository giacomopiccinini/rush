build:
    cargo check
    cargo clippy --fix
    cargo fmt 
    cargo llvm-cov
    cargo build --release

test:
    cargo check
    cargo llvm-cov