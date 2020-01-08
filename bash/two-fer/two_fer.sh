#!/usr/bin/env bash

set -e

main () {
	echo "One for ${1:-you}, one for me."
	return 0
}

main "$@"
