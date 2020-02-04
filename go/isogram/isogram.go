package isogram

import "strings"

// IsIsogram determines if a has any repeating letters
func IsIsogram(word string) bool {
	seen := make(map[rune]bool, 26) // max capacity is 26
	for _, r := range strings.ToUpper(word) {
		if r == '-' || r == ' ' {
			continue
		}
		if seen[r] {
			return false
		}
		seen[r] = true
	}
	return true
}
