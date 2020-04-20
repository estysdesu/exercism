from typing import List


class Matrix:
    """A matrix representation from a string using nested lists."""

    matrix: List[List[int]]

    def __init__(self, matrix_string) -> None:
        """Convert the matrix strings (columns separated by spaces; rows separated by newlines) to nested lists."""
        self.matrix = [
            [int(number) for number in row.split(" ")]
            for row in matrix_string.splitlines()
        ]

    def row(self, index) -> List[int]:
        """Return the matrix row given the 1-based index."""
        try:
            return self.matrix[index - 1].copy()
        except IndexError:
            raise IndexError("index is out of bounds of matrix dimensions")

    def column(self, index) -> List[int]:
        """Return the matrix column given the 1-based index."""
        try:
            return [row[index - 1] for row in self.matrix]
        except IndexError:
            raise IndexError("index is out of bounds of matrix dimensions")

    def transpose(self) -> List[List[int]]:
        self.matrix_transposed = [
            [row[i] for row in self.matrix] for i in range(len(self.matrix))
        ]
        return self.matrix_transposed.copy()

        # r, c = len(self.matrix), len(self.matrix[0])
        # self.matrix_transposed = [[0] * c for _ in range(r)]  # r-by-c zeros matrix
        # for rr in range(r):
        #     for cc in range(c):
        #         self.matrix_transposed[cc][rr] = self.matrix[rr][cc]
        # return self.matrix_transposed.copy()
