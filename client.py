from socket import socket
from tkinter import Tk, Entry, Button, CENTER, messagebox, END

main_window = Tk()
main_window.geometry("601x472")
main_window.title("Sudoku")
entries = []
font = ("FiraCode, 25")
def create_grid():
    global entries
    entries = []
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
            print(entry.get())
            text = entry.get()[0:1]
            output += text if text in "123456789" and text != "" else "0"
    return output
def solve():
    s = socket()
    s.connect(("localhost", 1234))
    data = serialise()
    print("Sent data:", data)
    s.send(data.encode())
    s.send(b"\n\n")
    output = s.recv(82).decode()
    print("Output Received:", output)
    s.close()
    if output[:1] != "1":
        print("Solution not found")
        messagebox.showerror("Error", "Solution not found!")
        return
    i = 0
    for sub_entries in entries:
        for entry in sub_entries:
            i += 1
            entry.delete(0, END)
            entry.insert(0, output[i:i+1])
create_grid()    
Button(main_window, text = "Solve", justify = CENTER, font = font, command = solve).grid(row = 0, column = 9)
Button(main_window, text = "Reset", justify = CENTER, font = font, command = create_grid).grid(row = 1, column = 9)

main_window.mainloop()
