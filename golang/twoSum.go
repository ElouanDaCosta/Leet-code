package main

import (
	"fmt"
)

func twoSum(nums []int, target int) []int {
	sumArray := 0
	var outpout []int
	index := 0
	for i := 0; i <= len(nums); i++ {
		sumArray = nums[i] + nums[i+1]
		if sumArray == target {
			fmt.Println(nums[i], nums[i+1])
			index = i
			outpout = append(outpout, index)
			index = i + 1
			outpout = append(outpout, index)
			return outpout
		} else {
			sumArray = 0
		}
	}
	return outpout
}
