// link to the exercice https://leetcode.com/problems/sort-array-by-parity/description/

package main

func sortArrayByParity(nums []int) []int {
	var outpout []int
	for i := range nums {
		evenOrOdd := nums[i] % 2
		if evenOrOdd == 0 {
			outpout = append([]int{nums[i]}, outpout...)
		} else {
			outpout = append(outpout, nums[i])
		}
	}
	return outpout
}

// best solution found

func sortArrayByParitySolution(nums []int) []int {
	i, j := 0, len(nums)-1

	for i < j {
		for i < j && nums[i]%2 == 0 {
			i++
		}
		for i < j && nums[j]%2 == 1 {
			j--
		}

		nums[i], nums[j] = nums[j], nums[i]
	}

	return nums
}
