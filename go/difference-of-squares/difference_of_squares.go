package diffsquares

// SumOfSquares is the sum of the squared first n natural numbers
func SumOfSquares(n int) int {
	var total int
	for i := 0; i <= n; i++ {
		total += i * i
	}
	return total
}

// SquareOfSum is the square the sum of the first n natural numbers
func SquareOfSum(n int) int {
	var total int
	for i := 0; i <= n; i++ {
		total += i
	}
	return total * total

}

// Difference is the difference between the SquareOfSum and the SumOfSquares for the first n natural numbers
func Difference(n int) int {
	return SquareOfSum(n) - SumOfSquares(n)
}
