This is a Rust implementation of the Nand2Tetris Hack Assembler. This is project 6 of the Nand2Tetris course.

References:

[Nand2Tetris IDE](https://nand2tetris.github.io/web-ide/asm)

[Nand2Tetris projects](https://www.nand2tetris.org/course)

[Project 6](https://drive.google.com/file/d/1CITliwTJzq19ibBF5EeuNBZ3MJ01dKoI/view)

[Lecture 6](https://drive.google.com/file/d/1uKGRMnL-gqk9DsgeN50z0EpHoSMWe6F5/view)

[Book Chapter 6](https://www.nand2tetris.org/_files/ugd/44046b_89a8e226476741a3b7c5204575b8a0b2.pdf)


How to test the code:

Download an .asm file from the [Nand2Tetris IDE](https://nand2tetris.github.io/web-ide/asm) (Hit the file button, go to projects, hit 06 and download the folder).

cargo run /your/file.asm

The program will spit out file.hack in your program folder, along with a bunch of debugging info (I used cargo run /your/file.asm > file.debug to look at this junk).

You can upload and compare the binary code with any of the project 6 .asm files using the compare tool in the online Nand2Tetris IDE!
