package main

import (
	"advent_of_code_25/pkg"
	"advent_of_code_25/utils"
	"flag"
	"fmt"
)

func main() {
	flag.BoolVar(&utils.Debug, "d", false, "enable debug output")
	flag.Parse()

	days := flag.Args()

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

		fmt.Println("\nDay 06")
		pkg.Day06()

		fmt.Println("\nDay 07")
		pkg.Day07()

		fmt.Println("\nDay 08")
		pkg.Day08()

		fmt.Println("\nDay 09")
		pkg.Day09()

		fmt.Println("\nDay 10")
		pkg.Day10()
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

		case "06", "6":
			fmt.Println("Day 06")
			pkg.Day06()

		case "07", "7":
			fmt.Println("Day 07")
			pkg.Day07()

		case "08", "8":
			fmt.Println("Day 08")
			pkg.Day08()

		case "09", "9":
			fmt.Println("Day 09")
			pkg.Day09()

		case "10":
			fmt.Println("Day 10")
			pkg.Day10()

		default:
			fmt.Println("Unknown day", day)
		}
	}
}
