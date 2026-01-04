.PHONY: docs docs-watch docs-test

# Build the contributor book
.docs: ;

docs:
	mdbook build docs/contributor_book

docs-watch:
	mdbook serve docs/contributor_book --open

docs-test:
	mdbook test docs/contributor_book
