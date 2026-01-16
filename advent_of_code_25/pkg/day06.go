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

	board := utils.NewBoardFromValues(formatted_input)
	b := utils.TransposeBoard(board)

	acc := 0
	for _, l := range b.Board {
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
	acc := 0
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
