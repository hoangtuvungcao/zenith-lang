# Zenith Tutorial - Bài 2: Biến và Kiểu Dữ Liệu

## Mục tiêu
- Khai báo và sử dụng biến
- Hiểu các kiểu dữ liệu cơ bản
- Thay đổi giá trị biến

---

## 1. Khai báo biến với `var`

```zenith
func main() {
    var name = "Alice"
    var age = 25
    var height = 1.65
    
    println("Tên: " + name)
    println("Tuổi: " + age)
    println("Chiều cao: " + height + "m")
}

main()
```

**Giải thích**:
- `var` là từ khóa khai báo biến
- `name` là tên biến
- `"Alice"` là giá trị gán cho biến
- Zenith tự động nhận biết kiểu dữ liệu!

---

## 2. Các kiểu dữ liệu cơ bản

### String (Chuỗi ký tự)
```zenith
var message = "Hello Zenith!"
var emoji = "🚀"
var multiline = "Dòng 1\nDòng 2"
```

### Integer (Số nguyên)
```zenith
var count = 42
var negative = -10
var zero = 0
```

### Float (Số thực)
```zenith
var pi = 3.14159
var temperature = 36.5
var price = 99.99
```

### Boolean (Đúng/Sai)
```zenith
var is_student = true
var has_license = false
```

### Null (Không có giá trị)
```zenith
var empty = null
```

---

## 3. Thay đổi giá trị biến

```zenith
func main() {
    var score = 0
    println("Điểm ban đầu: " + score)
    
    score = 10
    println("Điểm mới: " + score)
    
    score = 20
    println("Điểm cuối: " + score)
}

main()
```

**Kết quả**:
```
Điểm ban đầu: 0
Điểm mới: 10
Điểm cuối: 20
```

---

## 4. Kiểm tra kiểu dữ liệu với `type()`

```zenith
func main() {
    var name = "Zenith"
    var age = 1
    var is_cool = true
    
    println("Kiểu của name: " + type(name))    // string
    println("Kiểu của age: " + type(age))      // integer
    println("Kiểu của is_cool: " + type(is_cool))  // boolean
}

main()
```

---

## 5. Nối chuỗi

```zenith
func main() {
    var first_name = "Nguyen"
    var last_name = "Van A"
    var full_name = first_name + " " + last_name
    
    println("Họ tên: " + full_name)
}

main()
```

---

## 6. Chuyển đổi kiểu dữ liệu

```zenith
func main() {
    var age = 25
    var message = "Tôi " + age + " tuổi"  // Zenith tự động chuyển đổi!
    println(message)
    
    // Hoặc dùng hàm str() để chuyển sang string
    var height = 1.75
    println("Chiều cao: " + str(height))
}

main()
```

---

## 7. Bài tập

### Bài 1: Thẻ thông tin
Tạo biến cho:
- Tên của bạn
- Tuổi
- Thành phố
- Sở thích

In ra thẻ thông tin đẹp mắt!

### Bài 2: Máy tính đơn giản
```zenith
func main() {
    var a = 10
    var b = 5
    
    println("Tổng: " + (a + b))
    println("Hiệu: " + (a - b))
    // Thêm phép nhân và chia
}

main()
```

### Bài 3: Đổi giá trị
Tạo 2 biến `x` và `y`, sau đó hoán đổi giá trị của chúng.

---

## 8. Lỗi thường gặp

❌ **Sử dụng biến chưa khai báo**:
```zenith
println(username)  // SAI! Chưa khai báo 'username'
```

❌ **Quên từ khóa `var`**:
```zenith
name = "Alice"  // SAI! Phải dùng 'var name = ...'
```

✅ **Đúng**:
```zenith
var name = "Alice"
println(name)
```

---

## Tóm tắt

| Cú pháp | Ý nghĩa |
|---------|---------|
| `var x = value` | Khai báo biến |
| `x = new_value` | Thay đổi giá trị |
| `type(x)` | Kiểm tra kiểu dữ liệu |
| `str(x)` | Chuyển sang string |

---

## Bài tiếp theo
👉 [Bài 3: Phép Toán Số Học](03_math.md)
