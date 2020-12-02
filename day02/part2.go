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
		pos1, err := strconv.Atoi(splittedAmounts[0])

		if err != nil {
			fmt.Printf("FATAL ERR: %s (input:\"%s\")\n", err, text)
		}

		pos2, err := strconv.Atoi(splittedAmounts[1])

		if err != nil {
			fmt.Printf("FATAL ERR: %s (input:\"%s\")\n", err, text)
		}

		length := len(password)

		if pos1 > length || pos2 > length {
			fmt.Printf("Skipping \"%s\", too long\n", text)
			continue
		}

		charAtPos1 := password[pos1 - 1]
		charAtPos2 := password[pos2 - 1]

		validPos1 := string(charAtPos1) == char
		validPos2 := string(charAtPos2) == char

		if (validPos1 && !validPos2) || (!validPos1 && validPos2)  {
			valid++
		}
	}

	fmt.Printf("Valid passwords: %d\n", valid)
}
