// Link to the exercice https://leetcode.com/problems/roman-to-integer/description/

package main

import (
	"strings"
)

func romanToInt(s string) int {
	sSplit := strings.Split(s, "")
	var outpout = 0
	// var myArray []string
	myMap := make(map[int]string)
	for index, value := range sSplit {
		myMap[index] = value
		switch myMap[len(myMap)-1] {
		case "I":
			outpout += 1
		case "V":
			if myMap[index-1] == "I" {
				outpout += 3
			} else {
				outpout += 5
			}
		case "X":
			if myMap[index-1] == "I" {
				outpout += 8
			} else {
				outpout += 10
			}
		case "L":
			if myMap[index-1] == "X" {
				outpout += 40
			} else {
				outpout += 50
			}
		case "C":
			if myMap[index-1] == "X" {
				outpout += 80
			} else {
				outpout += 100
			}
		case "D":
			if myMap[index-1] == "C" {
				outpout += 300
			} else {
				outpout += 500
			}
		case "M":
			if myMap[index-1] == "C" {
				outpout += 800
			} else {
				outpout += 1000
			}
		}
	}
	return outpout
}

// Best solutions found

func romanToIntSolution(s string) int {
	var v, lv, cv int
	h := map[uint8]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}

	for i := len(s) - 1; i >= 0; i-- {
		cv = h[s[i]]
		if cv < lv {
			v -= cv
		} else {
			v += cv
		}
		lv = cv
	}

	return v
}
