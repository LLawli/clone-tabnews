default:
  just --list --unsorted

dev:
  just services-up && cargo watch -x run

lint-check:
  cargo fmt --check

lint-fix:
  topcoat fmt --rustfmt

test:
  cargo nextest run

test-watch:
  cargo watch -x "nextest run"

services-up:
  podman compose -f infra/compose.yaml up -d

services-down:
  podman compose -f infra/compose.yaml down

services-stop:
  podman compose -f infra/compose.yaml stop
