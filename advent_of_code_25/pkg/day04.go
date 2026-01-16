package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"strings"
)

func day4_pt1(input []string) int {
	grid := utils.NewGridFromValues(utils.Make2DRuneSlice(input))
	acc := 0

	grid.PrintGrid()

	for y := 0; y < grid.Y; y++ {
		for x := 0; x < grid.X; x++ {
			current := grid.Grid[y][x]

			if current == '@' {
				neighbours := grid.CheckNeighbours(x, y, rune('!'))
				utils.PrintNeighbours(neighbours)
				neighbours_slice := neighbours.NeighboursToSlice()

				tp_rolls := 0
				for _, n := range neighbours_slice {
					if n == '@' {
						tp_rolls++
					}
				}

				if tp_rolls < 4 {
					acc++
				}
			}
		}
	}

	return acc
}

func day4_pt2(input []string) int {
	grid := utils.NewGridFromValues(utils.Make2DRuneSlice(input))
	acc := 0
	to_remove := 0

	grid.PrintGrid()

	for ok := true; ok; ok = to_remove > 0 {
		to_remove = 0
		for y := 0; y < grid.Y; y++ {
			for x := 0; x < grid.X; x++ {
				current := grid.Grid[y][x]

				if current == '@' {
					neighbours := grid.CheckNeighbours(x, y, rune('!'))
					utils.PrintNeighbours(neighbours)
					neighbours_slice := neighbours.NeighboursToSlice()

					tp_rolls := 0
					for _, n := range neighbours_slice {
						if n == '@' {
							tp_rolls++
						}
					}

					if tp_rolls < 4 {
						to_remove++
						grid.Grid[y][x] = '.'
					}
				}
			}
		}
		acc += to_remove
	}

	return acc
}

func Day04() {
	data := utils.ReadFile("tests/day04.txt")

	input := strings.Split(data, "\n")
	ret := day4_pt1(input)
	fmt.Println(ret)

	ret = day4_pt2(input)
	fmt.Println(ret)

}
