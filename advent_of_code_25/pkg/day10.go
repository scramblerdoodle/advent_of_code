package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"github.com/mxschmitt/golang-combinations"
	"regexp"
	"strconv"
	"strings"
)

type indicatorLights []bool

func initLights(s int) indicatorLights {
	// Initializes new indicatorLights with default value `false`
	new_indicator_lights := []bool{}
	for range s {
		new_indicator_lights = append(new_indicator_lights, false)
	}
	return new_indicator_lights
}

func (l1 indicatorLights) lightsEqual(l2 indicatorLights) bool {
	// Compares two indicatorLights, returns true if they're equal, false otherwise
	if len(l1) != len(l2) {
		// TODO: raise error if mismatch in size
		return false
	}

	for i := range l1 {
		if l1[i] != l2[i] {
			return false
		}
	}
	return true
}

// TODO: handle error in case of out-of-bounds button
func (l *indicatorLights) changeLights(button []int) {
	// Alternates state of indicatorLights by flipping on/off based on the input `button` (array of indexes)
	for _, i := range button {
		(*l)[i] = !(*l)[i]
	}
}

func convert_input(input string) (indicatorLights, [][]int, []int) {
	// Match []
	indicator := []bool{}
	re_indicator := regexp.MustCompile(`\[.+\]`)
	indicator_raw := re_indicator.Find([]byte(input))
	// Ignore first and last elements (brackets), treat each char individually, convert to bool
	for _, c := range indicator_raw[1 : len(indicator_raw)-1] {
		switch c {
		case '.':
			indicator = append(indicator, false)
		case '#':
			indicator = append(indicator, true)
		}
	}
	// Example result:
	// [.##.] => [false, true, true, false]

	// All matches ()
	buttons := [][]int{}
	re_buttons := regexp.MustCompile(`\(.+\)`)
	buttons_raw := re_buttons.Find([]byte(input))
	// Split on space, then ignore first and last element of each (brackets), then split on `,`, convert to int
	for button := range strings.SplitSeq(string(buttons_raw), " ") {
		buttons_tmp := []int{}
		for b := range strings.SplitSeq(button[1:len(button)-1], ",") {
			b_i, _ := strconv.Atoi(b)
			buttons_tmp = append(buttons_tmp, b_i)
		}
		buttons = append(buttons, buttons_tmp)
	}
	// Example result:
	// (3) (1,3) (2) (2,3) (0,2) (0,1) => [[3], [1,3], [2], [2,3], [0,2], [0,1]]

	// Match {}
	joltage := []int{}
	re_joltages := regexp.MustCompile(`\{.+\}`)
	joltages_raw := re_joltages.Find([]byte(input))
	// Ignore first and last elements (brackets), split on `,`, convert to int
	for j := range strings.SplitSeq(string(joltages_raw[1:len(joltages_raw)-1]), ",") {
		j_i, _ := strconv.Atoi(j)
		joltage = append(joltage, j_i)
	}
	// Example result:
	// {3,5,4,7} => [3,5,4,7]

	return indicator, buttons, joltage
}

func day10_pt1(input []string) int {
	acc := 0
	for _, l := range input {
		// Convert line into expected structure
		target_indicator, buttons, joltage := convert_input(l)
		utils.DebugPrintln(target_indicator, buttons, joltage)

		// Get all possible combinations
		// TODO: may be overkill, could start with lowest combinations.Combinations(k), k:=1..n
		// 			 if any of the lowest ones match already, break the loop
		buttons_combos := combinations.All(buttons)

		min_button_presses := 100 // Magic number, but none of the sequences have more than 100 buttons so it's safe to set as MAX
		for _, b_c := range buttons_combos {

			// Skip if combination is longer than existing min already
			// Can't break altogether because it's not ordered by size (I don't think)
			if len(b_c) > min_button_presses {
				continue
			}

			// Init indicators to calculate their end result after pressing buttons
			curr_indicators := initLights(len(target_indicator))

			// For each button in the combination,
			for _, b := range b_c {
				// Change all of its connected lights
				curr_indicators.changeLights(b)
			}

			// If both light indicators are equal and number of buttons pressed is lower than current min,
			if curr_indicators.lightsEqual(target_indicator) && len(b_c) < min_button_presses {
				// Update min_button_presses
				min_button_presses = len(b_c)
			}

		}

		// Add min_button_presses to total
		acc += min_button_presses

	}

	return acc
}

func day10_pt2(input []string) int {
	acc := 0

	return acc
}

func Day10() {
	data := utils.ReadFile("tests/day10.txt")

	input := strings.Split(data, "\n")
	ret := day10_pt1(input)
	fmt.Println(ret)

	ret = day10_pt2(input)
	fmt.Println(ret)
}
