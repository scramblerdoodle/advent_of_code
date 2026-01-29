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
			if grid.Grid[i][j] && !grid.Grid[i][j+1] && grid.Grid[i-1][j+1] {
				if slices.Contains(grid.Grid[i][j+1:], true) {
					for k := j + 1; ; k++ {
						if grid.Grid[i][k] {
							j = k + 1
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
		for _, r2 := range coordinates[i+1:] {
			utils.DebugPrintln("Evaluating", r1, r2)
			area := math.Abs(float64(r1.X-r2.X+1)) * math.Abs(float64(r1.Y-r2.Y+1))
			// Right off the bat, if area is lower than max, don't even check
			if area <= max_area {
				continue
			}
			utils.DebugPrintln("Evaluating rectangles starting from coordinate index", i)
			utils.DebugPrintln("Current max area:", max_area)

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

			for _, r3 := range coordinates {
				// If coordinate fall within rectangle
				if lowest_x <= r3.X && r3.X <= highest_x && lowest_y <= r3.Y && r3.Y <= highest_y {
					// Check if the surroundings that fall within the rectangle are valid
					// i.e. .#XXXXX#.
					// 			.XXXXXXX.
					// 			.XX#X#XX.
					// 			.XXX.XXX.
					// 			.#X#.@X#.
					// Checking rectangle for top-left most and down-right most
					// would have all coordinates.
					// Once we check @, its left-neighbour would be inside the rectangle
					// but outside the valid area

					neighbours := grid.GetNeighbours(r3.X, r3.Y, false)
					// r3 is the lowest X coord, right has to be valid
					if r3.X == lowest_x {
						if !neighbours.E {
							utils.DebugPrintln("Invalid at lowest_x because of", r3)
							valid = false
							break
						}
					}
					if r3.Y == lowest_y {
						if !neighbours.S {
							utils.DebugPrintln("Invalid at lowest_y because of", r3)
							valid = false
							break
						}
					}
					if r3.X == highest_x {
						if !neighbours.W {
							utils.DebugPrintln("Invalid at highest_x because of", r3)
							valid = false
							break
						}
					}
					if r3.Y == highest_y {
						if !neighbours.N {
							utils.DebugPrintln("Invalid at highest_y because of", r3)
							valid = false
							break
						}
					}
					if lowest_x < r3.X && r3.X < highest_x {
						// r3 is right in the middle of X coordinates, left and right have to be valid:
						if !neighbours.W || !neighbours.E {
							utils.DebugPrintln("Invalid between lowest_x and highest_x because of", r3)
							valid = false
							break
						}
					} else if lowest_y < r3.Y && r3.Y < highest_y {
						// r3 is right in the middle of Y coordinates, up and down have to be valid:
						if !neighbours.N || !neighbours.S {
							utils.DebugPrintln("Invalid between lowest_y and highest_y because of", r3)
							valid = false
							break
						}
					} else if r3.X == lowest_x {
						// r3 is on the same column as lowest X
						// If below, up and right have to valid
						if r3.Y > lowest_y {
							if !neighbours.N || !neighbours.E {
								utils.DebugPrintln("Invalid at lowest_x and >= lowest_y because of", r3)
								valid = false
								break
							}
						}
						// If above, down and right have to be valid
						if r3.Y < highest_y {
							if !neighbours.S || !neighbours.E {
								utils.DebugPrintln("Invalid at lowest_x and <= highest_y because of", r3)
								valid = false
								break
							}
						}
					} else if r3.X == highest_x {
						// r3 is on the same column as highest X
						// If below, up and left have to valid
						if r3.Y > lowest_y {
							if !neighbours.N || !neighbours.W {
								utils.DebugPrintln("Invalid at highest_x and >= lowest_y because of", r3)
								valid = false
								break
							}
						}
						// If above, down and left have to valid
						if r3.Y < highest_y {
							if !neighbours.S || !neighbours.W {
								utils.DebugPrintln("Invalid at highest_x and <= highest_y because of", r3)
								valid = false
								break
							}
						}
					} else if r3.Y == lowest_y {
						// r3 is on the same row as lowest Y
						// If to the right, down and left have to valid
						if r3.X > lowest_x {
							if !neighbours.S || !neighbours.W {
								utils.DebugPrintln("Invalid at lowest_y and >= lowest_x because of", r3)
								valid = false
								break
							}
						}
						// If to the left, down and right have to valid
						if r3.X < highest_x {
							if !neighbours.S || !neighbours.E {
								utils.DebugPrintln("Invalid at lowest_y and <= highest_x because of", r3)
								valid = false
								break
							}
						}
					} else if r3.Y == highest_y {
						// r3 is on the same row as highest Y
						// If to the right, up and left have to valid
						if r3.X > lowest_x {
							if !neighbours.N || !neighbours.W {
								utils.DebugPrintln("Invalid at highest_y and >= lowest_x because of", r3)
								valid = false
								break
							}
						}
						// If to the left, up and right have to valid
						if r3.X < highest_x {
							if !neighbours.N || !neighbours.E {
								utils.DebugPrintln("Invalid at highest_y and <= highest_x because of", r3)
								valid = false
								break
							}
						}
					}
				}
			}

			if area > max_area && valid {
				utils.DebugPrintln(r1, r2)
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
	// Ret: 4739623064, too high
	// Ret: 4684846992, too high
	fmt.Println(ret)
}
