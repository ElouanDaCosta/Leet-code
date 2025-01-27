package main

import (
	"sort"
	"strings"
)

func findTheDifference(s string, t string) byte {
	s_split := strings.Split(s, "")
	t_split := strings.Split(t, "")
	sort.Strings(s_split)
	sort.Strings(t_split)

	for i := range s_split {
		if s_split[i] != t_split[i] {
			return t_split[i][0]
		}
	}
	return t_split[len(t_split)-1][0]
}
