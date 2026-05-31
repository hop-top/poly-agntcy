.PHONY: build test vet lint setup clean

setup:
	go mod download

build:
	mkdir -p bin
	go build -o bin/agntcy ./cmd/agntcy

test:
	go test ./...

vet:
	go vet ./...

lint:
	@if command -v golangci-lint >/dev/null 2>&1; then \
		golangci-lint run ./...; \
	else \
		echo "golangci-lint not installed; falling back to go vet"; \
		go vet ./...; \
	fi

clean:
	rm -rf bin/
