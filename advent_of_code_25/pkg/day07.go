package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"strings"
)

func day7_pt1(input []string) int {
	grid := utils.NewGridFromValues(utils.Make2DRuneSlice(input))

	acc := 0

	for y := 0; y < grid.Y; y++ {
		for x := 0; x < grid.X; x++ {
			current := grid.Grid[y][x]

			switch current {

			case 'S':
				grid.Grid[y+1][x] = '|'
				utils.DebugPrintln("Found S")
				grid.PrintGrid()
				utils.DebugPrintln()

			case '^':
				prev := grid.Grid[y-1][x]

				if prev == '|' {
					if x-1 >= 0 {
						grid.Grid[y][x-1] = '|'
						next_l := &grid.Grid[y+1][x-1]
						if *next_l != '^' {
							grid.Grid[y+1][x-1] = '|'
						}

					}
					if x+1 < grid.X {
						grid.Grid[y][x+1] = '|'
					}
					acc++

					utils.DebugPrintln("Found ^, prev |")
					grid.PrintGrid()
					utils.DebugPrintln()
				}

			case '|':
				if y+1 < grid.Y {
					next := &grid.Grid[y+1][x]

					if *next != '^' {
						*next = '|'
					}

				}

				utils.DebugPrintln("Found | at", x, y)
				grid.PrintGrid()
				utils.DebugPrintln()

			}
		}
	}

	return acc
}

func day7_pt2(input []string) int {
	acc := 0
	return acc
}

func Day07() {
	data := utils.ReadFile("tests/day07.txt")

	input := strings.Split(data, "\n")
	ret := day7_pt1(input)
	fmt.Println(ret)

	ret = day7_pt2(input)
	fmt.Println(ret)
}
