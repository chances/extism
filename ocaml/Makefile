VERSION?=0.4.0
TAG?=0.5.0

PREFIX?=$$HOME/.local

build:
	dune build

test:
	dune test
format:
	dune build @fmt --auto-promote

prepare:
	opam install .. --deps-only

publish:
	opam publish -v $(VERSION) https://github.com/extism/extism/archive/refs/tags/v$(TAG).tar.gz ..

install-cli: build
	install ../_build/default/ocaml/bin/main.exe "$(PREFIX)/bin/extism-call"
