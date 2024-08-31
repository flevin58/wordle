package game

import (
	_ "embed"
	"fmt"
	"image/color"
	"io"
	"log"
	"os"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/text"
	"golang.org/x/image/font"
)

type GameStatus byte

const (
	Playing GameStatus = iota
	Won
	Lost
)

type Game struct {
	fontWordle  font.Face
	fontTitle   font.Face
	fontMessage font.Face
	fontLetter  font.Face
	grid        [nRows][nCols]rune
	answer      [nCols]rune
	row         int
	col         int
	status      GameStatus
	ticks       uint64
	blinksecs   int
}

func New() Game {

	game := Game{
		fontWordle:  getFontFace(wconf.Font.Wordle),
		fontTitle:   getFontFace(wconf.Font.Title),
		fontMessage: getFontFace(wconf.Font.Message),
		fontLetter:  getFontFace(wconf.Font.Letter),
		blinksecs:   3,
	}

	game.ResetValues()

	ebiten.SetWindowSize(wconf.Screen.Width, wconf.Screen.Height)
	ebiten.SetWindowTitle(fmt.Sprintf("%s - %s", wconf.Name, wconf.Version))

	return game
}

// Reset values to start a new game.
// Used at creation or to restart a new game
func (g *Game) ResetValues() {
	g.answer = getRandomWord()
	g.status = Playing
	g.row = 0
	g.col = 0
	for row := 0; row < nRows; row++ {
		for col := 0; col < nCols; col++ {
			g.grid[row][col] = 0
		}
	}
}

// Returns the foreground anf background colors for a grid square
func (g *Game) getColor(row, col int) (color.Color, color.Color) {

	// All lines below and including current have
	// black font on lightgrey background
	if row >= g.row {
		return color.Black, wconf.Colors.Emptybox.toRGBA()
	}

	// Check proper color
	gridRune := g.grid[row][col]
	for i, char := range g.answer {
		if i == col && char == gridRune {
			return color.White, wconf.Colors.Rightpos.toRGBA()
		}
		if char == gridRune {
			return color.White, wconf.Colors.Wrongpos.toRGBA()
		}
	}

	// No chars found
	return color.White, wconf.Colors.Noletters.toRGBA()
}

// Layout is one of the methods required by the ebiten interface
// In our case we always return the window dimensions
func (g *Game) Layout(outsideWidth, outsideHeight int) (int, int) {
	return wconf.Screen.Width, wconf.Screen.Height
}

func (g *Game) PrintStatus(screen *ebiten.Image, message Message) {

	var x, y int
	tw, _ := font.BoundString(g.fontTitle, message.Title)
	x = (wconf.Screen.Width - tw.Max.X.Round()) / 2
	y = wconf.Screen.Height - wconf.Geometry.Statush + int(wconf.Font.Title.Size)
	text.Draw(screen, message.Title, g.fontTitle, x, y, color.Black)

	mw, _ := font.BoundString(g.fontMessage, message.Text)
	x = (wconf.Screen.Width - mw.Max.X.Round()) / 2
	y += int(wconf.Font.Title.Size)
	text.Draw(screen, message.Text, g.fontMessage, x, y, color.Black)
}

func xBoxCoordAtCol(col int) int {
	return wconf.Geometry.Boxsp + col*(wconf.Geometry.Boxsp+wconf.Geometry.Boxsz)
}

func yBoxCoordAtRow(row int) int {
	return wconf.Geometry.Titleh + wconf.Geometry.Boxsp + row*(wconf.Geometry.Boxsp+wconf.Geometry.Boxsz)
}

func DrawRect(screen *ebiten.Image, x, y, w, h int, c color.Color) {
	rect := ebiten.NewImage(w, h)
	rect.Fill(c)
	op := &ebiten.DrawImageOptions{}
	op.GeoM.Translate(float64(x), float64(y))
	screen.DrawImage(rect, op)
}

func CenterTextAt(screen *ebiten.Image, f font.Face, msg string, x, y, w, h int, c color.Color) {
	tw, _ := font.BoundString(f, msg)
	xText := (w - tw.Max.X.Round()) / 2
	yText := h - int((wconf.Font.Wordle.Size / 2))
	text.Draw(screen, msg, f, int(xText), int(yText), c)
}

// Draw is one of the the methods required by the ebiten interface
// Is being called once every frame (1/60 second) and draws the frame
func (g *Game) Draw(screen *ebiten.Image) {

	screen.Fill(color.White)

	// Draw the title area and title
	DrawRect(screen, 0, 0, wconf.Screen.Width, wconf.Geometry.Titleh, wconf.Colors.Titlearea.toRGBA())
	wtitle := fmt.Sprintf("%s %s", wconf.Name, wconf.Version)
	CenterTextAt(screen, g.fontWordle, wtitle, 0, 0, wconf.Screen.Width, wconf.Geometry.Titleh, color.Black)

	// Draw the grid area
	yGrid := wconf.Geometry.Titleh
	hGrid := (nRows+1)*wconf.Geometry.Boxsp + nRows*wconf.Geometry.Boxsz
	DrawRect(screen, 0, yGrid, wconf.Screen.Width, hGrid, wconf.Colors.Gridarea.toRGBA())

	// Draw current position cursor outline
	if g.status == Playing && g.col < nCols {
		y := wconf.Geometry.Titleh + g.row*(wconf.Geometry.Boxsz+wconf.Geometry.Boxsp)
		x := g.col * (wconf.Geometry.Boxsz + wconf.Geometry.Boxsp)
		sz := wconf.Geometry.Boxsz + 2*wconf.Geometry.Boxsp
		DrawRect(screen, x, y, sz, sz, wconf.Colors.Outline.toRGBA())
	}

	// Draw grid
	var xCoord, yCoord int
	for row := 0; row < nRows; row++ {
		yCoord = yBoxCoordAtRow(row)
		for col := 0; col < nCols; col++ {
			xCoord = xBoxCoordAtCol(col)
			fontColor, boxColor := g.getColor(row, col)
			// Special case if we have lost
			var char string
			delta := uint64(120 * g.blinksecs)
			if g.status == Lost && row == nRows-1 && g.ticks%delta < delta/2 {
				char = string(g.answer[col])
				fontColor = color.White
				boxColor = wconf.Colors.Answer.toRGBA()
			} else {
				char = string(g.grid[row][col])
			}

			DrawRect(screen, int(xCoord), int(yCoord), wconf.Geometry.Boxsz, wconf.Geometry.Boxsz, boxColor)

			// Draw the letter inside
			if g.grid[row][col] == 0 {
				continue
			}

			// Draw the letter in the box
			xLetter := xCoord + int(wconf.Font.Letter.Size/2)
			yLetter := yCoord + int(wconf.Font.Letter.Size)
			text.Draw(screen, char, g.fontLetter, int(xLetter), int(yLetter), fontColor)
		}
	}

	// Draw the status area
	yStatus := wconf.Screen.Height - wconf.Geometry.Statush
	DrawRect(screen, 0, yStatus, wconf.Screen.Width, wconf.Geometry.Statush, wconf.Colors.Statusarea.toRGBA())

	// Draw status info
	switch g.status {
	case Won:
		g.PrintStatus(screen, wconf.Messages.Win)
	case Lost:
		g.PrintStatus(screen, wconf.Messages.Loose)
	default:
		if g.col < nCols {
			g.PrintStatus(screen, wconf.Messages.Playing)
		} else {
			g.PrintStatus(screen, wconf.Messages.Enter)
		}
	}
}

// Draw is one of the the methods required by the ebiten interface
// Is being called once every frame (1/60 second) and updates game variables
func (g *Game) Update() error {

	// Update timer
	g.ticks++

	key := getPressedKey()
	if key == "" {
		log.SetOutput(io.Discard)
		return nil
	}

	log.SetOutput(os.Stdout)
	//log.Println()
	//log.Printf("Pressed:<%s>\n", key)
	switch {
	case key == "Escape":
		if g.status != Playing {
			os.Exit(0)
		}
	case key == "Backspace":
		if g.status != Playing {
			return nil
		}

		if g.col < nCols {
			g.grid[g.row][g.col] = 0
		}

		// Move left
		if g.col > 0 {
			g.col--
		}
		g.grid[g.row][g.col] = 0

	case key == "Enter":
		// If Enter is pressed at the end of the game, restart!
		if g.status != Playing {
			g.ResetValues()
			return nil
		}

		// When playing, you can press Enter only at the end of a line!
		if g.col != nCols {
			return nil
		}

		// Go down
		if g.row < nRows {
			g.row++
			g.col = 0
		}

		// Check if previous line is a winner!
		if g.IsWinner() {
			g.status = Won
		}

		// Here not a winner. If at the end of the board we lose!
		if g.row == nRows {
			g.status = Lost
		}

	case len(key) == 1:
		if g.status != Playing {
			return nil
		}

		if key < "A" || key > "Z" {
			return nil
		}

		// Cannot press a key if after last char
		if g.col == nCols {
			return nil
		}

		// Set the rune
		g.grid[g.row][g.col] = rune(key[0])

		// Move right
		if g.col < nCols {
			g.col++
		}
	}

	return nil
}

// Run wraps the ebiten RunGame method
// This way the main module does not need to include ebiten
func (g *Game) Run() {
	if err := ebiten.RunGame(g); err != nil {
		log.Fatal(err)
	}
}

// Checks if the line above the current one is equal to the answer
func (g *Game) IsWinner() bool {
	if g.row == 0 {
		return false
	}
	userWord := RuneArrayToString(g.grid[g.row-1])
	answer := RuneArrayToString(g.answer)
	return strings.Compare(userWord, answer) == 0
}
