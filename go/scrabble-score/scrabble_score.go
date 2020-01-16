package scrabble

var letterScores = [...]struct {
	letters string
	score   int
}{
	{"AEIOULNRST", 1},
	{"DG", 2},
	{"BCMP", 3},
	{"FHVWY", 4},
	{"K", 5},
	{"JX", 8},
	{"QZ", 10},
}

// Score takes a word (or a string of letters because it doesn't validate the validity of the string) and returns the Scrabble point value assigned to it.
func Score(word string) uint16 {

	if word == "" {
		return 0
	}
	for _, r := range []rune(word) {

	}
}
