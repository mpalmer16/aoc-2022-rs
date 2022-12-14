# list the available commands
default:
  just --list

alias b := build
# build the workspace
build:
  cargo build

alias bc := build-crate
# build a specific crate in the workspace
build-crate crate:
  cargo build -p {{crate}}


alias t := test
# run all tests
test:
  cargo test

alias tc := test-crate
# run tests for a crate
test-crate crate:
  @echo 'Running tests for {{crate}}...'
  cargo test -p {{crate}}



alias gs := git-status
# git status
git-status:
  git status

alias gcp := git-commit-and-push
# git commit with message and push to remote branch
git-commit-and-push message:
  git commit -am '{{message}}'
