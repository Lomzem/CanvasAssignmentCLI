package main

import (
	"os"
)

func main() {
	access_key := os.Getenv("CANVAS_ACCESS_KEY")
	canvas := Canvas{accessKey: access_key}
	canvas.CourseAssignments("33973")

	// fmt.Println(courseEndpoint)
	//
	// resp, err := http.Get(courseEndpoint)
	// if err != nil {
	// 	panic(err)
	// }
	//
	// defer resp.Body.Close()
	// body, err := io.ReadAll(resp.Body)
	// if err != nil {
	// 	panic(err)
	// }

	// fmt.Println(body)
	// os.WriteFile("./course.json", body, 0644)
}
