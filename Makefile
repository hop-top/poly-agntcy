.PHONY: build test lint links check setup

check: lint test links

build test lint setup:
	$(MAKE) -C go $@
	$(MAKE) -C rs $@
	$(MAKE) -C php $@
	$(MAKE) -C ts $@
	$(MAKE) -C py $@

links:
	@if command -v lychee >/dev/null 2>&1; then \
		lychee --no-progress .; \
	else \
		echo "lychee not installed; skipping link check"; \
	fi
