package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"sync"
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
		panic("Something's wrong with canvas url")
	}

	epUrl, err := baseUrl.Parse(endpoint)

	if err != nil {
		panic("Something's wrong with endpoint received or the course id")
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

func (c Canvas) parseAssignments(endpoint *string, ch chan<- AssignmentList, wg *sync.WaitGroup) {
	defer wg.Done()

	resp, err := http.Get(*endpoint)
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
	ch <- assList
}

func (c Canvas) CourseAssignments(courseId string) {
	upcoming, future := c.courseEndpoint(courseId)

	ch := make(chan AssignmentList)
	var wg sync.WaitGroup
	var assList AssignmentList

	wg.Add(1)
	go c.parseAssignments(&upcoming, ch, &wg)

	wg.Add(1)
	go c.parseAssignments(&future, ch, &wg)

	go func() {
		wg.Wait()
		close(ch)
	}()

	for result := range ch {
		assList = append(assList, result...)
	}

	fmt.Println(assList)

}
