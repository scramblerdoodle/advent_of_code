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

				}

			case '|':
				if y+1 < grid.Y {
					next := &grid.Grid[y+1][x]

					if *next != '^' {
						*next = '|'
					}

				}

			}
		}
		grid.PrintGrid()
		utils.DebugPrintln()
	}

	return acc
}

type Coordinate struct {
	X, Y int
}

func FindCoordinate(q *utils.Queue[Coordinate], target Coordinate) (int, bool) {

	for i, v := range q.Items {
		if v.X == target.X && v.Y == target.Y {
			return i, true
		}
	}

	return -1, false
}

func day7_pt2(input []string) int {
	grid := utils.NewGridFromValues(utils.Make2DRuneSlice(input))
	values_grid := utils.NewGrid(len(input[0]), len(input), 0)

	acc := 0

	q := &utils.Queue[Coordinate]{}
	q.Enqueue(Coordinate{grid.X / 2, 0}) // Starting point
	values_grid.Grid[0][grid.X/2]++

	for coord, ok := q.Dequeue(); ok; coord, ok = q.Dequeue() {
		utils.DebugPrintln("Evaluating coordinate", coord, string(grid.Grid[coord.Y][coord.X]))

		if coord.Y+1 >= grid.Y {
			acc += values_grid.Grid[coord.Y][coord.X]
			utils.DebugPrintln("Reached the end, acc", acc)
			continue
		}
		next := &grid.Grid[coord.Y+1][coord.X]
		current_value := values_grid.Grid[coord.Y][coord.X]
		next_value := &values_grid.Grid[coord.Y+1][coord.X]

		switch *next {
		case '.':
			q.Enqueue(Coordinate{coord.X, coord.Y + 1})
			*next_value += current_value
			*next = '|'

		case '^':
			if coord.X-1 >= 0 {
				grid.Grid[coord.Y+1][coord.X-1] = '|'
				values_grid.Grid[coord.Y+1][coord.X-1] += current_value

				_, exists := FindCoordinate(q, Coordinate{coord.X - 1, coord.Y + 1})
				if !exists {
					q.Enqueue(Coordinate{coord.X - 1, coord.Y + 1})
				}
			}
			if coord.X+1 < grid.X {
				grid.Grid[coord.Y+1][coord.X+1] = '|'
				values_grid.Grid[coord.Y+1][coord.X+1] += current_value

				_, exists := FindCoordinate(q, Coordinate{coord.X + 1, coord.Y + 1})
				if !exists {
					q.Enqueue(Coordinate{coord.X + 1, coord.Y + 1})
				}
			}
		case '|':
			*next_value += current_value
		}

		grid.PrintGrid()
		utils.DebugPrintln()

	}

	values_grid.PrintGrid()
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
