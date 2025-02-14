lint:
    cargo fmt --all -- --check

    cargo clippy --workspace --all-targets --all-features -- \
    	-D warnings \
    	-D trivial_casts \
    	-D trivial_numeric_casts \
    	-D unused_extern_crates \
    	-D unused_import_braces \
    	-D unused_qualifications \
    	-D clippy::all \
    	-D clippy::correctness \
    	-D clippy::suspicious \
    	-D clippy::complexity \
    	-D clippy::perf \
    	-D clippy::style

run:
    cargo run --bin snake
