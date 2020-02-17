package diffsquares

// Algorithms for this package found at https://brilliant.org/wiki/sum-of-n-n2-or-n3/#sum-of-the-first-n-positive-integers

// SumOfSquares is the sum of the squared first n natural numbers
func SumOfSquares(n int) int {
	return (n * (n + 1) * (2*n + 1)) / 6
}

// SquareOfSum is the square the sum of the first n natural numbers
func SquareOfSum(n int) int {
	sum := (n * (n + 1)) / 2
	return sum * sum
}

// Difference is the difference between the SquareOfSum and the SumOfSquares for the first n natural numbers
func Difference(n int) int {
	return SquareOfSum(n) - SumOfSquares(n)
}
