# Zenith Language Handbook: Zero to Hero

Welcome to Zenith! This handbook will guide you through the Zenith programming language, from writing your first "Hello World" to building interactive GUI applications.

## Table of Contents
1. [Introduction & Setup](#1-introduction--setup)
2. [Variables & Types](#2-variables--types)
3. [Control Flow](#3-control-flow)
4. [Functions](#4-functions)
5. [Data Structures](#5-data-structures)
6. [Building UIs](#6-building-uis)
7. [Standard Library](#7-standard-library)

---

## 1. Introduction & Setup

Zenith is a dynamic, interpreted language designed for rapid application development with built-in GUI support.

### Running a Script
To run a Zenith script in the console:
```bash
zenith run main.zn
```

### Running a GUI Application
To run a script with a graphical interface:
```bash
zenith gui app.zn
```

### Your First Program
Create a file named `hello.zn`:
```zenith
func main() {
    println("Hello, Zenith!")
}
```
Run it: `zenith run hello.zn`

---

## 2. Variables & Types

Zenith is dynamically typed. You declare variables using `var`.

```zenith
var name = "Alice"       // String
var age = 30             // Integer/Number
var height = 1.75        // Number
var is_active = true     // Boolean
var scores = [10, 20]    // Array
var user = {"id": 1}     // Object
```

### Strings
Strings support concatenation with `+`.
```zenith
var greeting = "Hello " + name
```

---

## 3. Control Flow

### If / Else
```zenith
if (age >= 18) {
    println("Adult")
} else if (age > 12) {
    println("Teen")
} else {
    println("Child")
}
```

### Loops
**While Loop**:
```zenith
var i = 0
while (i < 5) {
    println(i)
    i = i + 1
}
```

**For Loop**:
Iterate over a range:
```zenith
for i in 0..5 {
    println(i) // 0, 1, 2, 3, 4
}
```

Iterate over an array:
*Coming soon! For now, use index:*
```zenith
var items = ["a", "b"]
for i in 0..len(items) {
    println(items[i])
}
```

---

## 4. Functions

Callback functions are first-class citizens.

```zenith
func add(a, b) {
    return a + b
}

var result = add(5, 10)
```

### Recursion
Zenith supports recursive calls.
```zenith
func fib(n) {
    if (n <= 1) { return n }
    return fib(n-1) + fib(n-2)
}
```

---

## 5. Data Structures

### Arrays
Mixed-type arrays are supported.
```zenith
var list = ["apple", 10, true]

println(list[0]) // Access
push(list, "banana") // Add
// remove(list, index) // Remove
println(len(list)) // Length
```

### Objects (Dictionaries)
Key-value pairs with string keys.
```zenith
var person = {
    "name": "Bob",
    "city": "New York"
}

println(person["name"])
person["age"] = 40 // Add/Update
```

---

## 6. Building UIs

Zenith's killer feature is its declarative UI engine.

### The UI Loop
A typical UI app looks like this:

1.  **State**: Define your app's data.
2.  **UI Function**: A function that takes state and returns a UI Object.
3.  **Main Loop**: Wait for events and update state.

```zenith
func main() {
    var state = { "count": 0 }
    
    // Initial Render
    render_ui(build_ui(state))
    
    while (true) {
        var event = wait_for_event()
        if (event["type"] == "click") {
            if (event["id"] == "inc") {
                state["count"] = state["count"] + 1
                render_ui(build_ui(state))
            }
        }
    }
}

func build_ui(state) {
    return {
        "type": "Center",
        "child": {
            "type": "Column",
            "children": [
                {
                    "type": "Text", 
                    "text": "Count: " + state["count"]
                },
                {
                    "type": "Button",
                    "label": "Increment",
                    "on_click": "inc"
                }
            ]
        }
    }
}
```

### Widgets

**Layouts:**
- `Column`: Vertical stack.
- `Row`: Horizontal stack.
- `Center`: Centers child.
- `Container`: Adds padding/margin.

**Controls:**
- `Text`: plain text.
- `Button`: Clickable button.
- `TextField`: Editable text input.

---

## 7. Standard Library

### IO
- `print(msg)`
- `println(msg)`

### Math & Logic
- `len(obj)`: Length of string/array/object.

### String Utils
- `str_upper(s)`
- `str_lower(s)`
- `str_split(s, sep)`
- `parse_int(s)`

### Array Utils
- `push(arr, item)`
- `remove(arr, index)`

### UI Utils
- `render_ui(ui_object)`: Updates the window.
- `wait_for_event()`: Blocks until user interaction. Returns an event object.
- `create_state(data)`: Creates managed state (advanced).
