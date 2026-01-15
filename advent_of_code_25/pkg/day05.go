package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"strconv"
	"strings"
)

func day5_pt1(input []string) int {
	input_mode := 0 // 0: reading ranges; 1: reading ingredients
	ranges := [][2]int{}
	ingredients := []int{}

	for _, line := range input {
		if line == "\n" || line == "" {
			input_mode = 1
			continue
		}

		if input_mode == 0 {
			start_end := strings.Split(line, "-")
			start, _ := strconv.Atoi(start_end[0])
			end, _ := strconv.Atoi(start_end[1])
			ranges = append(ranges, [2]int{start, end})
		}

		if input_mode == 1 {
			ing, _ := strconv.Atoi(line)
			ingredients = append(ingredients, ing)
		}
	}

	acc := 0

	for _, ing := range ingredients {
		for _, r := range ranges {
			if ing >= r[0] && ing <= r[1] {
				acc++
				break
			}
		}
	}

	return acc
}

func overlaps_with_existing(r [2]int, ranges [][2]int) []int {
	overlappings := []int{}

	for i, existing := range ranges {
		if r[0] <= existing[1] && r[1] >= existing[0] {
			overlappings = append(overlappings, i)
		}
	}

	return overlappings
}

func day5_pt2(input []string) int {
	ranges := [][2]int{}

	for _, line := range input {
		if line == "\n" || line == "" {
			break
		}

		start_end := strings.Split(line, "-")
		start, _ := strconv.Atoi(start_end[0])
		end, _ := strconv.Atoi(start_end[1])
		ranges = append(ranges, [2]int{start, end})
	}

	// Merge overlapping ranges as they're being read
	for _, r := range ranges {
		overlap_indexes := overlaps_with_existing(r, ranges)
		for _, overlap_index := range overlap_indexes {

			if overlap_index != -1 {
				if r[0] < ranges[overlap_index][0] {
					ranges[overlap_index][0] = r[0]
				}
				if r[1] > ranges[overlap_index][1] {
					ranges[overlap_index][1] = r[1]
				}
			}
		}
	}

	merged_ranges := [][2]int{}

	// Create final ranges list with all merged versions
	for _, r := range ranges {
		overlap_indexes := overlaps_with_existing(r, merged_ranges)
		if len(overlap_indexes) == 0 {
			merged_ranges = append(merged_ranges, r)
		}
	}

	acc := 0
	for _, r := range merged_ranges {
		acc += r[1] - r[0] + 1
	}

	return acc
}

func Day05() {
	data := utils.ReadFile("tests/day05.txt")

	input := strings.Split(data, "\n")
	ret := day5_pt1(input)
	fmt.Println(ret)

	ret = day5_pt2(input)
	fmt.Println(ret)

}
