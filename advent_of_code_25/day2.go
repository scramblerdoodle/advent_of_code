package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func ReadFile(fileName string) string {
	fileContent, err := os.ReadFile(fileName)
	if err != nil {
		fmt.Printf("Could not read the content in the file due to %v", err)
	}
	return string(fileContent)
}

func day2(input string) int {
	// Input is 11-22,95-115, etc
	// split on comma to get each range, then split on dash, then for loop in that range (convert from string to int)
	// for each digit in that range, convert back to string for repeat analysis
	//
	// Always even number of digits. if odd #, skip
	// "palindrome"-esque type shi
	// 11 => 1 1
	// 111 => ok
	// 1111 => 11 11
	// 12341234 => 1234 1234
	//
	// idea: split digits in half, compare first half and second half. if equal, then it's invalid, add to sum
	count := 0
	ranges := strings.Split(input, ",")
	for _, r := range ranges {
		limits := strings.Split(r, "-")

		// Convert range constraints to int
		start, _ := strconv.Atoi(limits[0])
		end, _ := strconv.Atoi(limits[1])

		for d := start; d <= end; d++ {

			// Convert back to string so we can separate it
			v := strconv.Itoa(d)

			if len(v)%2 != 0 {
				continue
			}

			n := len(v) / 2
			if v[:n] == v[n:] {
				// fmt.Println(v, v[:n], v[n:])
				count += d
			}

		}

	}
	return count
}

func day2_pt2(input string) int {
	return 0
}

func main() {
	data := ReadFile("tests/day2.txt")
	data = strings.Trim(data, "\n")

	ret := day2(data)
	fmt.Println(ret)

	ret = day2_pt2(data)
	fmt.Println(ret)
}
