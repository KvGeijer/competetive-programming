N = int(input())

def to_arabic(c):
    return {'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}.get(c)

for _ in range(N):
    roman = list(input())
    largest = 0

    value = 0
    for c in reversed(roman):
        c_a = to_arabic(c)
        if c_a < largest:
            value -= c_a
        else:
            largest = c_a
            value += c_a

    print(value)