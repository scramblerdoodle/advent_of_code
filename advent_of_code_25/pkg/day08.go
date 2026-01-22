package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"slices"
	"sort"
	"strconv"
	"strings"
)

var N = 1000

type coordAndDist struct {
	Coord1, Coord2 utils.Coordinate3D
	dist           float64
}

type circuit []utils.Coordinate3D

func circuitContains(circuits []circuit, c utils.Coordinate3D) int {
	for i, circuit := range circuits {
		contains := slices.Contains(circuit, c)
		if contains {
			return i
		}
	}
	return -1
}

func computeDistance(coord1, coord2 utils.Coordinate3D) float64 {
	// Doesn't have to be absolute value, it'll be squared later anyway
	// dist_x := math.Abs(float64(coord1.X - coord2.X))
	// dist_y := math.Abs(float64(coord1.Y - coord2.Y))
	// dist_z := math.Abs(float64(coord1.Z - coord2.Z))
	dist_x := float64(coord1.X - coord2.X)
	dist_y := float64(coord1.Y - coord2.Y)
	dist_z := float64(coord1.Z - coord2.Z)

	// Doesn't have to literally be the sqrt, it's a bijective function so it doesn't affect order
	// i.e. a,b>0; a > b <=> sqrt(a) > sqrt(b)
	// return math.Sqrt(dist_x*dist_x + dist_y*dist_y + dist_z*dist_z)
	return dist_x*dist_x + dist_y*dist_y + dist_z*dist_z
}

func coordinatesToDistances(coords []utils.Coordinate3D) []coordAndDist {
	// Compute all distances
	// TODO: In practice we don't need ALL distances so keep that in mind, would be a good memory complexity save
	coordinates_distances := []coordAndDist{}
	for i, c1 := range coords {
		for _, c2 := range coords[i+1:] {
			dist := computeDistance(c1, c2)

			coordinates_distances = append(coordinates_distances, coordAndDist{c1, c2, dist})
		}
	}

	// Sort by distance, decreasing
	sort.Slice(coordinates_distances, func(i, j int) bool {
		return coordinates_distances[i].dist < coordinates_distances[j].dist
	})

	return coordinates_distances

}

func day8_pt1(input []string) int {
	coordinates := []utils.Coordinate3D{}

	if len(input) == 20 {
		N = 10
	}
	for _, l := range input {
		values := strings.Split(l, ",")
		x, _ := strconv.Atoi(values[0])
		y, _ := strconv.Atoi(values[1])
		z, _ := strconv.Atoi(values[2])

		coordinates = append(coordinates, utils.Coordinate3D{X: x, Y: y, Z: z})
	}
	distances := coordinatesToDistances(coordinates)
	utils.DebugPrintln(distances[:N+5])

	circuits := []circuit{}
	connections_count := 0

	for _, c := range distances {
		c1 := c.Coord1
		c2 := c.Coord2

		if connections_count >= N {
			break
		}

		existing_index_c1 := circuitContains(circuits, c1)
		existing_index_c2 := circuitContains(circuits, c2)

		if existing_index_c1 == existing_index_c2 && existing_index_c1 != -1 {
			// Case 1: both are already present in the same circuit
			// Just skip them, don't add to connections_count
			utils.DebugPrintln(c1, "and", c2, "already in circuit")

		} else if existing_index_c1 != -1 && existing_index_c2 != -1 {
			// Case 2: both are already present but in different circuits
			// Merge both circuits together, add to connections_count
			first_index := min(existing_index_c1, existing_index_c2)
			second_index := max(existing_index_c1, existing_index_c2)

			utils.DebugPrintln(c1, "and", c2, "exist in different circuits:", "index", existing_index_c1, "and", "index", existing_index_c2)

			circuit_2, sub_circuits := utils.Pop(circuits, second_index)
			circuits = sub_circuits
			circuits[first_index] = append(circuits[first_index], circuit_2...)

		} else if existing_index_c1 != -1 {
			// Case 3: c1 exists in a circuit, c2 does not
			// Add c2 to c1's circuit, add to connections_count
			utils.DebugPrintln(c1, "present in circuit", existing_index_c1)
			circuits[existing_index_c1] = append(circuits[existing_index_c1], c2)

		} else if existing_index_c2 != -1 {
			// Case 4: c2 exists in a circuit, c1 does not
			// Add c1 to c2's circuit, add to connections_count
			utils.DebugPrintln(c2, "present in circuit", existing_index_c2)
			circuits[existing_index_c2] = append(circuits[existing_index_c2], c1)

		} else {
			// Case 5: neither c1 nor c2 exist in a circuit
			// Create new circuit containing both, add to connections_count
			utils.DebugPrintln(c1, "and", c2, "not present in circuit")
			circuits = append(circuits, circuit{c1, c2})
		}
		connections_count++

		utils.DebugPrintln("-----")
		utils.DebugPrintln("Circuits:", circuits)
		utils.DebugPrintln("Connections count:", connections_count)
		utils.DebugPrintln("-----")
	}

	// Get 3 largest circuits
	sort.Slice(circuits, func(i, j int) bool {
		return len(circuits[i]) > len(circuits[j])
	})

	acc := 1
	for _, circuit := range circuits[:3] {
		acc *= len(circuit)
	}

	return acc
}

func day8_pt2(input []string) int {
	coordinates := []utils.Coordinate3D{}

	if len(input) == 20 {
		N = 10
	}
	for _, l := range input {
		values := strings.Split(l, ",")
		x, _ := strconv.Atoi(values[0])
		y, _ := strconv.Atoi(values[1])
		z, _ := strconv.Atoi(values[2])

		coordinates = append(coordinates, utils.Coordinate3D{X: x, Y: y, Z: z})
	}
	distances := coordinatesToDistances(coordinates)

	circuits := []circuit{}
	connections_count := 0

	for _, c := range distances {
		c1 := c.Coord1
		c2 := c.Coord2

		utils.DebugPrintln("Evaluating coordinates", c)

		existing_index_c1 := circuitContains(circuits, c1)
		if existing_index_c1 != -1 {
			utils.DebugPrintln("c1", c1, "found at index", existing_index_c1)
		}
		existing_index_c2 := circuitContains(circuits, c2)
		if existing_index_c2 != -1 {
			utils.DebugPrintln("c2", c2, "found at index", existing_index_c2)
		}

		if existing_index_c1 == existing_index_c2 && existing_index_c1 != -1 {
			// Case 1: both are already present in the same circuit
			// Just skip them, don't add to connections_count
			utils.DebugPrintln(c1, "and", c2, "already in circuit", existing_index_c1)
			continue
		} else if existing_index_c1 != -1 && existing_index_c2 != -1 {
			// Case 2: both are already present but in different circuits
			// Merge both circuits together, add to connections_count
			first_index := min(existing_index_c1, existing_index_c2)
			second_index := max(existing_index_c1, existing_index_c2)
			utils.DebugPrintln(c1, "and", c2, "exist in different circuits:", "index", existing_index_c1, "and", "index", existing_index_c2)
			circuit_sub, circuits := utils.Pop(circuits, second_index)
			circuits[first_index] = append(circuits[first_index], circuit_sub...)

		} else if existing_index_c1 != -1 {
			// Case 3: c1 exists in a circuit, c2 does not
			// Add c2 to c1's circuit, add to connections_count
			utils.DebugPrintln(c1, "present in circuit", existing_index_c1)
			circuits[existing_index_c1] = append(circuits[existing_index_c1], c2)

		} else if existing_index_c2 != -1 {
			// Case 4: c2 exists in a circuit, c1 does not
			// Add c1 to c2's circuit, add to connections_count
			utils.DebugPrintln(c2, "present in circuit", existing_index_c2)
			circuits[existing_index_c2] = append(circuits[existing_index_c2], c1)

		} else {
			// Case 5: neither c1 nor c2 exist in a circuit
			// Create new circuit containing both, add to connections_count
			utils.DebugPrintln(c1, "and", c2, "not present in circuit")
			circuits = append(circuits, circuit{c1, c2})
		}
		connections_count++

		if connections_count == len(input)-1 {
			return c1.X * c2.X

		}
	}

	return -1
}

func Day08() {
	data := utils.ReadFile("tests/day08.txt")

	input := strings.Split(data, "\n")
	ret := day8_pt1(input)
	fmt.Println(ret)

	ret = day8_pt2(input)
	fmt.Println(ret)
}
