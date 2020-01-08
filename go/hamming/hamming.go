package hamming

import (
	"fmt"
)

// Distance calculates and returns the Hamming distance between two strings
func Distance(a, b string) (int, error) {
	// check if strings are same length and return error early if not
	if len(a) != len(b) {
		return 0, fmt.Errorf("strings are not equal length.\na: %v\nb: %v", len(a), len(b))
	}

	// return early and don't compute Hamming distance if strings are identical
	if a == b {
		return 0, nil
	}

	// convert strings to runes and iterate over runes to calculate Hamming distance by comparing runes by index
	var c int
	runeA := []rune(a)
	runeB := []rune(b)
	for i := range runeA {
		if runeA[i] != runeB[i] {
			c++
		}
	}

	return c, nil

}
