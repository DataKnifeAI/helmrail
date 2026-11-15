# Image URL for build/push targets (Harbor, same pattern as windrose/palworld operators)
IMG ?= harbor.dataknife.net/library/helmrail-operator:latest

VERSION ?= 0.1.0

CARGO ?= cargo

.PHONY: all
all: build

.PHONY: help
help: ## Show this help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-16s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY: build
build: ## Build workspace
	$(CARGO) build --workspace

.PHONY: release
release: ## Build release binaries
	$(CARGO) build --workspace --release

.PHONY: test
test: ## Run tests
	$(CARGO) test --workspace

.PHONY: fmt
fmt: ## Format Rust sources
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check: ## Check formatting (CI)
	$(CARGO) fmt --all -- --check

.PHONY: lint
lint: ## Clippy with warnings denied
	$(CARGO) clippy --workspace --all-targets -- -D warnings

.PHONY: vet
vet: fmt-check lint ## Static checks (fmt + clippy)

.PHONY: ci
ci: vet test ## Local quality gate (matches CI jobs)

.PHONY: run
run: ## Run operator stub locally
	$(CARGO) run -p helmrail-operator

.PHONY: clean
clean: ## Remove target/
	$(CARGO) clean

.PHONY: docker-build
docker-build: ## Build operator image
	docker build -t $(IMG) .

.PHONY: docker-push
docker-push: ## Push operator image
	docker push $(IMG)

# Placeholders until M1 CRD install / deploy exist
.PHONY: install
install: ## Apply CRDs (M1+)
	@echo "install: not implemented until M1 CRD generation" && exit 1

.PHONY: deploy
deploy: ## Deploy operator (M1+)
	@echo "deploy: not implemented until M1 manifests" && exit 1
