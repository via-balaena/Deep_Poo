.PHONY: docs docs-watch docs-test

# Build the CortenForge book
.docs: ;

docs:
	mdbook build docs/cortenforge_book

docs-watch:
	mdbook serve docs/cortenforge_book --open

docs-test:
	mdbook test docs/cortenforge_book
