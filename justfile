build: # build deployable artifact
    cargo build --release

watch: # run game in dev mode (with matchbox server)
    matchbox_server & cargo-watch -cx "run --release" && fg

test: # run unit tests
    cargo test