.PHONY: all
all: ci

.PHONY: clean
clean:
	rm -rf node_modules dist

.PHONY: generate-api
generate-api:
	openapi-generator -i ../server/openapi.yml -g typescript -o src/api.ts

.PHONY: start
start: generate-api
	@if [ ! -d "node_modules" ]; then npm ci; fi
	npm start

.PHONY: ci
ci: generate-api
	./check_copyright.sh
	@if [ ! -d "node_modules" ]; then npm ci; fi
	npm run lint
	npm run build
