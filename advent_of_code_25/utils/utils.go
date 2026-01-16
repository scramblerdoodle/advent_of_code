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

func Make2DRuneSlice(input []string) [][]rune {
	{
		result := make([][]rune, len(input))
		for i := range input {
			result[i] = []rune(input[i])
		}
		return result
	}
}
