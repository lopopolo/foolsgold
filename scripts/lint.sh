#!/usr/bin/env bash

set -euo pipefail

yarn install
PATH="$(yarn bin):$PATH"
export PATH
cd "$(pkg-dir)"

set -x

# Yarn orchestration

## Lint package.json
pjv

# Rust sources

## Format with rustfmt
cargo fmt
## Lint with Clippy
cargo clippy --all-targets --all-features
## Lint docs
cargo doc --no-deps --all

# Lint Ruby sources

lint_ruby_sources() {
  pushd "$@" >/dev/null
  bundle install >/dev/null
  bundle exec rubocop -a
  popd >/dev/null
}

## Backend Core and StdLib
lint_ruby_sources mruby/src/extn
## spec-runner
lint_ruby_sources spec-runner/src
## mruby bins
lint_ruby_sources mruby-bin/ruby
## nemesis
lint_ruby_sources nemesis/ruby
## foolsgold
lint_ruby_sources foolsgold/ruby
## hubris
lint_ruby_sources hubris/src

# C sources

## Format with clang-format
./scripts/format-c.sh --format

# Shell sources

## Format with shfmt
shfmt -f . | grep -v target/ | grep -v node_modules/ | grep -v spec-runner/spec/ | grep -v vendor/ | xargs shfmt -i 2 -ci -s -w
## Lint with shellcheck
shfmt -f . | grep -v target/ | grep -v node_modules/ | grep -v spec-runner/spec/ | grep -v vendor/ | xargs shellcheck

# Web sources

## Format with prettier
./scripts/format-text.sh --format "css"
./scripts/format-text.sh --format "html"
./scripts/format-text.sh --format "js"
./scripts/format-text.sh --format "json"
./scripts/format-text.sh --format "yaml"
./scripts/format-text.sh --format "yml"
## Lint with eslint
yarn run eslint --fix --ext .html,.js .

# Text sources

## Format with prettier
./scripts/format-text.sh --format "md"
