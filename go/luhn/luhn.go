package luhn

import (
	"strconv"
	"strings"
	"unicode"
)

// Valid validates if the identification string according to the [criteria for the luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm)
func Valid(identification string) bool {
	// identification = strings.TrimFunc(identification, func(r rune) bool {
	// 	return unicode.IsSpace(r)
	// })
	failRune := 'Z' // arbitrary
	identification = strings.Map(func(r rune) rune {
		switch {
		case unicode.IsSpace(r):
			return -1
		case !unicode.IsDigit(r):
			return failRune
		}
		return r
	}, identification)
	// if strings.ContainsRune(identification, failRune) {
	// 	return false
	// }
	if identification, err := strconv.Atoi(identification); err != nil {
		return false
	}
	return false
}
