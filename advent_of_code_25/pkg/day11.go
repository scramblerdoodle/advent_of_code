package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	// "strconv"
	"strings"
)

func day11_pt1(input []string) int {
	graph := make(map[string][]string)
	// WARNING: Assuming it's a DAG, don't need to keep track of visited nodes
	// visited := make(map[string]bool)
	for _, l := range input {
		nodes := strings.Split(l, " ")
		source := nodes[0][:len(nodes[0])-1] // Removing ":" from source node
		targets := nodes[1:]

		graph[source] = targets
		// visited[source] = false
	}

	q := utils.Queue[string]{Items: graph["you"]}
	utils.DebugPrintln(q)

	acc := 0
	for node, ok := q.Dequeue(); ok; node, ok = q.Dequeue() {
		utils.DebugPrintln("Visiting node", node)
		// If out node, add to acc and continue
		if node == "out" {
			acc++
			continue
		}

		// Skip already visited node, avoid cycles
		// if visited[node] {
		// 	continue
		// }
		// visited[node] = true

		for _, n := range graph[node] {
			q.Enqueue(n)
		}
		utils.DebugPrintln("Queue", q)
	}

	return acc
}

func day11_pt2(input []string) int {
	acc := 0

	return acc
}

func Day11() {
	data := utils.ReadFile("tests/day11.txt")

	input := strings.Split(data, "\n")
	ret := day11_pt1(input)
	fmt.Println(ret)

	ret = day11_pt2(input)
	fmt.Println(ret)
}
