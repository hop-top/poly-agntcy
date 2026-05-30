.PHONY: build test vet clean

build:
	mkdir -p bin
	go build -o bin/agntcy ./cmd/agntcy

test:
	go test ./...

vet:
	go vet ./...

clean:
	rm -rf bin/
