package main

import (
	"fmt"
    "bufio"
	"sync"
	"os"
)

func main() {
	answers := make([]int32, 3)
	var wg sync.WaitGroup
    scanner := bufio.NewScanner(os.Stdin)

	for i := range answers {
		var cities int32
		var boxes int32

		scanner.Scan()
		fmt.Sscanf(scanner.Text(), "%d %d", &cities, &boxes)
		// fmt.Println("CITIES", cities)

		if cities <= 0 {
			break
		}
		wg.Add(1)
	
		populations := make([]int32, cities)
		for i := range populations {
	        scanner.Scan()
			populations[i] = toInt(scanner.Bytes())
		}
		scanner.Scan()

		go func (i int) {
			defer wg.Done()
			answers[i] = solve(boxes, populations)
		}(i)
		// solve(boxes, populations)
	}
	wg.Wait()
	for _, ans := range answers {
		if ans > 0 {
			fmt.Println(ans)
		}
	}
}

func toInt(buf []byte) int32 {
	n := int32(0)
    for _, v := range buf {
        n = n*10 + int32(v-'0')
    }
	// fmt.Println("int", n)
    return n
}

func solve(boxes int32, populations []int32) int32 {
	// fmt.Println("POPS", boxes)
	max := populations[0]
	for _, pop := range populations {
		if pop > max {
			max = pop
		}
	}

	low := max/boxes
	high := max

	for low != high {
		mid := (low + high)/2
		// fmt.Println("mid", mid, "low", low, "high", high)
		if feasible(mid, boxes, populations) {
			high = mid
		} else {
			low = mid + 1
		}
	}
	return high
	
}

func feasible(max_pop int32, boxes int32, pops []int32) bool {
	for _, pop := range pops {
		req_boxes := (pop + max_pop - 1)/max_pop
		boxes -= req_boxes
	}
	return boxes >= 0
}
 
