# Lesson 8: Arrays

Arrays are ordered collections of values. They can hold values of any type, including other arrays.

## Creating Arrays

Use square brackets `[]` to create an array:

```zenith
var empty = []
var numbers = [1, 2, 3, 4, 5]
var fruits = ["Apple", "Banana", "Orange"]
var mixed = [1, "two", true, [3, 4]]

println(fruits)
```

## Accessing Elements

Access elements using their index (starting at 0):

```zenith
var fruits = ["Apple", "Banana", "Orange"]

println(fruits[0]) // Apple
println(fruits[1]) // Banana
println(fruits[2]) // Orange
```

## Modifying Elements

You can change an element by assigning a new value to its index:

```zenith
var numbers = [10, 20, 30]
numbers[1] = 99
println(numbers) // [10, 99, 30]
```

## Array Length

Use the `len()` function to get the number of elements:

```zenith
var fruits = ["Apple", "Banana"]
println(len(fruits)) // 2
```

## Adding Elements

Use `push()` to add elements to the end of an array. Note that `push()` returns the *new* array (or modifies it in place depending on implementation - in Zenith currently it modifies the array and returns it).

```zenith
var nums = [1, 2]
push(nums, 3)
println(nums) // [1, 2, 3]
```

## Removing Elements

Use `remove(array, index)` to remove an element at a specific index.

```zenith
var items = ["A", "B", "C"]
remove(items, 1) // Removes "B" at index 1
println(items) // ["A", "C"]
```

## Iterating Over Arrays

Use a `for` loop with a range to iterate through an array:

```zenith
var colors = ["Red", "Green", "Blue"]

for i in 0..len(colors) {
    println(colors[i])
}
```

## Multi-dimensional Arrays

Arrays can contain other arrays:

```zenith
var matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]

println(matrix[1][2]) // 6 (Row 1, Column 2)
```

## Next Steps

Now that you can store lists of data, let's learn about **Objects** to store key-value pairs.

[Next Lesson: Objects](09_objects.md)
