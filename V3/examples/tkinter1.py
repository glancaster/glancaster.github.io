import tkinter as tk

def button_click(number):
    current = entry.get()
    entry.delete(0, tk.END)
    entry.insert(0, current + str(number))

def button_clear():
    entry.delete(0, tk.END)

def button_equal():
    try:
        result = eval(entry.get())
        entry.delete(0, tk.END)
        entry.insert(0, str(result))
    except:
        entry.delete(0, tk.END)
        entry.insert(0, "Error")

# Create the main window
root = tk.Tk()
root.title("Calculator")

# Create and place entry for the display
entry = tk.Entry(root, width=35, borderwidth=5)
entry.grid(row=0, column=0, columnspan=4, padx=10, pady=10)

# Create and place buttons
buttons = [
    '7', '8', '9', '/',
    '4', '5', '6', '*',
    '1', '2', '3', '-',
    '0', '.', '=', '+'
]

row_val = 1
col_val = 0

for button in buttons:
    if button == '=':
        tk.Button(root, text=button, padx=20, pady=20, command=button_equal).grid(row=row_val, column=col_val, columnspan=2)
        col_val += 1
    elif button == '0':
        tk.Button(root, text=button, padx=20, pady=20, command=lambda b=button: button_click(b)).grid(row=row_val, column=col_val, columnspan=2)
        col_val += 1
    else:
        tk.Button(root, text=button, padx=20, pady=20, command=lambda b=button: button_click(b)).grid(row=row_val, column=col_val)
    
    col_val += 1
    if col_val > 3:
        col_val = 0
        row_val += 1

tk.Button(root, text="Clear", padx=20, pady=20, command=button_clear).grid(row=row_val, column=col_val, columnspan=4)

# Start the GUI event loop
root.mainloop()
