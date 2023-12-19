import math

# Two circle equations
# r1 = (x1 - x)**2 + (y1 - y)**2
# r2 = (x2 - x)**2 + (y2 - y)**2

# Now just subtract them, expand, and simplify
# r1 - r2 = x1**2 - 2*x1*x + x**2 + y1**2 - 2*y1*y + y**2 - 
#           (x2**2 - 2*x2*x + x**2 + y2**2 - 2*y2*y + y**2)
# r1 - r2 = x1**2 - 2*x1*x + x**2 + y1**2 - 2*y1*y + y**2 
#           - x2**2 + 2*x2*x - x**2 - y2**2 + 2*y2*y - y**2
# r1 - r2 = x1**2 - 2*x1*x + y1**2 - 2*y1*y  
#           - x2**2 + 2*x2*x - y2**2 + 2*y2*y 
# r1 - r2 = x1**2 - 2*x1*x + y1**2 - 2*y1*y - x2**2 + 2*x2*x - y2**2 + 2*y2*y 
# r1 - r2 = x1**2 + 2*x(x2 - x1) + y1**2 + 2*y(y2 - y1) - x2**2 - y2**2 
# r1 - r2 = 2*x(x2 - x1) + 2*y(y2 - y1) + x1**2 + y1**2 - x2**2 - y2**2 
# y = (2*x(x2 - x1) + x1**2 + y1**2 - x2**2 - y2**2 - r1 + r2)/2(y2 - y1) 

# Now we insert (x1, y1) = (1, 0), (x2, y2) = (0, 1)
# y = (-2*x - r1 + r2)/2 
# y = -x + (r2 - r1)/2
# Generic line which passes through both intersection points

# Now we substitute this into the first eq:
# r1 = (x1 - x)**2 + (y1 + x + (r1 - r2)/2)**2
# r1 = x1**2 - 2*x*x1 + x**2 + (y1 + x + (r1 - r2)/2)**2

# Again, insert (x1, y1) = (1, 0), (x2, y2) = (0, 1)
# r1 = 1 - 2*x + x**2 + (x + (r1 - r2)/2)**2
# r1 = 1 - 2*x + x**2 + x**2 + 2*x*(r1 - r2)/2 + (r1 - r2)**2/4

# let rd = r1 - r2
# r1 = 1 - 2*x + x**2 + x**2 + 2*x*rd/2 + rd**2/4
# r1 = 2*x**2 + x*(rd - 2) + 1 + rd**2/4
# 0 = x**2 + 2*x*(rd - 2)/4 + (1 + rd**2/4)/2 - r1/2
# 0 = (x + (rd - 2)/4)**2 - (rd - 2)**2/16  + (1 + rd**2/4)/2 - r1/2
# (x + (rd - 2)/4)**2 = (rd - 2)**2/16 - (1 + rd**2/4)/2 + r1/2
# x + (rd - 2)/4 = sqrt((rd - 2)**2/16 - (1 + rd**2/4)/2 + r1/2)
# x = sqrt((rd - 2)**2/16 - (1 + rd**2/4)/2 + r1/2) - (rd - 2)/4

# Use earlier to find y
# y = -x - rd/2

# As python
# x = round(math.sqrt((rd - 2)**2/16 - (1 + rd**2/4)/2 + r1/2) - (rd - 2)/4)
# y = x + rd//2

# So, what are the chanses this is correct? About 1 in 100?

# Test: (5, 7)
# r1 = 4**2 + 7**2
# r2 = 5**2 + 6**2

left = int(input())

start = 3
for r in range(start):
    for c in range(start):
        if input(f'{c} {r}\n') == '0':
            left -= 1

def validate(r1, r2, x, y):
    r1r = (x-1)**2 + y**2
    r2r = x**2 + y**2
    return r1r == r1 and r2r == r2
    

while left > 0:
    r1 = int(input("1 0\n"))
    r2 = int(input("0 1\n"))

    rd = r1 - r2 

    x = round(math.sqrt((rd - 2)**2/16 - (1 + rd**2/4)/2 + r1/2) - (rd - 2)/4)
    y = x + rd//2

    if not validate(r1, r2, x, y):
        for yo in range(-100, 100):
            for xo in range(-100, 100):
                if validate(r1, r2, x+xo, y+yo):
                    x = x+xo
                    y = y+yo

    ans = input(f"{x} {y}\n")
    if ans == '0':
        left -= 1
    else:
        print("FUCK")
        exit()
