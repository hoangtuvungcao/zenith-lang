# Lesson 7: Functions

Functions are reusable blocks of code that perform a specific task. They help organize your code and make it more modular.

## Defining a Function

Use the `func` keyword to define a function:

```zenith
func fail_greeting() {
    println("Hello, Zenith!")
}

fail_greeting() // Call the function
```

## Parameters

Functions can accept input values called parameters.

```zenith
func greet(name) {
    println("Hello, " + name + "!")
}

greet("Alice")
greet("Bob")
```

Multiple parameters are separated by commas:

```zenith
func add(a, b) {
    println(a + b)
}

add(5, 3) // 8
```

## Return Values

Use the `return` keyword to send a value back from the function.

```zenith
func square(n) {
    return n * n
}

var result = square(4)
println(result) // 16
```

If no `return` statement is used, the function returns `null`.

## Variable Scope

Variables defined inside a function are **local** to that function and cannot be accessed outside.

```zenith
func test_scope() {
    var local_var = "I am inside"
    println(local_var)
}

test_scope()
// println(local_var) // Error: Variable not found
```

Functions *can* access variables defined in the outer scope (Global or Parent variables).

```zenith
var global_var = "I am global"

func read_global() {
    println(global_var)
}

read_global() // "I am global"
```

## First-Class Functions

In Zenith, functions are values. You can assign them to variables or pass them to other functions.

```zenith
var my_func = func(x) {
    return x * 2
}

println(my_func(5)) // 10
```


## Recursion

Functions can call themselves. This is useful for tasks like calculating factorials.

```zenith
func factorial(n) {
    if (n <= 1) {
        return 1
    }
    return n * factorial(n - 1)
}

println(factorial(5)) // 120
```

## Next Steps

Now that you can organize code with functions, let's learn how to store collections of data with **Arrays**.

[Next Lesson: Arrays](08_arrays.md)
