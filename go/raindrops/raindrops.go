package raindrops

import "strconv"

var raindrops = [...]struct {
	factor int
	snd    string
}{
	{3, "Pling"},
	{5, "Plang"},
	{7, "Plong"},
}

// Convert turns a number into a string that contains raindrop sounds corresponding to certain potential factors
func Convert(num int) string {
	var numSnd string
	for _, raindrop := range raindrops {
		if (num % raindrop.factor) == 0 {
			numSnd += raindrop.snd
		}
	}
	if numSnd == "" {
		return strconv.Itoa(num)
	}
	return numSnd
}
