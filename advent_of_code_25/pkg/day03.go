package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"strings"
)

func day3_pt1(input []string) int {
	// General idea:
	//	For each line,
	//		Loop i,j, i<j over each number
	//		Add str l[i] + l[j]
	//		Convert to int d
	//		Save to `largest` if d > acc
	//		acc += largest
	//	Ret acc

	acc := 0

	for _, line := range input {
		largest := 0

		for i := 0; i < len(line); i++ {
			for j := i + 1; j < len(line); j++ {
				pair := string(line[i]) + string(line[j])
				d := 0
				fmt.Sscanf(pair, "%d", &d)

				if d > largest {
					largest = d
				}
			}
		}

		acc += largest
	}

	return acc
}

func day3_pt2(input []string) int {
	acc := 0

	for _, line := range input {
		largest_str := []rune(line[:12])
		pos := -1

		for i := 0; i < 12; i++ {
			largest_digit := '0'

			for j := pos + 1; j <= len(line)-12+i; j++ {
				if rune(line[j]) > largest_digit {
					largest_digit = rune(line[j])
					pos = j
					largest_str[i] = largest_digit
				}
			}
		}
		acc += utils.RunesToInt(largest_str)
	}
	return acc
}

func Day03() {
	data := utils.ReadFile("tests/day3.txt")

	input := strings.Split(data, "\n")
	ret := day3_pt1(input)
	fmt.Println(ret)

	ret = day3_pt2(input)
	fmt.Println(ret)

}
