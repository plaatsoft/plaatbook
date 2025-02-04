#!/bin/bash

set -e

function clean() {
	rm -rf node_modules dist
}

function check_copyright() {
    exit=0
    for file in $(find src \( -name "*.js" -o -name "*.ts" -o -name "*.tsx" -o -name "*.scss" \) ! -name "api.ts"); do
        if ! grep -E -q "Copyright \(c\) 20[0-9]{2}(-20[0-9]{2})? PlaatSoft" "$file"; then
            echo "Bad copyright header in: $file"
            exit=1
        fi
    done
    if [ "$exit" -ne 0 ]; then
        exit 1
    fi
}

function generate_api() {
    openapi-generator -i ../server/openapi.yml -g typescript -o src/api.ts
}

function install_deps() {
    if [ ! -d "node_modules" ]; then
        npm ci
    fi
}

function check() {
    check_copyright
    install_deps
    npm run lint
    npm run build
}

function start() {
    install_deps
    npm start
}

case "${1:-check}" in
    clean)
        clean
        ;;
    check)
        check
        ;;
    start)
        start
        ;;
    *)
        echo "Usage: $0 {clean|check|start}"
        exit 1
        ;;
esac
