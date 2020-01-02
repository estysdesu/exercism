#!/usr/bin/env bash

set -e 

main () {
	if [ $# -eq 0 ]; then
		echo "No func params given"
		exit 1
	fi
	if [ -z "$1" ]; then
		echo "Func param is empty string"
		exit 1
	fi
	return 0
}

main "$@"
