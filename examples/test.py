def multiply(a, b):
    # Handle the case where b is negative
    negative_result = False
    if b < 0:
        b = -b
        negative_result = True

    # Initialize result
    result = 0

    # Add 'a' to result 'b' times
    for _ in range(b):
        result += a

    # If the result was supposed to be negative, negate it
    if negative_result:
        result = -result

    return result

print(multiply(5, 3))  # Output: 15
print(multiply(-5, 3)) # Output: -15
print(multiply(5, -3)) # Output: -15
print(multiply(-5, -3)) # Output: 15
print(multiply(0, 3))  # Output: 0
print(multiply(5, 0))  # Output: 0
