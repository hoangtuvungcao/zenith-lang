# Zenith Standard Library Reference

**Version**: 1.0.0  
**Last Updated**: 2026-02-12

Complete reference for all built-in functions available in Zenith runtime.

---

## 📝 String Functions

### `str(value) -> String`
Convert any value to a string representation.

```zenith
str(42)          // "42"
str(true)        // "true"
str(3.14)        // "3.14"
str([1, 2, 3])   // "[1, 2, 3]" (representation)
```

### `str_split(text, delimiter) -> Array<String>`
Split a string by a delimiter.

```zenith
var parts = str_split("hello,world", ",")
// parts = ["hello", "world"]

var words = str_split("one two three", " ")
// words = ["one", "two", "three"]
```

### `str_contains(text, substring) -> Boolean`
Check if a string contains a substring.

```zenith
str_contains("hello world", "world")  // true
str_contains("zenith", "python")      // false
```

---

## 🔢 Type & Conversion Functions

### `type(value) -> String`
Get the type name of a value.

```zenith
type(42)        // "integer"
type("hello")   // "string"
type(true)      // "boolean"
type([1, 2])    // "array"
type({})        // "object"
```

### `parse_int(string) -> Integer`
Parse a string to an integer.

```zenith
parse_int("42")    // 42
parse_int("100")   // 100
parse_int("-5")    // -5
```

### `len(value) -> Integer`
Get the length of a string, array, or object.

```zenith
len("hello")           // 5
len([1, 2, 3, 4])      // 4
len({"a": 1, "b": 2})  // 2
```

---

## 📦 Array Functions

### `push(array, item) -> Array`
Add an item to an array and return the array.

```zenith
var nums = [1, 2, 3]
nums = push(nums, 4)
// nums = [1, 2, 3, 4]

// Functional style:
var items = []
items = push(items, "first")
items = push(items, "second")
```

**Note**: Returns the array itself for functional programming style.

### `remove(array, index) -> Array`
Remove an item from an array by index.

```zenith
var fruits = ["apple", "banana", "cherry"]
fruits = remove(fruits, 1)
// fruits = ["apple", "cherry"]
```

---

## 🎨 UI Functions

### `create_state(initial_value) -> State`
Create a reactive state container.

```zenith
var state = create_state({
    "count": 0,
    "name": "User"
})
```

### `get_state(state) -> Value`
Get the current value from a state container.

```zenith
var current = get_state(state)
var count = current["count"]
```

###set_state(state, new_value) -> Null`
Update the state with a new value.

```zenith
var current = get_state(state)
current["count"] = current["count"] + 1
set_state(state, current)
```

### `render_ui(ui_tree) -> Null`
Render a UI tree to the screen.

```zenith
render_ui({
    "type": "Center",
    "child": {
        "type": "Text",
        "text": "Hello, World!"
    }
})
```

### `wait_for_event() -> Object`
Wait for and return the next UI event.

```zenith
var event = wait_for_event()
// event = { "type": "click", "id": "button1" }
// event = { "type": "input", "id": "field1", "value": "text" }
```

**Event Types**:
- `"click"` - Button click events
- `"input"` - TextField change events
- `"change"` - Checkbox/other widget changes

---

## 🖨️ Console I/O

### `print(value) -> Null`
Print a value to stdout without newline.

```zenith
print("Hello")
print(" World")
// Output: Hello World
```

### `println(value) -> Null`
Print a value to stdout with newline.

```zenith
println("Line 1")
println("Line 2")
// Output:
// Line 1
// Line 2
```

---

## 📁 File I/O

### `read_file(path) -> String`
Read the contents of a file.

```zenith
var content = read_file("data.txt")
println(content)
```

### `write_file(path, content) -> Null`
Write content to a file.

```zenith
write_file("output.txt", "Hello, World!")
```

**Note**: File I/O requires appropriate filesystem permissions.

---

## ⏱️ Time Functions

### `time_now() -> String`
Get the current local time as a string.

```zenith
var now = time_now()
// "2026-02-12 06:44:50"
```

### `time_utc() -> String`
Get the current UTC time as a string.

```zenith
var utc = time_utc()
// "2026-02-11 23:44:50"
```

### `time_format(time, format) -> String`
Format a time string according to a format.

```zenith
var formatted = time_format(time_now(), "%Y-%m-%d")
// "2026-02-12"
```

---

## Complete Function List

| Function | Arguments | Returns | Description |
|----------|-----------|---------|-------------|
| `str` | value | String | Convert to string |
| `type` | value | String | Get type name |
| `len` | value | Integer | Get length |
| `parse_int` | string | Integer | Parse integer |
| `str_split` | text, delim | Array | Split string |
| `str_contains` | text, sub | Boolean | Check substring |
| `push` | array, item | Array | Add to array |
| `remove` | array, index | Array | Remove from array |
| `print` | value | Null | Print to stdout |
| `println` | value | Null | Print line to stdout |
| `read_file` | path | String | Read file |
| `write_file` | path, content | Null | Write file |
| `time_now` | - | String | Current local time |
| `time_utc` | - | String | Current UTC time |
| `time_format` | time, format | String | Format time |
| `create_state` | initial | State | Create state |
| `get_state` | state | Value | Get state value |
| `set_state` | state, value | Null | Update state |
| `render_ui` | tree | Null | Render UI |
| `wait_for_event` | - | Object | Get next event |

---

## Usage Examples

### Example 1: String Manipulation

```zenith
var text = "hello,world,zenith"
var parts = str_split(text, ",")

for i in 0..len(parts) {
    var word = parts[i]
    println("Word " + str(i) + ": " + word)
}

// Output:
// Word 0: hello
// Word 1: world
// Word 2: zenith
```

### Example 2: Array Operations

```zenith
var numbers = []
numbers = push(numbers, 10)
numbers = push(numbers, 20)
numbers = push(numbers, 30)

println("Array length: " + str(len(numbers)))
println("First: " + str(numbers[0]))
println("Last: " + str(numbers[2]))

// Remove middle element
numbers = remove(numbers, 1)
println("After remove: " + str(len(numbers)))
```

### Example 3: State Management

```zenith
func main() {
    var state = create_state({ "count": 0 })
    
    while (true) {
        var current = get_state(state)
        
        render_ui({
            "type": "Center",
            "child": {
                "type": "Column",
                "children": [
                    { "type": "Text", "text": "Count: " + str(current["count"]) },
                    { "type": "Button", "label": "Increment", "on_click": "inc" }
                ]
            }
        })
        
        var event = wait_for_event()
        
        if (event["type"] == "click" && event["id"] == "inc") {
            current["count"] = current["count"] + 1
            set_state(state, current)
        }
    }
}
```

### Example 4: File Processing

```zenith
// Read file
var content = read_file("input.txt")

// Process
var lines = str_split(content, "\n")
var line_count = len(lines)

// Write result
var result = "Total lines: " + str(line_count)
write_file("output.txt", result)

println(result)
```

---

## Error Handling

Most functions will raise runtime errors for invalid inputs:

```zenith
// TypeError: len() expects string, array, or object
len(42)

// TypeError: push expects array as first arg
push("not an array", 5)

// TypeError: parse_int expects string
parse_int(123)
```

**Best Practice**: Validate types with `type()` before operations:

```zenith
var value = get_input()
if (type(value) == "string") {
    var num = parse_int(value)
    println("Parsed: " + str(num))
}
```

---

## See Also

- [UI Widgets Reference](ui_widgets.md) - Complete UI widget documentation
- [Tutorial](../tutorial/) - Step-by-step lessons
- [Examples](../../examples/) - Working code examples

---

**Zenith Standard Library** - Simple, Powerful, Cross-Platform 🚀
