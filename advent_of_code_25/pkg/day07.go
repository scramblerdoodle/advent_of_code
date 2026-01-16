package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"strings"
)

func day7_pt1(input []string) int {
	board := utils.NewBoardFromValues(utils.Make2DRuneSlice(input))

	acc := 0

	for y := 0; y < board.Y; y++ {
		for x := 0; x < board.X; x++ {
			current := board.Board[y][x]

			switch current {

			case 'S':
				board.Board[y+1][x] = '|'
				utils.DebugPrintln("Found S")
				utils.PrintBoard(board)
				utils.DebugPrintln()

			case '^':
				prev := board.Board[y-1][x]

				if prev == '|' {
					if x-1 >= 0 {
						board.Board[y][x-1] = '|'
						next_l := &board.Board[y+1][x-1]
						if *next_l != '^' {
							board.Board[y+1][x-1] = '|'
						}

					}
					if x+1 < board.X {
						board.Board[y][x+1] = '|'
					}
					acc++

					utils.DebugPrintln("Found ^, prev |")
					utils.PrintBoard(board)
					utils.DebugPrintln()
				}

			case '|':
				if y+1 < board.Y {
					next := &board.Board[y+1][x]

					if *next != '^' {
						*next = '|'
					}

				}

				utils.DebugPrintln("Found | at", x, y)
				utils.PrintBoard(board)
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
