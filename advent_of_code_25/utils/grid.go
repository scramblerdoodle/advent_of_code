package utils

import (
	"fmt"
)

type Cell interface {
	~rune | ~int | ~string
}

type Grid[T Cell] struct {
	X, Y int
	Grid [][]T
}

func NewGrid[T Cell](x, y int, defaultValue T) *Grid[T] {

	g := &Grid[T]{
		X:    x,
		Y:    y,
		Grid: make([][]T, y),
	}

	for i := range g.Grid {
		g.Grid[i] = make([]T, x)
		for j := range g.Grid[i] {
			g.Grid[i][j] = defaultValue
		}
	}

	return g
}

func NewGridFromValues[T Cell](data [][]T) *Grid[T] {
	y := len(data)
	x := len(data[0])
	g := &Grid[T]{
		X:    x,
		Y:    y,
		Grid: make([][]T, y),
	}
	for i := range g.Grid {
		g.Grid[i] = make([]T, x)
		for j := range g.Grid[i] {
			g.Grid[i][j] = data[i][j]
		}
	}

	return g
}

func (g *Grid[T]) FillGrid(data [][]T) {
	for i := 0; i < g.Y; i++ {
		for j := 0; j < g.X; j++ {
			g.Grid[i][j] = data[i][j]
		}
	}
}

func (g *Grid[T]) TransposeGrid() *Grid[T] {
	newGrid := NewGrid(g.Y, g.X, g.Grid[0][0])
	for i := 0; i < g.Y; i++ {
		for j := 0; j < g.X; j++ {
			newGrid.Grid[j][i] = g.Grid[i][j]
		}
	}

	return newGrid
}

func (g *Grid[rune]) PrintGrid() {
	if Debug {
		for y := 0; y < g.Y; y++ {
			for x := 0; x < g.X; x++ {
				printCell(g.Grid[y][x])
			}
			fmt.Println()
		}
	}
}

func printCell[T Cell](v T) {
	switch v := any(v).(type) {
	case rune:
		fmt.Print(string(v))
	case int:
		if v == 0 {
			fmt.Print(string('.'))
		} else {
			fmt.Print(v)

		}
	case string:
		fmt.Print(v)
	}
}
