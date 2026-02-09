package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"strconv"
	"strings"
)

func day6_pt1(input []string) int {
	formatted_input := [][]string{}
	for _, line := range input {
		formatted_input = append(formatted_input, strings.Fields(line))
	}

	grid := utils.NewGridFromValues(formatted_input)
	g := grid.TransposeGrid()

	acc := 0
	for _, l := range g.Grid {
		utils.DebugPrintln(l)

		numbers := []int{}
		for _, numstr := range l[:len(l)-1] {
			num, _ := strconv.Atoi(numstr)
			numbers = append(numbers, num)
		}
		utils.DebugPrintln(numbers)

		operation := l[len(l)-1]
		if operation == "+" {
			result := 0
			for _, n := range numbers {
				result += n
			}
			utils.DebugPrintln("result:", result)

			acc += result
		}

		if operation == "*" {
			result := 1
			for _, n := range numbers {
				result *= n
			}
			utils.DebugPrintln("result:", result)
			acc += result
		}
	}

	return acc
}

func day6_pt2(input []string) int {
	// Take input as-is, just convert to runes to handle them as chars
	formatted_input := [][]rune{}
	for _, line := range input {
		formatted_input = append(formatted_input, []rune(line))
	}

	grid := utils.NewGridFromValues(formatted_input)
	max_Y := len(grid.Grid)
	max_X := len(grid.Grid[0])

	acc := 0
	digits_col := []int{}
XLoop:
	for x := max_X - 1; x >= 0; x-- {

		// Variable to build the numbers column-wise
		digit_str := []rune{}
		for y := range max_Y {
			c := grid.Grid[y][x]

			switch c {
			case ' ':
				// do nothing

			case '+', '*':
				// run operation on column's numbers

				// add last built number to the column digits
				d, _ := strconv.Atoi(string(digit_str))
				digits_col = append(digits_col, d)
				utils.DebugPrintln("digits_col:", digits_col)

				// Compute operation
				result := 0
				if c == '+' {
					for _, n := range digits_col {
						result += n
					}
					utils.DebugPrintln("result:", result)

				} else {
					result = 1
					for _, n := range digits_col {
						result *= n
					}
					utils.DebugPrintln("result:", result)

				}

				// Add to acc
				acc += result

				// empty digits_col
				digits_col = []int{}

				// next column is empty so skip it
				x--

				// also skip to next x loop otherwise we would add the current digit back
				continue XLoop

			default:
				// append to digits_str
				digit_str = append(digit_str, c)
			}
		}

		// Append accumulated digit into digits_col for the final operation
		d, _ := strconv.Atoi(string(digit_str))
		digits_col = append(digits_col, d)
		utils.DebugPrintln("digits_col:", digits_col)
	}
	return acc
}

func Day06() {
	data := utils.ReadFile("tests/day06.txt")

	input := strings.Split(data, "\n")
	ret := day6_pt1(input)
	fmt.Println(ret)

	ret = day6_pt2(input)
	fmt.Println(ret)

}
