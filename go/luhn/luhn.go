package luhn

import (
	"strings"
	"unicode"
)

// Valid validates if the identification string according to the [criteria for the luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm)
func Valid(identification string) bool {
	/*
		Trim space
		Convert to int (error means non digit characters present)
	*/
	// identification = strings.TrimFunc(identification, func(r rune) bool {
	// 	return unicode.IsSpace(r)
	// })
	// if identification, err := strconv.Atoi(identification); err != nil {
	// 	return false
	// }

	/*
		Trim space
		Fail if non-digits present
	*/
	failRune := 'Z'
	identification = strings.Map(func(r rune) rune {
		switch {
		case unicode.IsSpace(r):
			return -1
		case !unicode.IsDigit(r):
			return failRune
		default:
			return r
		}
	}, identification)
	if strings.ContainsRune(identification, failRune) {
		return false
	}
	identificationSlice := []rune(identification)
	return false
}
