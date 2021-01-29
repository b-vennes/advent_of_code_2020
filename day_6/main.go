package main

import (
	"log"
	"os"
	"strings"

	"../util/goutil"
)

func parseInput(input string) ([]string, error) {
	return strings.Split(input, "\r\n\r\n"), nil
}

func main() {
	path, err := goutil.GetInputPathFromArgs(os.Args)

	if err != nil {
		log.Fatalln(err.Error())
	}

	groupValues, err := goutil.ParseFilePathToStringSlice(path, parseInput)

	if err != nil {
		log.Fatalln(err.Error())
	}

	groups := []groupAnswers{}

	for _, groupValue := range groupValues {
		groups = append(groups, createGroupAnswersFrom(strings.Split(groupValue, "\r\n")))
	}

	sumUnique := 0
	sumUnanimous := 0

	for _, g := range groups {
		sumUnique += g.getNumberOfUniqueAnswers()
	}

	for _, g := range groups {
		sumUnanimous += g.getNumberOfUnanimousAnswers()
	}

	log.Println(sumUnique)
	log.Println(sumUnanimous)
}
