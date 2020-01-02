// Package twofer prints the pattern "One for X, one for me." with X being a user's name or 'you'.
package twofer

import (
	"strings"
)

// ShareWith compares the name string and completes the twofer string to be printed.
func ShareWith(name string) string {
	if strings.Compare("", name) == 0 {
		name = "you"

	}
	return "One for " + name + ", one for me."
}
