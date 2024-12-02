.PHONY: all
all: ci

.PHONY: ci
ci:
	./check_copyright.sh
	npm ci
	npm run lint
	npm run build
