package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
)

type Canvas struct {
	accessKey string
}

type Course struct {
	name string
	id   int
}

func main() {
	access_key := os.Getenv("CANVAS_ACCESS_KEY")
	canvas := Canvas{accessKey: access_key}
	courses := []Course{
		{"Physics", 33973},
		{"Discrete Structures", 33161},
		{"Architecture and Organization", 33114},
		{"Programming and Algorithms", 33148},
	}

	var assList AssignmentList

	if _, err := os.Stat("assignments.json"); errors.Is(err, os.ErrNotExist) {
		assList = canvas.SaveCourseAssignments(&courses)
		fmt.Println("did call")
	} else {
		jsonFile, err := os.Open("assignments.json")
		defer jsonFile.Close()
		if err != nil {
			panic(err)
		}

		bytes, err := io.ReadAll(jsonFile)
		if err != nil {
			panic(err)
		}

		json.Unmarshal(bytes, &assList)
	}
}
