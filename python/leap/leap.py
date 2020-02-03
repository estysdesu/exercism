def leap_year(year: int) -> bool:
    """Determine if a year number (4 digit integer) is a leap year."""

    if year % 4 == 0:
        if year % 100 == 0:
            if year % 400 == 0:
                return True
            else:
                return False
        else:
            return True
    else:
        return False
