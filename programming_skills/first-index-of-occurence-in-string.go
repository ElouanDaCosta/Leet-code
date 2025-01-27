// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/?envType=study-plan-v2&envId=programming-skills
package main

import (
	"regexp"
)

func strStr(haystack string, needle string) int {
	regex := regexp.MustCompile(needle)
	match := regex.FindAllStringIndex(haystack, -1)
	if match != nil {
		return match[0][0]
	}
	return -1
}
