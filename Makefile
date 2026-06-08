.PHONY: setup gen lint test conformance links check

setup:
	mise run install_all

gen:
	mise run gen

lint:
	mise run lint

test:
	mise run test

conformance:
	mise run conformance

links:
	@if command -v lychee >/dev/null 2>&1; then \
		lychee --offline --no-progress .; \
	else \
		echo "lychee not installed; skipping link check"; \
	fi

check: lint test links
