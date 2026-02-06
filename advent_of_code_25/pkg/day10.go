package pkg

import (
	"advent_of_code_25/utils"
	"errors"
	"fmt"
	"regexp"
	"slices"
	"strconv"
	"strings"

	"codeberg.org/Gusted/algorithms-go/math/row-reduction"
	"github.com/mxschmitt/golang-combinations"
)

const MAX = 1000

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

type joltages []int

func initJoltage(s int) joltages {
	// Initializes new joltage with default value `0`
	new_joltage := []int{}
	for range s {
		new_joltage = append(new_joltage, 0)
	}
	return new_joltage
}

func (js joltages) getMinJoltage() int {
	min_joltage := MAX
	min_joltage_index := -1
	for i, j := range js {
		if j < min_joltage && j != 0 {
			min_joltage = j
			min_joltage_index = i
		}
	}
	return min_joltage_index
}

func (js joltages) getZeroedJoltages() []int {
	zeroed := []int{}

	for i, j := range js {
		if j == 0 {
			zeroed = append(zeroed, i)
		}
	}
	return zeroed
}

type buttons [][]int

func isValidBaseForJoltage(bs buttons, n int) bool {
	base := []bool{}
	for range n {
		base = append(base, false)
	}

	for _, b := range bs {
		for _, i := range b {
			base[i] = true
		}
	}

	for _, elem := range base {
		if !elem {
			return false
		}
	}
	return true
}

func removeButtons(bs buttons, to_remove []int) buttons {
	new_buttons := [][]int{}
ButtonLoop:
	for _, b := range bs {
		for _, b_i := range to_remove {
			if slices.Contains(b, b_i) {
				continue ButtonLoop
			}
		}
		new_buttons = append(new_buttons, b)
	}
	return new_buttons
}

func popLongestSequenceForJoltageIndex(bs *[][]int, n int) ([]int, error) {
	max_length := 0
	max_index := -1
	max_button_seq := []int{}
	for i, b := range *bs {
		if len(b) > max_length && slices.Contains(b, n) {
			max_length = len(b)
			max_index = i
			max_button_seq = b
		}
	}

	if max_index == -1 {
		err_msg := fmt.Sprintf("Target joltage %d not found in buttons %v", n, *bs)
		return []int{}, errors.New(err_msg)
	}

	*bs = append((*bs)[:max_index], (*bs)[max_index+1:]...)

	return max_button_seq, nil
}

func (js *joltages) updateJoltage(button_seq []int, n int) {
	for _, i := range button_seq {
		(*js)[i] += n
	}
}

func (js1 joltages) compareJoltages(js2 joltages) bool {
	for i := range js1 {
		if js1[i] != js2[i] {
			return false
		}
	}
	return true
}

func convert_input(input string) (indicatorLights, buttons, joltages) {
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

	// Match {}
	joltage := []int{}
	re_joltages := regexp.MustCompile(`\{.+\}`)
	joltages_raw := re_joltages.Find([]byte(input))
	// Ignore first and last elements (brackets), split on `,`, convert to int
	for j := range strings.SplitSeq(string(joltages_raw[1:len(joltages_raw)-1]), ",") {
		j_i, _ := strconv.Atoi(j)
		joltage = append(joltage, j_i)
	}

	return indicator, buttons, joltage
}

func day10_pt1(input []string) int {
	acc := 0
	for _, l := range input {
		// Convert line into expected structure
		target_indicator, existing_buttons, _ := convert_input(l)
		utils.DebugPrintln(target_indicator, existing_buttons)

		// Get all possible combinations
		// TODO: may be overkill, could start with lowest combinations.Combinations(k), k:=1..n
		// 			 if any of the lowest ones match already, break the loop
		buttons_combos := combinations.All(existing_buttons)

		min_button_presses := MAX
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

func convertButtonsToLinearSystem(button_combo [][]int, size int) *utils.Grid[int] {
	// Convert button combo into a matrix for the linear system
	A := [][]int{}
	for _, b := range button_combo {
		// Init vec as len(target_joltage) all zeroes
		vec := []int{}
		for range size {
			vec = append(vec, 0)
		}
		for _, i := range b {
			vec[i] = 1
		}
		A = append(A, vec)
	}

	grid_A := utils.NewGridFromValues(A)

	// Transpose A
	A_t := grid_A.TransposeGrid()

	return A_t
}

func solveLinearSystem(linear_system utils.Grid[int], target_joltage joltages) (res [][]int, err error) {
	// Convert to linear equations
	equations := make([][]float64, len(target_joltage))
	for i, vec := range linear_system.Grid {
		eq := make([]float64, len(vec)+1)
		for j, v := range vec {
			eq[j] = float64(v)
		}

		eq[len(eq)-1] = float64(target_joltage[i])
		equations[i] = eq
	}

	defer func() {
		if r := recover(); r != nil {
			// Convert the panic value to an error
			switch v := r.(type) {
			case error:
				err = v
			default:
				err = fmt.Errorf("gaussian solver panicked: %v", v)
			}
		}
	}()
	utils.DebugPrintln("Attempting to solve for equations", equations)
	// WARNING: This is a Diophantine Equation, Gaussian Elimination will not necessarily work here
	// TODO: Implement a Diophantine equation solver
	res_f64 := rowreduction.GaussianElimination(equations)
	_, err = solutionSanityCheck(res_f64)
	if err != nil {
		return [][]int{}, err
	}

	result := make([][]int, len(target_joltage))
	for i, vec := range res_f64 {
		eq := make([]int, len(vec))
		for j, v := range vec {
			eq[j] = int(v)
		}

		result[i] = eq
	}

	return result, nil
}

func solutionSanityCheck(res [][]float64) (bool, error) {
	defer func() {
		if r := recover(); r != nil {
		}
	}()
	for _, r := range res {
		for _, v := range r {
			if !utils.IsNatural(v) {
				return false, errors.New("Non-natural solution")
			}
		}
	}

	return true, nil
}

func backSubstitution(mat [][]int) (res []int, err error) {
	N := len(mat)
	res = make([]int, len(mat))
	for i := N - 1; i >= 0; i-- {
		res[i] = mat[i][N]

		for j := i + 1; j < N; j++ {
			res[i] -= mat[i][j] * res[j]
		}
		// res[i] = res[i] / mat[i][i]
		if res[i] < 0 {
			return []int{}, errors.New("Non-positive solution")
		}
	}
	return res, nil

}

func day10_pt2(input []string) (int, error) {
	acc := 0
	for _, l := range input {
		// Convert line into expected structure
		_, existing_buttons, target_joltage := convert_input(l)
		utils.DebugPrintln(existing_buttons, target_joltage)

		buttons_combos := combinations.All(existing_buttons)

		min_button_presses := MAX

		for _, b_c := range buttons_combos {
			utils.DebugPrintln("Checking combo", b_c)
			linear_system := convertButtonsToLinearSystem(b_c, len(target_joltage))
			mat, err := solveLinearSystem(*linear_system, target_joltage)
			if err != nil {
				utils.DebugPrintln("Failed to reduce linear system w/ Gaussian Elimination, error:", err)
				continue
			}

			res, err := backSubstitution(mat)
			if err != nil {
				utils.DebugPrintln("Failed to solve linear system w/ back substitution, error:", err)
				continue
			}

			utils.DebugPrintln("Result:", res)

			button_presses := 0
			for _, v := range res {
				button_presses += v
			}

			if min_button_presses > button_presses && button_presses != 0 {
				min_button_presses = button_presses
				utils.DebugPrintln("New min found:", min_button_presses)

			}

		}

		if min_button_presses == MAX {
			utils.DebugPrintln("No solution found for", existing_buttons, target_joltage)
			return 0, errors.New("No solution found for one of the inputs")
		}
		acc += min_button_presses
	}

	return acc, nil
}

func Day10() {
	data := utils.ReadFile("tests/day10.txt")

	input := strings.Split(data, "\n")
	ret := day10_pt1(input)
	fmt.Println(ret)

	ret, err := day10_pt2(input)
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Println(ret)
	}
	// Ret: 138277, too high

}
