package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"strconv"
	"strings"
)

func day1_pt1(input []string) int {
	var pos = 50
	var count = 0

	for _, v := range input {
		// fmt.println(v)
		d, err := strconv.Atoi(v[1:])
		if err != nil {
			fmt.Printf("Error: input %v NaN, err %v", v, err)
		}

		if v[0:1] == "R" {
			pos += d
		} else {
			pos -= d
		}
		pos = pos % 100

		if pos == 0 {
			count++
		}
	}

	return count
}

func day1_pt2(input []string) int {
	var pos = 50
	var count = 0

	var prev_pos int = pos

	for _, v := range input {
		// fmt.println(v)
		d, err := strconv.Atoi(v[1:])
		if err != nil {
			fmt.Printf("Error: input %v NaN, err %v", v, err)
		}

		if v[0:1] == "R" {
			pos += d
		} else {
			pos -= d
		}

		// curr_pos := pos

		// fmt.Println(v)

		if pos < 0 {

			if prev_pos == 0 {
				count -= pos / 100
			} else {
				count += 1 - pos/100
			}

			pos = pos % 100
			pos = 100 + pos

			// fmt.Println(curr_pos, pos, count)

		} else if pos > 100 {
			// curr_pos := pos

			count += pos / 100
			pos = pos % 100

			// fmt.Println(curr_pos, pos, count)
		} else if pos == 0 {
			count++
			// fmt.Println(curr_pos, pos, count)
		}
		prev_pos = pos

	}

	return count
}

func Day01() {
	data := utils.ReadFile("tests/day1.txt")

	input := strings.Split(data, "\n")
	ret := day1_pt1(input)
	fmt.Println(ret)

	ret = day1_pt2(input)
	fmt.Println(ret)
}
