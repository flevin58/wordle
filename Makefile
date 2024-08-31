.SUFFIXES:
.PHONY: all clean build run windows linux darwin mac

APP = wordle
OS = $(go env $(GOHOSTOS))
ARCH = $(go env $(GOARCH))

WINDOWS_TARGET = target/windows/$(APP).exe
LINUX_TARGET = target/linux/$(APP)
DARWIN_UNIVERSAL = target/darwin/universal/$(APP)
DARWIN_AMD64 = target/darwin/x64/$(APP)
DARWIN_ARM64 = target/darwin/arm/$(APP)

ifeq ($(OS),darwin)
EXECUTABLE = $(DARWIN_UNIVERSAL)
DEL = rm -f
BUILD = darwin
endif

ifeq ($(OS),linux)
EXECUTABLE = $(LINUX_TARGET)
DEL = rm -f

build: linux

endif

ifeq ($(OS),windows)
EXECUTABLE = $(WINDOWS_TARGET)
DEL = del

build: windows

endif

run: build
	@$(EXECUTABLE)

# --------------------------------------------------------------------
# Cross-Platform targets.
# They will work independently of the host os.
# --------------------------------------------------------------------

windows:
	@echo "Compiling for Windows x64"
	@GOOS=windows GOARCH=amd64 go build -o $(WINDOWS_TARGET)

linux:
	@echo "Compiling for Linux x64"
	@GOOS=linux GOARCH=amd64 go build -o $(LINUX_TARGET)

darwin:
	@echo "Compiling for Darwin x64"
	@GOOS=darwin GOARCH=amd64 go build -o $(DARWIN_AMD64)
	@echo "Compiling for Darwin arm64"
	@GOOS=darwin GOARCH=arm64 go build -o $(DARWIN_ARM64)
	@echo "Creating Darwin universal binary"
	@lipo $(DARWIN_ARM64) $(DARWIN_AMD64) -create -output $(DARWIN_TARGET)

mac: darwin

dependencies:
	@go install github.com/konoui/lipo@latest
ifeq ($(OS),Linux)
	@apt install libx11-dev xorg-dev
endif

clean:
	@$(DEL) $(WINDOWS_TARGET)
	@$(DEL) $(LINUX_TARGET)
	@$(DEL) $(DARWIN_UNIVERSAL)
	@$(DEL) $(DARWIN_ARM64)
	@$(DEL) $(DARWIN_AMD64)

all: windows linux darwin
