package main

import "strings"

func numJewelsInStones(jewels string, stones string) int {
	stoneJewelCount := 0

	for _, jewel := range strings.Split(jewels, "") {
		stoneJewelCount += strings.Count(stones, jewel)
	}

	return stoneJewelCount
}
