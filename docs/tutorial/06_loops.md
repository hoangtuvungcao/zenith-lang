# Zenith Tutorial - Bài 6: Vòng Lặp (Loops)

## Mục tiêu
- Sử dụng vòng lặp `while`
- Sử dụng vòng lặp `for`
- Làm việc với ranges
- Điều khiển vòng lặp với `break` và `continue`

---

## 1. Vòng Lặp While

Vòng lặp `while` lặp lại khi điều kiện còn đúng:

```zenith
func main() {
    var count = 0
    
    while (count < 5) {
        println("Count: " + count)
        count = count + 1
    }
    
    println("Xong!")
}

main()
```

**Kết quả**:
```
Count: 0
Count: 1
Count: 2
Count: 3
Count: 4
Xong!
```

---

## 2. Đếm Ngược

```zenith
func main() {
    var countdown = 10
    
    while (countdown > 0) {
        println(countdown)
        countdown = countdown - 1
    }
    
    println("🚀 Phóng!")
}

main()
```

---

## 3. Vòng Lặp For với Range

Cú pháp: `for variable in start..end`

```zenith
func main() {
    // In số từ 0 đến 4
    for i in 0..5 {
        println("Số: " + i)
    }
}

main()
```

**Lưu ý**: `0..5` nghĩa là từ 0 đến 4 (không bao gồm 5)

---

## 4. Bảng Cửu Chương

```zenith
func main() {
    println("=== Bảng Cửu Chương 5 ===")
    
    for i in 1..11 {
        var result = 5 * i
        println("5 x " + i + " = " + result)
    }
}

main()
```

---

## 5. Vòng Lặp Lồng Nhau

```zenith
func main() {
    println("Bảng nhân từ 1 đến 3:")
    
    for i in 1..4 {
        for j in 1..4 {
            var product = i * j
            println(i + " x " + j + " = " + product)
        }
        println("---")
    }
}

main()
```

---

## 6. Lặp Qua Mảng

```zenith
func main() {
    var fruits = ["Apple", "Banana", "Orange"]
    
    println("Danh sách trái cây:")
    for i in 0..len(fruits) {
        println((i + 1) + ". " + fruits[i])
    }
}

main()
```

**Kết quả**:
```
Danh sách trái cây:
1. Apple
2. Banana
3. Orange
```

---

## 7. Break - Thoát Vòng Lặp

`break` dừng vòng lặp ngay lập tức:

```zenith
func main() {
    var i = 0
    
    while (true) {  // Vòng lặp vô hạn
        if (i >= 5) {
            break  // Thoát khi i = 5
        }
        println("i = " + i)
        i = i + 1
    }
    
    println("Đã thoát vòng lặp")
}

main()
```

---

## 8. Continue - Bỏ Qua Lần Lặp

`continue` bỏ qua phần còn lại và chuyển sang lần lặp tiếp theo:

```zenith
func main() {
    println("Số chẵn từ 0 đến 9:")
    
    for i in 0..10 {
        if (i % 2 != 0) {
            continue  // Bỏ qua số lẻ
        }
        println(i)
    }
}

main()
```

**Kết quả**:
```
0
2
4
6
8
```

---

## 9. Tính Tổng

```zenith
func main() {
    var sum = 0
    
    for i in 1..101 {
        sum = sum + i
    }
    
    println("Tổng từ 1 đến 100: " + sum)  // 5050
}

main()
```

---

## 10. Tìm Số Nguyên Tố

```zenith
func is_prime(n) {
    if (n < 2) {
        return false
    }
    
    for i in 2..n {
        if (n % i == 0) {
            return false
        }
    }
    
    return true
}

func main() {
    println("Số nguyên tố từ 1 đến 20:")
    
    for num in 1..21 {
        if (is_prime(num)) {
            println(num)
        }
    }
}

main()
```

---

## 11. Bài Tập

### Bài 1: FizzBuzz
In số từ 1 đến 30, nhưng:
- Nếu chia hết cho 3: in "Fizz"
- Nếu chia hết cho 5: in "Buzz"
- Nếu chia hết cho cả 3 và 5: in "FizzBuzz"
- Ngược lại: in số

### Bài 2: Tìm Số Lớn Nhất
Cho mảng `[15, 42, 8, 23, 16, 4]`, tìm số lớn nhất.

### Bài 3: Đảo Ngược Mảng
Cho mảng `[1, 2, 3, 4, 5]`, in ra `[5, 4, 3, 2, 1]`.

### Bài 4: Vẽ Hình
Sử dụng vòng lặp để vẽ tam giác:
```
*
**
***
****
*****
```

### Bài 5: Tính Giai Thừa
Viết hàm tính giai thừa (factorial) của một số.
Ví dụ: `5! = 5 * 4 * 3 * 2 * 1 = 120`

---

## 12. Pattern Matching (Mẫu Hình)

### Hình Tam Giác
```zenith
func main() {
    for i in 1..6 {
        var line = ""
        for j in 0..i {
            line = line + "*"
        }
        println(line)
    }
}

main()
```

### Hình Vuông
```zenith
func main() {
    for i in 0..5 {
        var line = ""
        for j in 0..5 {
            line = line + "* "
        }
        println(line)
    }
}

main()
```

---

## 13. Lỗi Thường Gặp

❌ **Vòng lặp vô hạn**:
```zenith
var i = 0
while (i < 10) {
    println(i)
    // Quên tăng i!
}
```

❌ **Off-by-one error**:
```zenith
// Muốn lặp 10 lần
for i in 0..10 {  // Lặp từ 0 đến 9 (10 lần) ✅
}

for i in 0..11 {  // Lặp 11 lần ❌
}
```

❌ **Sai range**:
```zenith
var arr = [1, 2, 3]
for i in 0..4 {      // ❌ Index 3 sẽ lỗi!
    println(arr[i])
}

for i in 0..len(arr) {  // ✅ Đúng
    println(arr[i])
}
```

---

## 14. Tips & Tricks

### Đếm Xuôi/Ngược
```zenith
// Xuôi
for i in 0..10 {
    println(i)  // 0, 1, 2, ..., 9
}

// Ngược (cần tính toán)
for i in 0..10 {
    var reverse = 9 - i
    println(reverse)  // 9, 8, 7, ..., 0
}
```

### Lặp Với Bước Nhảy
```zenith
// Chỉ số chẵn
for i in 0..10 {
    if (i % 2 == 0) {
        println(i)  // 0, 2, 4, 6, 8
    }
}
```

---

## Tóm Tắt

| Cấu trúc | Cú pháp | Mục đích |
|----------|---------|----------|
| `while` | `while (condition) { }` | Lặp khi điều kiện đúng |
| `for` | `for var in range { }` | Lặp qua một dãy số |
| `break` | `break` | Thoát vòng lặp |
| `continue` | `continue` | Bỏ qua lần lặp hiện tại |
| `range` | `start..end` | Dãy số từ start đến end-1 |

---

## Bài Tiếp Theo
👉 [Bài 7: Hàm (Functions)](07_functions.md)

**Congratulations! Bạn đã nắm vững vòng lặp! 🎉**
