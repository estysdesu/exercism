package scrabble

import (
	"strings"
)

var letterScores = map[string]int{
	"A": 1,
	"B": 3,
	"C": 3,
	"D": 2,
	"E": 1,
	"F": 4,
	"G": 2,
	"H": 4,
	"I": 1,
	"J": 8,
	"K": 5,
	"L": 1,
	"M": 3,
	"N": 1,
	"O": 1,
	"P": 3,
	"Q": 10,
	"R": 1,
	"S": 1,
	"T": 1,
	"U": 1,
	"V": 4,
	"W": 4,
	"X": 8,
	"Y": 4,
	"Z": 10,
}

func Score(word string) int {
	if !IsAsciiLetters(word) {
		return 0 // function signature doesn't allow returning an error as I'd prefer; chose return 0 over panic
	}
	var score int
	for _, r := range strings.ToUpper(word) {
		score += letterScores[string(r)]
	}
	return score
}

// https://golang.org/src/net/http/cookiejar/punycode.go?h=ascii#L152
func IsAsciiLetters(s string) bool {
	const startUpper = '\u0041' // A
	const stopUpper = '\u005A'  // Z
	const startLower = '\u0061' // a
	const stopLower = '\u007A'  // z
	for i := 0; i < len(s); i++ {
		if !(s[i] >= startUpper && s[i] <= stopUpper) && !(s[i] >= startLower && s[i] <= stopLower) {
			return false
		}
	}
	return true
}
