# Lesson 4: Strings in Zenith

Strings are sequences of characters used to represent text. In Zenith, strings are immutable and support a variety of powerful operations.

## Creating Strings

You can create strings using double quotes:

```zenith
var name = "Zenith"
var greeting = "Hello, World!"
var empty = ""
```

## Concatenation

You can join strings together using the `+` operator.

```zenith
var first = "Hello"
var second = "World"
var message = first + ", " + second + "!"
println(message) // "Hello, World!"
```

### Auto-Conversion

Zenith automatically converts other types to strings when concatenated with a string:

```zenith
var age = 25
var text = "I am " + age + " years old."
// "I am 25 years old."

var list = [1, 2, 3]
println("Items: " + list) 
// "Items: [1, 2, 3]"
```

### Null Handling

If you concatenate a string with `Null`, it is treated as an empty string (or "null" depending on context, but typically handled gracefully to verify).

```zenith
var missing = null
var combined = "Value: " + missing
println(combined) // "Value: " (or similar safe output)
```

## String Properties

### Length

Use the `len()` function to get the number of characters in a string:

```zenith
var text = "Zenith"
println(len(text)) // 6
```

## String Methods

The standard library provides several useful functions for strings.

### `str_split(text, delimiter)`

Splits a string into an array of substrings.

```zenith
var csv = "apple,banana,cherry"
var fruits = str_split(csv, ",")
// fruits is ["apple", "banana", "cherry"]
```

### `str_contains(text, substring)`

Checks if a string contains a specific substring. Returns `true` or `false`.

```zenith
var email = "user@example.com"
if (str_contains(email, "@")) {
    println("Valid email format")
}
```

### `parse_int(text)`

Converts a string to an integer. Returns an error if the format is invalid.

```zenith
var number_str = "42"
var number = parse_int(number_str)
println(number + 10) // 52
```

## Practical Example: User Input

Here is how you might process user input in a CLI or Form:

```zenith
func process_input(input) {
    // 1. Validate length
    if (len(input) == 0) {
        return "Error: Input cannot be empty"
    }

    // 2. Check for keywords
    if (str_contains(input, "admin")) {
        return "Welcome, Administrator!"
    }

    // 3. Format output
    return "User said: " + input
}

println(process_input("admin_login"))
```

## Next Steps

Now that you understand strings, let's look at how to control the flow of your program with **Conditionals**.

[Next Lesson: Conditionals](05_conditionals.md)
