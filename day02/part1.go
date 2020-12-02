package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, _ := os.Open("input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	valid := 0

	for scanner.Scan() {
		text := scanner.Text()

		splitted := strings.Split(text, ": ")
		policy := splitted[0]
		password := splitted[1]

		splittedPolicy := strings.Split(policy, " ")
		amounts := splittedPolicy[0]
		char := splittedPolicy[1]

		splittedAmounts := strings.Split(amounts, "-")
		min, err := strconv.Atoi(splittedAmounts[0])

		if err != nil {
			fmt.Printf("FATAL ERR: %s (input:\"%s\")\n", err, text)
		}

		max, err := strconv.Atoi(splittedAmounts[1])

		if err != nil {
			fmt.Printf("FATAL ERR: %s (input:\"%s\")\n", err, text)
		}

		found := strings.Count(password, char)

		if found >= min && found <= max {
			valid++
		}

	}

	fmt.Printf("Valid passwords: %d\n", valid)
}
