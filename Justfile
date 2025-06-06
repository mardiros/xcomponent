default_testsuite:='tests/unittests'

export RUST_LOG := "xcore=warn"
export RUST_BACKTRACE := "1"

develop:
    uv run maturin develop

install:
    uv sync --group dev --frozen

update:
    uv sync --group dev --group docs

upgrade:
    uv sync --group dev --group docs --upgrade

doc:
    uv sync --group dev --group docs
    cd docs && uv run make html
    xdg-open docs/build/html/index.html

cleandoc:
    cd docs && uv run make clean
    rm -rf docs/source/develop

test: lint typecheck unittest

lint:
    uv run ruff check .

typecheck:
    uv run mypy src/python

unittest testsuite=default_testsuite: develop
    uv run pytest -sxv {{testsuite}}

lf: develop
    uv run pytest --lf -svvv

fmt:
    cargo fmt
    uv run ruff check --fix .
    uv run ruff format src tests

release major_minor_patch: && changelog
    #!/bin/bash
    cargo release {{major_minor_patch}} --no-confirm --no-tag --no-push --no-publish --execute
    git reset --soft HEAD^
    export VERSION=$(head -n 10 Cargo.toml | grep version | sed 's/.*"\([^"]*\)".*/\1/')
    sed -i "s/version = \"\(.*\)\"/version = \"${VERSION}\"/" pyproject.toml
    uv sync

changelog:
    uv run python scripts/write_changelog.py
    cat CHANGELOG.md >> CHANGELOG.md.new
    rm CHANGELOG.md
    mv CHANGELOG.md.new CHANGELOG.md
    $EDITOR CHANGELOG.md

publish:
    git commit -am "Release $(uv run scripts/get_version.py)"
    git push
    git tag "v$(uv run scripts/get_version.py)"
    git push origin "v$(uv run scripts/get_version.py)"
