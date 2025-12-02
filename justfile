#!/usr/bin/env just

default: check

ensure:
   mkdir -p inputs/
   @bash -c 'touch inputs/day_{01,02,03,04,05,06,07,08,09,10,11,12}.txt'

default := 'check'
watch cmd=default:
    mkdir -p inputs/
    uvx watchfiles --verbosity warning 'just {{cmd}}' src/ inputs/ justfile

alias w := watch

update:
    cargo update

alias upgrade := update
alias u := update

check: ensure
    cargo fmt
    cargo check
    cargo clippy --fix --allow-dirty --allow-staged

alias c := check

test: ensure
    cargo test

alias t := test

run *args: ensure
    cargo run -- {{args}}

alias r := run
