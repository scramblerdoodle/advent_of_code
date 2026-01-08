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
