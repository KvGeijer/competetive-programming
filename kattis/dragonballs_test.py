def read2(text):
	return [int(i) for i in input(text).split(" ")]
	
while True:
	x, y = read2("Give me secret point in form 'x y'")

	while True:
		xp, yp = read2("guess point!")
		if xp == x and yp == y:
			print("Success!")
			break
		else:
			print((x - xp)**2 + (y - yp)**2)
