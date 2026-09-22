default:
  just --list --unsorted

dev:
  cargo run

test:
  cargo nextest run

test-watch:
  cargo watch -x "nextest run" 
