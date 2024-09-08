def multiply_without_operator(a, b):
    # Handle signs to determine if the result will be negative
    is_negative = (a < 0) != (b < 0)
    
    # Work with absolute values for easier calculations
    abs_a = abs(a)
    abs_b = abs(b)

    # Initialize result to 0
    result = 0.0

    # Repeated addition for the integer part of abs_b
    while abs_b >= 1:
        result += abs_a
        abs_b -= 1

    # Handle the fractional part of abs_b
    frac_part = abs_b
    precision = 1e-10  # Precision for handling small fractions
    increment = abs_a / (1.0 / precision)  # Increment for fractional multiplication
    
    while frac_part > precision:
        result += increment
        frac_part -= precision

    # Apply the correct sign to the result
    if is_negative:
        result = -result

    return result


print(multiply_without_operator(2,4.2))