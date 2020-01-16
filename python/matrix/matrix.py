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
            return self.matrix[index - 1]
        except IndexError:
            raise IndexError("index is out of bounds of matrix dimensions")

    def column(self, index) -> List[int]:
        """Return the matrix column given the 1-based index."""
        try:
            return [row[index - 1] for row in self.matrix]
        except IndexError:
            raise IndexError("index is out of bounds of matrix dimensions")
