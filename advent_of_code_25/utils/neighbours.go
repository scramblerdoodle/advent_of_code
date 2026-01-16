package utils

import (
	"fmt"
)

type Neighbours[T any] struct {
	NW, N, NE T
	W, E      T
	SW, S, SE T
}

func NeighboursDefault[T any](defaultValue T) Neighbours[T] {
	return Neighbours[T]{
		NW: defaultValue,
		N:  defaultValue,
		NE: defaultValue,
		W:  defaultValue,
		E:  defaultValue,
		SW: defaultValue,
		S:  defaultValue,
		SE: defaultValue,
	}
}

func (g *Grid[T]) CheckNeighbours(x, y int, defaultValue T) Neighbours[T] {
	nw, n, ne := defaultValue, defaultValue, defaultValue
	w, e := defaultValue, defaultValue
	sw, s, se := defaultValue, defaultValue, defaultValue

	if 0 <= y-1 && y-1 < g.Y && 0 <= x-1 && x-1 < g.X {
		nw = g.Grid[y-1][x-1]
	}

	if 0 <= y-1 && y-1 < g.Y && 0 <= x && x < g.X {
		n = g.Grid[y-1][x]
	}

	if 0 <= y-1 && y-1 < g.Y && 0 <= x+1 && x+1 < g.X {
		ne = g.Grid[y-1][x+1]
	}

	if 0 <= y && y < g.Y && 0 <= x-1 && x-1 < g.X {
		w = g.Grid[y][x-1]
	}

	if 0 <= y && y < g.Y && 0 <= x+1 && x+1 < g.X {
		e = g.Grid[y][x+1]
	}

	if 0 <= y+1 && y+1 < g.Y && 0 <= x-1 && x-1 < g.X {
		sw = g.Grid[y+1][x-1]
	}

	if 0 <= y+1 && y+1 < g.Y && 0 <= x && x < g.X {
		s = g.Grid[y+1][x]
	}

	if 0 <= y+1 && y+1 < g.Y && 0 <= x+1 && x+1 < g.X {
		se = g.Grid[y+1][x+1]
	}

	neighbours := Neighbours[T]{
		NW: nw,
		N:  n,
		NE: ne,
		W:  w,
		E:  e,
		SW: sw,
		S:  s,
		SE: se,
	}

	return neighbours
}

func (n Neighbours[T]) NeighboursToSlice() []T {
	return []T{n.NW, n.N, n.NE, n.W, n.E, n.SW, n.S, n.SE}
}

func PrintNeighbours[T rune](n Neighbours[rune]) {
	if Debug {
		fmt.Println("----")
		fmt.Print(string(n.NW))
		fmt.Print(string(n.N))
		fmt.Print(string(n.NE))
		fmt.Println()
		fmt.Print(string(n.W))
		fmt.Print(".")
		fmt.Print(string(n.E))
		fmt.Println()
		fmt.Print(string(n.SW))
		fmt.Print(string(n.S))
		fmt.Print(string(n.SE))
		fmt.Println()
		fmt.Println("----")
	}
}
