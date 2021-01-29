package main

import (
	"errors"
	"fmt"
	"log"
	"os"
	"sort"
	"strings"

	"../util/goutil"
)

func parseToStringSlice(text string) ([]string, error) {
	result := strings.Split(text, "\r\n")

	return result, nil
}

type boardingPass struct {
	rowPath []bool
	colPath []bool
}

type seat struct {
	row int
	col int
}

func (s seat) GetId() int {
	return (s.row * 8) + s.col
}

func followPath(path []bool, lowerBound int, upperBound int) int {
	if len(path) == 0 {
		return lowerBound
	}

	if path[0] {
		return followPath(path[1:], lowerBound, lowerBound+((upperBound-lowerBound)/2))
	} else {
		return followPath(path[1:], upperBound-((upperBound-lowerBound)/2), upperBound)
	}
}

func (pass boardingPass) getRow() int {
	return followPath(pass.rowPath, 0, 127)
}

func (pass boardingPass) getCol() int {
	return followPath(pass.colPath, 0, 7)
}

func (pass boardingPass) GetSeat() seat {
	return seat{pass.getRow(), pass.getCol()}
}

func parseToBoardingPass(value string) (boardingPass, error) {
	runes := []rune(value)

	if len(runes) != 10 {
		return boardingPass{}, errors.New("String must be length 10 to be a boarding pass")
	}

	row := []bool{}

	for i, v := range runes[:7] {
		if v == 'F' {
			row = append(row, true)
		} else if v == 'B' {
			row = append(row, false)
		} else {
			return boardingPass{}, errors.New(fmt.Sprintf("Invalid character at index %d", i))
		}
	}

	col := []bool{}

	for i, v := range runes[7:10] {
		if v == 'L' {
			col = append(col, true)
		} else if v == 'R' {
			col = append(col, false)
		} else {
			return boardingPass{}, errors.New(fmt.Sprintf("Invalid character at index %d", i))
		}
	}

	return boardingPass{row, col}, nil
}

func parseIntoBoardingPasses(values []string) ([]boardingPass, error) {
	passes := []boardingPass{}

	for _, v := range values {
		pass, err := parseToBoardingPass(v)

		if err != nil {
			return []boardingPass{}, err
		}

		passes = append(passes, pass)
	}

	return passes, nil
}

func main() {
	inputPath, err := goutil.GetInputPathFromArgs(os.Args)

	if err != nil {
		log.Fatalln(err.Error())
	}

	inputValues, err := goutil.ParseFilePathToStringSlice(inputPath, parseToStringSlice)

	if err != nil {
		log.Fatalln(err.Error())
	}

	passes, err := parseIntoBoardingPasses(inputValues)

	if err != nil {
		log.Fatalln(err.Error())
	}

	ids := []int{}

	for _, pass := range passes {
		ids = append(ids, pass.GetSeat().GetId())
	}

	sort.Sort(sort.IntSlice(ids))

	var mySeat int

	for i, id := range ids[1 : len(ids)-1] {
		if ids[i+2] != id+1 {
			mySeat = id + 1
			break
		}
	}

	log.Println(mySeat)
}
