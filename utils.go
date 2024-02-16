package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"time"
)

type Canvas struct {
	accessKey string
}

type AssignmentList []Assignment

type Assignment struct {
	Name string `json:"name"`
	// Description string    `json:"description"`
	DueAt time.Time `json:"due_at"`
}

func (c Canvas) courseEndpoint(courseId string) (string, string) {
	endpoint := fmt.Sprintf("/api/v1/courses/%s/assignments", courseId)
	baseUrl, err := url.Parse("https://canvas.butte.edu/")

	if err != nil {
		_ = fmt.Errorf("Something's wrong with canvas url")
	}

	epUrl, err := baseUrl.Parse(endpoint)

	if err != nil {
		_ = fmt.Errorf("Something's wrong with endpoint received or the course id")
	}

	params := epUrl.Query()
	params.Set("bucket", "upcoming")
	params.Set("access_token", c.accessKey)

	epUrl.RawQuery = params.Encode()

	upcoming := epUrl.String()

	params.Set("bucket", "future")
	epUrl.RawQuery = params.Encode()

	future := epUrl.String()

	return upcoming, future
}

func (c Canvas) parseAssignments(endpoint string) AssignmentList {
	resp, err := http.Get(endpoint)
	if err != nil {
		panic(err)
	}

	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}

	var assList AssignmentList
	json.Unmarshal(body, &assList)
	return assList
}

func (c Canvas) CourseAssignments(courseId string) {
	upcoming, _ := c.courseEndpoint(courseId)
	assList := c.parseAssignments(upcoming)

	fmt.Println(assList)

}
