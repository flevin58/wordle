package game

import (
	"embed"
	_ "embed"
	"fmt"
	"image/color"
	"log"
	"strconv"

	"golang.org/x/image/font"
	"golang.org/x/image/font/opentype"
	"gopkg.in/yaml.v3"
)

const (
	nRows = 6
	nCols = 5
)

type Color string

func (c Color) toRGBA() color.RGBA {
	s := string(c)
	r, err1 := strconv.ParseUint(s[1:3], 16, 8)
	g, err2 := strconv.ParseUint(s[3:5], 16, 8)
	b, err3 := strconv.ParseUint(s[5:7], 16, 8)
	a, err4 := strconv.ParseUint(s[7:9], 16, 8)
	if err1 != nil || err2 != nil || err3 != nil || err4 != nil {
		log.Fatalln("error in configuration file")
	}
	return color.RGBA{
		R: uint8(r),
		G: uint8(g),
		B: uint8(b),
		A: uint8(a),
	}
}

type FontFace struct {
	Face string
	Size float64
}

type Message struct {
	Title string
	Text  string
}

type Config struct {
	Name    string
	Version string
	Screen  struct {
		Width  int
		Height int
	}
	Messages struct {
		Playing Message
		Enter   Message
		Win     Message
		Loose   Message
	}
	Colors struct {
		Lightgrey Color
		Grey      Color
		Yellow    Color
		Green     Color
		Outline   Color
		Blue      Color
		White     Color
	}
	Font struct {
		Letter  FontFace
		Title   FontFace
		Message FontFace
	}
}

var wconf Config

//go:embed assets/wordle.yaml
var yfile []byte

//go:embed assets/fonts
var fontDir embed.FS

func getFontFace(theFont FontFace) font.Face {
	fontName := fmt.Sprintf("assets/fonts/UbuntuSansMono-%s.otf", theFont.Face)
	fontData, err := fontDir.ReadFile(fontName)
	if err != nil {
		log.Fatal(err)
	}

	tt, err := opentype.Parse(fontData)
	if err != nil {
		log.Fatal(err)
	}
	ff, err := opentype.NewFace(tt, &opentype.FaceOptions{
		Size:    theFont.Size,
		DPI:     72,
		Hinting: font.HintingFull,
	})
	if err != nil {
		log.Fatal(err)
	}

	return ff
}

func init() {
	err := yaml.Unmarshal(yfile, &wconf)
	if err != nil {
		log.Fatalf("couldn't unmarshal the configuration file: %v", err)
	}

	log.Println(wconf)
}
