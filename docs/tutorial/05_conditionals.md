# Lesson 5: Conditionals

Conditionals allow your program to make decisions and execute different code based on whether a condition is true or false.

## Boolean Values

Zenith has a `Boolean` type which can be either `true` or `false`.

```zenith
var is_active = true
var is_finished = false
```

## Comparison Operators

You can compare values using the following operators:

*   `==` : Equal to
*   `!=` : Not equal to
*   `<`  : Less than
*   `>`  : Greater than
*   `<=` : Less than or equal to
*   `>=` : Greater than or equal to

```zenith
var age = 18
var is_adult = age >= 18
println(is_adult) // true
```

## Logical Operators

You can combine boolean values using logical operators:

*   `&&` : AND (true if **both** sides are true)
*   `||` : OR (true if **at least one** side is true)
*   `!`  : NOT (inverts the value)

```zenith
var has_ticket = true
var has_id = false
var can_enter = has_ticket && has_id // false
```

## The `if` Statement

The `if` statement executes a block of code only if the condition is true.

```zenith
var score = 85

if (score > 50) {
    println("You passed!")
}
```

## `else` and `else if`

You can handle alternative cases using `else` and `else if`.

```zenith
var temperature = 25

if (temperature > 30) {
    println("It's hot outside.")
} else if (temperature < 10) {
    println("It's cold outside.")
} else {
    println("The weather is nice.")
}
```

## Null Checks

You can explicitly check if a value is `null` (or `Null` in type verification).

```zenith
var user = null

if (user == null) {
    println("No user found.")
} else {
    println("Welcome, user!")
}
```

## Truthiness

In Zenith, conditionals expect boolean values. Comparisons return strict booleans.

```zenith
if (true) {
    println("Always runs")
}

// if (1) { ... } // Error: Condition must be boolean
```

## Practical Example: Access Control

```zenith
func check_access(user_role, is_admin) {
    if (is_admin || user_role == "editor") {
        return "Access Granted"
    } else {
        return "Access Denied"
    }
}

println(check_access("viewer", false)) // Access Denied
println(check_access("editor", false)) // Access Granted
```

## Next Steps

Conditionals allow your code to branch. Next, let's look at how to repeat code execution with **Loops**.

[Next Lesson: Loops](06_loops.md)
