package isogram

import "unicode"

// IsIsogram determines if a word is an isogram
func IsIsogram(word string) bool {
	uniqRunes := make(map[rune]bool, 26) // max capacity is 26
	for _, r := range word {
		if r == rune('-') || r == rune(' ') {
			continue
		}
		R := unicode.ToUpper(r)
		if uniqRunes[R] {
			return false
		}
		uniqRunes[R] = true
	}
	return true
}
