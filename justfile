#!/usr/bin/env just

default: check

prep:
   mkdir -p inputs/
   @bash -c 'touch inputs/day_{01,02,03,04,05,06,07,08,09,10,11,12}.txt'

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
    cargo test

alias t := test

run *args:
    cargo run -- {{args}}

alias r := run
