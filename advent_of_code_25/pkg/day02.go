package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"iter"
	"strconv"
	"strings"
)

func day2_pt1(input string) int {
	// Input is 11-22,95-115, etc
	// split on comma to get each range, then split on dash, then for loop in that range (convert from string to int)
	// for each digit in that range, convert back to string for repeat analysis
	//
	// Always even number of digits. if odd #, skip
	// "palindrome"-esque type shi
	// 11 => 1 1
	// 111 => ok
	// 1111 => 11 11
	// 12341234 => 1234 1234
	//
	// idea: split digits in half, compare first half and second half. if equal, then it's invalid, add to sum
	count := 0
	// TODO: look up SplitSeq
	ranges := strings.Split(input, ",")
	for _, r := range ranges {
		limits := strings.Split(r, "-")

		// Convert range constraints to int
		start, _ := strconv.Atoi(limits[0])
		end, _ := strconv.Atoi(limits[1])

		for d := start; d <= end; d++ {

			// Convert back to string so we can separate it
			v := strconv.Itoa(d)

			if len(v)%2 != 0 {
				continue
			}

			n := len(v) / 2
			if v[:n] == v[n:] {
				utils.DebugPrintln(v, v[:n], v[n:])
				count += d
			}

		}

	}
	return count
}

func getDivisors(n int) iter.Seq[int] {
	return func(yield func(int) bool) {
		for d := 1; d <= n/2; d++ {
			if n%d == 0 {
				if !yield(d) {
					return
				}
			}
		}
	}
}

func day2_pt2(input string) int {
	count := 0
	for r := range strings.SplitSeq(input, ",") {
		limits := strings.Split(r, "-")

		// Convert range constraints to int
		start, _ := strconv.Atoi(limits[0])
		end, _ := strconv.Atoi(limits[1])

		for d := start; d <= end; d++ {

			// Convert back to string so we can separate it
			v := strconv.Itoa(d)

		SubstringComparison:
			// Get all possible substrings that might be repeated in the sequence (necessarily same length as divisors)
			for n := range getDivisors(len(v)) {
				source_seq := v[:n]

				// Compare source_seq to all other subsequences of length n
				for i := n; i < len(v); i += n {
					substring := v[i : i+n]
					// If any of the substrings don't match source, this seq does not repeat, so skip it
					if source_seq != substring {
						continue SubstringComparison
					}
				}

				// If we get to this point, it's a match and we count it
				count += d
				utils.DebugPrintln("Match", v, source_seq, v[n:])

				// Then move on to the next digit in the range
				break
			}
		}

	}
	return count
}

func Day02() {
	data := utils.ReadFile("tests/day02.txt")

	ret := day2_pt1(data)
	fmt.Println(ret)

	ret = day2_pt2(data)
	fmt.Println(ret)
}
