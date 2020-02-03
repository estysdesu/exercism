def leap_year(year: int) -> bool:
    """Determine if a year number (4 digit integer) is a leap year."""

    return year % 4 == 0 if (year % 100 != 100 or year % 400 == 0)
