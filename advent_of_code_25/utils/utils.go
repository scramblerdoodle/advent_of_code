package utils

import (
	"fmt"
	"os"
	"strings"
)

var Debug bool

func DebugPrintln(a ...any) {
	if Debug {
		fmt.Println(a...)
	}
}

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

func Contains[T comparable](slice []T, value T) int {
	for i, v := range slice {
		if v == value {
			return i
		}
	}
	return -1
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

type Coordinate struct {
	X, Y int
}

type Coordinate3D struct {
	X, Y, Z int
}

func Pop[T any](slice []T, i int) (T, []T) {
	value := slice[i]
	return value, append(slice[:i], slice[i+1:]...)
}
