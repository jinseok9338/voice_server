#!/bin/sh
cargo clippy --fix --allow-staged --allow-dirty -- -W clippy::nursery
