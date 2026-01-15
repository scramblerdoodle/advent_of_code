package main

import (
	"advent_of_code_25/pkg"
	"fmt"
	"os"
)

func main() {
	days := os.Args[1:]

	if len(days) == 0 {
		fmt.Println("Day 01")
		pkg.Day01()

		fmt.Println("\nDay 02")
		pkg.Day02()

		fmt.Println("\nDay 03")
		pkg.Day03()

		fmt.Println("\nDay 04")
		pkg.Day04()

		fmt.Println("\nDay 05")
		pkg.Day05()
	}

	for i, day := range days {
		if i > 0 {
			fmt.Println()
		}

		switch day {

		case "01", "1":
			fmt.Println("Day 01")
			pkg.Day01()

		case "02", "2":
			fmt.Println("Day 02")
			pkg.Day02()

		case "03", "3":
			fmt.Println("Day 03")
			pkg.Day03()

		case "04", "4":
			fmt.Println("Day 04")
			pkg.Day04()

		case "05", "5":
			fmt.Println("Day 05")
			pkg.Day05()

		default:
			fmt.Printf("Unknown day %s", day)
		}
	}
}
