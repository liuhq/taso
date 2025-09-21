[positional-arguments]
@cli +cmd: fmt
    cargo run --quiet -- "$@"

@ui: fmt
    cargo run --quiet

@check: fmt
    cargo check

@fmt:
    cargo fmt

@release jobs = "8":
    cargo build --release --jobs {{jobs}}

[positional-arguments]
@preview *cmd:
    ./target/release/taso "$@"
