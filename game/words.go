package game

import (
	_ "embed"
	"math/rand"
	"strings"
)

//go:embed assets/words.txt
var wordFile string

func RuneArrayToString(a [nCols]rune) string {
	var s strings.Builder
	for _, ch := range a {
		s.WriteRune(ch)
	}

	return s.String()
}

// TODO: Implement func g.isWinner() bool  -> compares
func StringToRuneArray(s string) [nCols]rune {
	var result [nCols]rune
	for i := 0; i < nCols; i++ {
		result[i] = rune(s[i])
	}
	return result
}

func getRandomWord() [nCols]rune {
	words := strings.Split(wordFile, "\n")
	index := rand.Intn(len(words))
	word := strings.ToUpper(words[index])
	return StringToRuneArray(word)
}
