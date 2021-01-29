package main

type individualAnswers struct {
	answers []rune
}

type groupAnswers struct {
	individuals []individualAnswers
}

func (g groupAnswers) getNumberOfUniqueAnswers() int {
	uniques := []rune{}

	for _, individual := range g.individuals {
		for _, answer := range individual.answers {
			if !contains(uniques, answer) {
				uniques = append(uniques, answer)
			}
		}
	}

	return len(uniques)
}

func (g groupAnswers) getNumberOfUnanimousAnswers() int {
	unanimous := []rune{}

	for i, individual := range g.individuals {
		for _, answer := range individual.answers {
			toCheck := []individualAnswers{}

			for j, other := range g.individuals {
				if i != j {
					toCheck = append(toCheck, other)
				}
			}

			if countWithSameAnswer(toCheck, answer) == len(g.individuals)-1 && !contains(unanimous, answer) {
				unanimous = append(unanimous, answer)
			}
		}
	}

	return len(unanimous)
}

func (individual individualAnswers) answeredSame(answer rune) bool {
	return contains(individual.answers, answer)
}

func createGroupAnswersFrom(values []string) groupAnswers {
	individuals := []individualAnswers{}

	for _, value := range values {
		answers := []rune{}
		for _, letter := range value {
			answers = append(answers, letter)
		}

		individuals = append(individuals, individualAnswers{answers})
	}

	return groupAnswers{individuals}
}

func contains(answers []rune, value rune) bool {
	for _, a := range answers {
		if a == value {
			return true
		}
	}

	return false
}

func countWithSameAnswer(indiduals []individualAnswers, answer rune) int {
	count := 0

	for _, i := range indiduals {
		if i.answeredSame(answer) {
			count++
		}
	}

	return count
}
