_list:
    @just --list

# Format and lint the code
format:
    @cargo clippy --fix
    @cargo fmt

# Build for release
build:
    @cargo check
    @cargo clippy --fix
    @cargo fmt 
    @cargo llvm-cov
    @cargo build --release

# Run the tests with code coverage
test:
    @cargo check
    @cargo llvm-cov

# Remove binaries
clean:
    @cargo clean

# Install rush
install:
    @cargo install --path .
