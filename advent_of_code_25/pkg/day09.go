package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"
)

func day9_pt1(input []string) int {
	coordinates := []utils.Coordinate{}
	for _, l := range input {
		values := strings.Split(l, ",")
		x, _ := strconv.Atoi(values[0])
		y, _ := strconv.Atoi(values[1])

		coordinates = append(coordinates, utils.Coordinate{X: x, Y: y})
	}

	max_area := 0.0
	for i, r1 := range coordinates {
		for _, r2 := range coordinates[i+1:] {
			area := math.Abs(float64(r1.X-r2.X+1)) * math.Abs(float64(r1.Y-r2.Y+1))
			if area > max_area {
				max_area = area
			}
		}

	}
	return int(max_area)
}

func max_X(coords []utils.Coordinate) int {
	max_x := 0
	for _, c := range coords {
		if c.X > max_x {
			max_x = c.X
		}
	}
	return max_x
}

func max_Y(coords []utils.Coordinate) int {
	max_y := 0
	for _, c := range coords {
		if c.Y > max_y {
			max_y = c.Y
		}
	}
	return max_y
}

func day9_pt2(input []string) int {
	coordinates := []utils.Coordinate{}

	// Converting string input into coordinates
	for _, l := range input {
		values := strings.Split(l, ",")
		x, _ := strconv.Atoi(values[0])
		y, _ := strconv.Atoi(values[1])

		coordinates = append(coordinates, utils.Coordinate{X: x, Y: y})
	}

	// WARNING: Everything from this point onward is complete bullshit
	// The code doesn't fill up the region properly,
	// it takes forever w/ the full 100.000 x 100.000 grid and
	// doesn't even return the correct result (too low).

	// TODO: Rework the entire logic
	// there's definitely a better way to do this without filling up a matrix w/ 1e12 booleans
	// Possible idea:
	// 	1. Fix the filling-up-the-grid part, right now we're taking it as absolute truth if any values
	// 		 on the right of the evaluated cell are inside
	// 		 Instead, fill up to the next, maybe set them as visited? Idk, there's some clever way of doing this
	// 	2. Instead of checking every single cell, loop over coordinates r1, r2 as part 1
	// 		 Then, check if there is any other edge r3 that falls inside of that region (lowest_x <= r3.X <= highest_x, etc)
	// 		 If so, check if lowest_y == r3.Y || r3.Y == highest_y; then if lowest_x < r3.X < highest_x, or vice-versa
	// 		 If it's the case for either r3.X or r3.Y, then it's an edge in the middle of the polygon, thus invalid rectangle
	// 		 Maybe we don't even need to fill up the cells in this case

	// Init grid, false means invalid spot (outside of region), true means valid
	utils.DebugPrintln("Initializing grid")
	grid := utils.NewGrid(max_X(coordinates)+2, max_Y(coordinates)+2, false)
	utils.DebugPrintln("Grid initialized")

	// Emulate coordinates looping back to start
	coordinates = append(coordinates, coordinates[0])

	// Filling up board based on coordinates
	for i, c := range coordinates[:len(coordinates)-1] {
		// Every value in coordinates is true
		grid.Grid[c.Y][c.X] = true

		// Every spot between coordinates is true
		next := coordinates[i+1]
		if c.Y == next.Y {
			lowest := min(c.X, next.X)
			highest := max(c.X, next.X)

			for x := lowest + 1; x < highest; x++ {
				grid.Grid[c.Y][x] = true
			}
		} else if c.X == next.X {
			lowest := min(c.Y, next.Y)
			highest := max(c.Y, next.Y)

			for y := lowest + 1; y < highest; y++ {
				grid.Grid[y][c.X] = true
			}
		}
	}
	// Removing loop
	coordinates = coordinates[:len(coordinates)-1]

	// Filling up area of grid between coordinates
	utils.DebugPrintln("Filling up grid areas")
	for i := 0; i < len(grid.Grid); i++ {
		for j := 0; j < len(grid.Grid[i]); j++ {
			if grid.Grid[i][j] {
				if slices.Contains(grid.Grid[i][j+1:], true) {
					for k := j + 1; ; k++ {
						if grid.Grid[i][k] {
							break
						}
						grid.Grid[i][k] = true
					}
				}
			}
		}
	}
	if len(coordinates) < 100 {
		grid.PrintGrid()

	}

	utils.DebugPrintln("Computing rectangles")
	max_area := 0.0
	for i, r1 := range coordinates {
		utils.DebugPrintln("Evaluating rectangles starting from coordinate index", i)
		utils.DebugPrintln("Current max area:", max_area)
		for _, r2 := range coordinates[i+1:] {
			area := math.Abs(float64(r1.X-r2.X+1)) * math.Abs(float64(r1.Y-r2.Y+1))
			// Right off the bat, if area is lower than max, don't even check
			if area < max_area {
				continue
			}

			// if r1.X < r2.X {
			// 	if !grid.Grid[r1.Y][r1.X+1] || !grid.Grid[r2.Y][r2.X-1] {
			// 		break
			// 	}
			// } else if r1.X > r2.X {
			// 	if !grid.Grid[r1.Y][r1.X-1] || !grid.Grid[r2.Y][r2.X+1] {
			// 		break
			// 	}
			// }
			// if r1.Y < r2.Y {
			// 	if !grid.Grid[r1.Y+1][r1.X] || !grid.Grid[r2.Y-1][r2.X] {
			// 		break
			// 	}
			// } else if r1.Y > r2.Y {
			// 	if !grid.Grid[r1.Y-1][r1.X] || !grid.Grid[r2.Y+1][r2.X] {
			// 		break
			// 	}
			// }
			valid := true

			lowest_x := min(r1.X, r2.X)
			highest_x := max(r1.X, r2.X)
			lowest_y := min(r1.Y, r2.Y)
			highest_y := max(r1.Y, r2.Y)

			for j1 := lowest_y; j1 < highest_y && valid; j1++ {
				for j2 := lowest_x; j2 < highest_x && valid; j2++ {
					if !grid.Grid[j1][j2] {
						valid = false
					}
				}
			}
			if !valid {
				continue
			}

			if area > max_area {
				max_area = area
			}
		}
	}
	return int(max_area)
}

func Day09() {
	data := utils.ReadFile("tests/day09.txt")

	input := strings.Split(data, "\n")
	ret := day9_pt1(input)
	fmt.Println(ret)

	ret = day9_pt2(input)
	// Ret: 227040352, too low
	fmt.Println(ret)
}
