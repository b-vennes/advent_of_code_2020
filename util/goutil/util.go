package goutil

import (
	"errors"
	"io/ioutil"
)

func GetInputPathFromArgs(args []string) (string, error) {
	if len(args) < 2 {
		return "", errors.New("No input path specified")
	} else {
		return args[1], nil
	}
}

func ParseFilePathToStringSlice(path string, parser func(string) ([]string, error)) ([]string, error) {
	text, err := ioutil.ReadFile(path)

	if err != nil {
		return []string{}, err
	}

	return parser(string(text))
}
