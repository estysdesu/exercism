def leap_year(year: int) -> bool:
    """Determine if a year number (4 digit integer) is a leap year."""

    return year % 4 == 0 and (year % 100 != 0 or year % 400 == 0)
