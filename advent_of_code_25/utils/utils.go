package utils

import (
	"fmt"
	"os"
	"strings"
)

func ReadFile(fileName string) string {
	fileContent, err := os.ReadFile(fileName)
	if err != nil {
		fmt.Printf("Could not read the content in the file due to %v", err)
	}
	return strings.Trim(string(fileContent), "\n")
}

func RunesToInt(r []rune) int {
	d := 0
	fmt.Sscanf(string(r), "%d", &d)
	return d
}

func Make2DRuneSlice(input []string) [][]rune {
	{
		result := make([][]rune, len(input))
		for i := range input {
			result[i] = []rune(input[i])
		}
		return result
	}
}

type Board[T any] struct {
	X, Y  int
	Board [][]T
}

func NewBoard[T any](x, y int, defaultValue T) *Board[T] {
	b := &Board[T]{
		X:     x,
		Y:     y,
		Board: make([][]T, y),
	}
	for i := range b.Board {
		b.Board[i] = make([]T, x)
		for j := range b.Board[i] {
			b.Board[i][j] = defaultValue
		}
	}

	return b
}

func NewBoardFromValues[T any](data [][]T) *Board[T] {
	y := len(data)
	x := len(data[0])
	b := &Board[T]{
		X:     x,
		Y:     y,
		Board: make([][]T, y),
	}
	for i := range b.Board {
		b.Board[i] = make([]T, x)
		for j := range b.Board[i] {
			b.Board[i][j] = data[i][j]
		}
	}

	return b
}

func FillBoard[T any](b *Board[T], data [][]T) {
	for i := 0; i < b.Y; i++ {
		for j := 0; j < b.X; j++ {
			b.Board[i][j] = data[i][j]
		}
	}
}

func TransposeBoard[T any](b *Board[T]) *Board[T] {
	newBoard := NewBoard[T](b.Y, b.X, b.Board[0][0])
	for i := 0; i < b.Y; i++ {
		for j := 0; j < b.X; j++ {
			newBoard.Board[j][i] = b.Board[i][j]
		}
	}

	return newBoard
}

func PrintBoard[T rune](b *Board[rune]) {
	for y := 0; y < b.Y; y++ {
		for x := 0; x < b.X; x++ {
			fmt.Print(string(b.Board[y][x]))
		}
		fmt.Println()
	}
}

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

func CheckNeighbours[T any](b *Board[T], x, y int, defaultValue T) Neighbours[T] {
	nw, n, ne := defaultValue, defaultValue, defaultValue
	w, e := defaultValue, defaultValue
	sw, s, se := defaultValue, defaultValue, defaultValue

	if 0 <= y-1 && y-1 < b.Y && 0 <= x-1 && x-1 < b.X {
		nw = b.Board[y-1][x-1]
	}

	if 0 <= y-1 && y-1 < b.Y && 0 <= x && x < b.X {
		n = b.Board[y-1][x]
	}

	if 0 <= y-1 && y-1 < b.Y && 0 <= x+1 && x+1 < b.X {
		ne = b.Board[y-1][x+1]
	}

	if 0 <= y && y < b.Y && 0 <= x-1 && x-1 < b.X {
		w = b.Board[y][x-1]
	}

	if 0 <= y && y < b.Y && 0 <= x+1 && x+1 < b.X {
		e = b.Board[y][x+1]
	}

	if 0 <= y+1 && y+1 < b.Y && 0 <= x-1 && x-1 < b.X {
		sw = b.Board[y+1][x-1]
	}

	if 0 <= y+1 && y+1 < b.Y && 0 <= x && x < b.X {
		s = b.Board[y+1][x]
	}

	if 0 <= y+1 && y+1 < b.Y && 0 <= x+1 && x+1 < b.X {
		se = b.Board[y+1][x+1]
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

func NeighboursToSlice[T any](n Neighbours[T]) []T {
	return []T{n.NW, n.N, n.NE, n.W, n.E, n.SW, n.S, n.SE}
}

func PrintNeighbours[T rune](n Neighbours[rune]) {
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
