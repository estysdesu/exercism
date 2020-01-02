package hamming

import (
	"fmt"
	"strings"
)

func Distance(a, b string) (int, error) {
	var c int
	var e error

	if len(a) != len(b) { // check if strings are same length and return error early if not
		e = fmt.Errorf("strings are not equal length.\na: %v\nb: %v", len(a), len(b))
	} else if strings.Compare(a, b) == 0 { // return early and don't compute Hamming distance if strings are identical
		c = 0
	} else { // iterate over string a and calculate Hamming distance by comparing to string b by index
		for i, _ := range a {
			if a[i] != b[i] {
				c++
			}
		}
	}
	return c, e
}
