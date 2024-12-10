.PHONY: all
all: ci

.PHONY: clean
clean:
	rm -rf node_modules dist

.PHONY: start
start:
	@if [ ! -d "node_modules" ]; then npm ci; fi
	npm start

.PHONY: ci
ci:
	./check_copyright.sh
	@if [ ! -d "node_modules" ]; then npm ci; fi
	npm run lint
	npm run build
