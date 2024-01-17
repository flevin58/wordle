# wordle

This is yet another wordle game, written in go.
Although it is a perferctly playable game, the main purpose I wrote it is for
me to learn ***ebitengine***

## Installation

***wordle*** is a self-contained executable. Copy it to a location defined in yout PATH.

## Words dictionary

The words used by this game are embedded and they were scraped from the following site:
https://www.wordunscrambler.net/word-list/wordle-word-list

If interested, you can find the source code of the scraping project at the following github repository: flevin58/wordle-scrape


## Project Tree Structure

```
.
├── LICENSE
├── README.md
├── game
│   ├── assets
│   │   ├── fonts
│   │   ├── wordle.yaml
│   │   └── words.txt
│   ├── config.go
│   ├── game.go
│   ├── input.go
│   └── words.go
├── go.mod
├── go.sum
└── main.go
```
