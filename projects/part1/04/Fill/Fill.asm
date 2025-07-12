// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

    @color
    M=1
(LOOP)
    @KBD
    D=M
    @FILL_BLACK
    D;JNE // if (key_pressed) goto FILL_BLACK

    @color
    D=M
    @FILL_WHITE
    D;JEQ // if (color == 0) goto FILL_WHITE

    @LOOP
    0;JMP

(FILL_BLACK)
    @color
    M=0 // black + 1
    @FILL
    0;JMP

(FILL_WHITE)
    @color
    M=1 // white + 1
    @FILL
    0;JMP

(FILL)
    @24576 // 24575 + 1
    D=A
    @pos
    M=D
(FILL_LOOP)
    @color
    D=M

    @pos
    AM=M-1
    M=D-1 // set color

    D=A // *pos
    
    // if (*pos - SCREEN == 0) goto MAIN
    @SCREEN
    D=D-A
    @LOOP
    D;JEQ

    @FILL_LOOP
    0;JMP
