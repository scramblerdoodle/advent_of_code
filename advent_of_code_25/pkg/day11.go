package pkg

import (
	"advent_of_code_25/utils"
	"fmt"
	"maps"
	"strings"
)

func day11_pt1(input []string) int {
	graph := make(map[string][]string)
	for _, l := range input {
		nodes := strings.Split(l, " ")
		source := nodes[0][:len(nodes[0])-1] // Removing ":" from source node
		targets := nodes[1:]

		graph[source] = targets
	}

	q := utils.Queue[string]{Items: graph["you"]}

	acc := 0
	// BFS
	// WARNING: if there are ANY loops in the graph, this will go into an infinite loop
	// so we're assuming that the source graph is a DAG
	for node, ok := q.Dequeue(); ok; node, ok = q.Dequeue() {
		// utils.DebugPrintln("Visiting node", node)
		// If out node, add to acc and continue
		if node == "out" {
			acc++
			continue
		}

		for _, n := range graph[node] {
			q.Enqueue(n)
		}
	}

	return acc
}

func countPathsFromTarget(graph map[string][]string, target string) (ret map[string]int) {
	counts := make(map[string]int)
	new_counts := make(map[string]int)

	for node := range graph {
		new_counts[node] = 0
	}
	new_counts[target] = 1

	for !maps.Equal(counts, new_counts) {
		maps.Copy(counts, new_counts)

		for node := range counts {
			if node == target {
				new_counts[node] = 1
			} else {
				sum := 0
				for _, child := range graph[node] {
					sum += counts[child]
				}
				new_counts[node] = sum
			}
		}
	}

	return new_counts
}

func day11_pt2(input []string) int {
	graph := make(map[string][]string)
	visited := map[string]bool{}
	for _, l := range input {
		nodes := strings.Split(l, " ")
		source := nodes[0][:len(nodes[0])-1] // Removing ":" from source node
		targets := nodes[1:]

		graph[source] = targets
		visited[source] = false
	}
	acc := countPathsFromTarget(graph, "fft")["svr"] * countPathsFromTarget(graph, "dac")["fft"] * countPathsFromTarget(graph, "out")["dac"]
	acc += countPathsFromTarget(graph, "dac")["svr"] * countPathsFromTarget(graph, "fft")["dac"] * countPathsFromTarget(graph, "out")["fft"]
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
