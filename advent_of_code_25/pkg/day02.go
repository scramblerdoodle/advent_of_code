package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"strconv"
	"strings"
)

func day2_pt1(input string) int {
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
	// TODO: look up SplitSeq
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
				utils.DebugPrintln(v, v[:n], v[n:])
				count += d
			}

		}

	}
	return count
}

func day2_pt2(input string) int {
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

			n := len(v) / 2
			if v[:n] == v[n:] {
				utils.DebugPrintln(v, v[:n], v[n:])
				count += d
			}

		}

	}
	return count
}

func Day02() {
	data := utils.ReadFile("tests/day02.txt")

	ret := day2_pt1(data)
	fmt.Println(ret)

	ret = day2_pt2(data)
	fmt.Println(ret)
}
