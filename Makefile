.PHONY: setup gen lint test cross links check

setup:
	mise run install_all

gen:
	mise run gen

lint:
	mise run lint

test:
	mise run test

cross:
	mise run cross

links:
	@if command -v lychee >/dev/null 2>&1; then \
		lychee --no-progress .; \
	else \
		echo "lychee not installed; skipping link check"; \
	fi

check: lint test links
