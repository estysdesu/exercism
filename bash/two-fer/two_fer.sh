#!/usr/bin/env bash

set -e 

main () {
	if [ $# -eq 0 ]; then
    		echo "One for you, one for me."
		return
	fi
#	if [ -z "$1" ]; then
#		echo "One for you, one for me."
#		return
#	fi
	echo "One for $1, one for me."
}

main "$@"
