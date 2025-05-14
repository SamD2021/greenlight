package main

import "github.com/extism/go-pdk"

type Output struct {
	Status string `json:"status"`
	Reason string `json:"reason,omitempty"`
}

type Input struct {
	Status string `json:"status"`
	Reason string `json:"reason"`
}

//go:export check
func Check() int32 {
	result := Output{Status: "success"}
	pdk.OutputJSON(result)
	return 0
}

func main() {}
