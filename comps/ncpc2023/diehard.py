dices = [[int(x) for x in input().split()] for _ in range(3)]

found = False
for i, sides in enumerate(dices):
    oks = [0, 0, 0]
    for j in range(3):
        if j == i:
            continue
        osides = dices[j]

        countss = 0
        countsl = 0
        for s in sides:
            smaller = len(list(filter(lambda x: x < s, osides)))
            larger = len(list(filter(lambda x: x > s, osides)))
            countss += smaller
            countsl += larger

        if countss == 0:
            pass
        elif countss >= countsl:
            oks[j] = 1

    if sum(oks) == 2:
        print(i+1)
        exit(0)


print("No dice")
