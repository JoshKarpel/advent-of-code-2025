#!/usr/bin/env just

default: check

default := 'check'
watch cmd=default:
    uvx watchfiles --verbosity warning 'just {{cmd}}' src/ inputs/ justfile

alias w := watch

update:
    cargo update

alias upgrade := update
alias u := update

check:
    cargo fmt
    cargo check
    cargo clippy --fix --allow-dirty --allow-staged

alias c := check

test:
    cargo test --config 'build.warnings="allow"' -Z warnings

alias t := test

run *args:
    cargo run -- {{args}}

alias r := run
