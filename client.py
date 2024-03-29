from socket import socket
from tkinter import Tk, Entry, Button, CENTER, messagebox, END

main_window = Tk()
main_window.configure(bg="#E8FFCE")
main_window.attributes("-type", "dialog")
main_window.geometry("575x440")
main_window.title("Sudoku Client")

entries = []
previous_value = [" "] * 81
font = ("FiraCode, 28")

def create_grid():
    global entries
    entries = []
    for i in range(9):
        temp_list = []
        for j in range(9):
            grid_test = lambda x, y: ((3 * (x // 3) + y) // 3 % 2 == 0)
            if grid_test(i, j):
                temp_list.append(Entry(main_window, width = 2,  font = font, justify = CENTER, bg= "#cdffee", fg="#3b7c65"))
            else:
                temp_list.append(Entry(main_window, width = 2,  font = font, justify = CENTER, bg= "#acfadf", fg="#285e4c"))
            temp_list[j].grid(row = i, column = j, padx = 2, pady = 2)
        entries.append(temp_list)

def set_grid(value):
    create_grid()
    if value == None:
        return
    i = 0
    for sub_entries in entries:
        for entry in sub_entries:
            entry.delete(0, END)
            entry.insert(0, value[i])
            i += 1

def serialise():
    global entries
    output = ""
    for sub_entries in entries:
        for entry in sub_entries:
            text = entry.get()[0:1]
            output += text if text in "123456789" and text != "" else "0"
    return output

def solve():
    global previous_value
    s = socket()
    s.connect(("localhost", 6528))
    data = serialise()
    for (i, val) in enumerate(data):
        previous_value[i] = val if val != "0" else " "
    print("Sent data:\t\t", data)
    s.send(data.encode())
    s.send(b"\n\n")
    output = s.recv(82).decode()
    print("Output Received:\t", output)
    s.close()
    if output[:1] != "1":
        print("Solution not found")
        messagebox.showerror("Error", "Solution not found!")
        return
    i = 0
    for sub_entries in entries:
        for entry in sub_entries:
            i += 1
            if (entry.get() == " " or entry.get() == ""):
                entry.configure(fg="#50a95d")
            entry.delete(0, END)
            entry.insert(0, output[i:i+1])
    print()

create_grid()    

button_bg = "#a6d7cd"
button_fg = "#084639"
Button(main_window, text = "Solve", justify = CENTER, font = (font[0], 20), command = solve, width=5, bg=button_bg, fg=button_fg).grid(row = 0, column = 9)
Button(main_window, text = "Reset", justify = CENTER, font = (font[0], 20), command = lambda:set_grid(previous_value), width=5, bg=button_bg, fg=button_fg).grid(row = 1, column = 9)
Button(main_window, text = "Clear", justify = CENTER, font = (font[0], 20), command = lambda:set_grid(None), width=5, bg=button_bg, fg=button_fg).grid(row = 2, column = 9)

main_window.mainloop()
