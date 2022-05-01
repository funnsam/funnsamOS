package kernel
 
import "terminal"
 
func Main() {
	terminal.Init()
	terminal.Color = terminal.MakeColor(terminal.White, terminal.Blue)
	terminal.Clear() // Clear screen
 
	// Center the text a little
	terminal.Row = 11
	terminal.Column = 30
 
	// Print our Hello, World!
	terminal.Print("Hello, Kernel World!\n")
}