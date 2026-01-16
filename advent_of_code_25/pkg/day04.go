package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"strings"
)

func day4_pt1(input []string) int {
	board := utils.NewBoardFromValues(utils.Make2DRuneSlice(input))
	acc := 0

	utils.PrintBoard(board)

	for y := 0; y < board.Y; y++ {
		for x := 0; x < board.X; x++ {
			current := board.Board[y][x]

			if current == '@' {
				neighbours := utils.CheckNeighbours(board, x, y, rune('!'))
				utils.PrintNeighbours(neighbours)
				neighbours_slice := utils.NeighboursToSlice(neighbours)

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
	board := utils.NewBoardFromValues(utils.Make2DRuneSlice(input))
	acc := 0
	to_remove := 0

	utils.PrintBoard(board)

	for ok := true; ok; ok = to_remove > 0 {
		to_remove = 0
		for y := 0; y < board.Y; y++ {
			for x := 0; x < board.X; x++ {
				current := board.Board[y][x]

				if current == '@' {
					neighbours := utils.CheckNeighbours(board, x, y, rune('!'))
					utils.PrintNeighbours(neighbours)
					neighbours_slice := utils.NeighboursToSlice(neighbours)

					tp_rolls := 0
					for _, n := range neighbours_slice {
						if n == '@' {
							tp_rolls++
						}
					}

					if tp_rolls < 4 {
						to_remove++
						board.Board[y][x] = '.'
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
