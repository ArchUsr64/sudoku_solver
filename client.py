from socket import socket
from tkinter import Tk, Entry, Button, CENTER

main_window = Tk()
main_window.geometry("601x472")
main_window.title("Sudoku")
entries = []
font = ("FiraCode, 25")
for i in range(9):
    temp_list = []
    for j in range(9):
        temp_list.append(Entry(main_window, width = 2,  font = font, justify = CENTER))
        temp_list[j].grid(row = i, column = j, padx = 2, pady = 2)
    entries.append(temp_list)
def serialise():
    global entries
    output = ""
    for sub_entries in entries:
        for entry in sub_entries:
            text = entry.get()[0:1]
            output += text if text in "123456789" and text != "" else "0"
    return output
def solve():
    print(serialise());
Button(main_window, text = "Solve", justify = CENTER, font = font, command = solve).grid(row = 0, column = 9)

main_window.mainloop()
