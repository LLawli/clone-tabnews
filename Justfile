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

migrations_dir := "infra/migrations"

# Cria uma migration nova: just migration-create create_users
migrations-create name:
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ ! "{{name}}" =~ ^[a-z0-9_]+$ ]]; then
        echo "nome inválido: use apenas letras minúsculas, números e _" >&2
        exit 1
    fi
    mkdir -p "{{migrations_dir}}"
    last=$(ls "{{migrations_dir}}" | sed -nE 's/^V([0-9]+)__.*/\1/p' | sort -n | tail -1)
    next=$(( ${last:-0} + 1 ))
    file="{{migrations_dir}}/V${next}__{{name}}.sql"
    touch "$file"
    echo "criada: $file"

migrations-run:
    cargo run --bin migrate
