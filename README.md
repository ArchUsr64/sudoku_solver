# Sudoku Solver
This application uses a server client architecture for solving sudoku boards. The server is implemented in Rust and utilizes backtracking algorithm to solve the board. While the client is Python based and provides the user with a GUI to enter unsolved boards and see the output. By default the port number `6528` is used.  
### The client/server communicate over TCP with the following text encoded data fromat:
#### Query
Size: 83 bytes  
First 81 bytes contain ASCII characters from `'0'`-`'9'` where `'0'` denotes an empty cell terminated by two `'\n'` new-line symbols
#### Reply
Size: 82 bytes  
First byte is either `'0'` or `'1'`, `'0'` denotes the board can't be solved else the rest 81 bytes represent the solved board state
## Run the client
On a system with the `tkinter` library installed, run:
`python client.py`  
<br>
![client](https://github.com/ArchUsr64/sudoku_solver/assets/83179501/a5bdbfff-3e4d-4006-a6c5-83ec82c6b81a)
  
## Run the server
On a system with `rust toolchain` installed, run:
`cargo run --release`  
<br>
![server](https://github.com/ArchUsr64/sudoku_solver/assets/83179501/b281d910-edc8-4c58-88da-de79fc37a74a)

