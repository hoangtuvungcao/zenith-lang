# Zenith Tutorial - Bài 3: Phép Toán và Biểu Thức

## Mục tiêu
- Thực hiện các phép toán số học
- Hiểu thứ tự ưu tiên của phép toán
- Sử dụng biểu thức phức tạp

---

## 1. Các Phép Toán Cơ Bản

### Phép cộng (+)
```zenith
func main() {
    var a = 10
    var b = 20
    var sum = a + b
    println("Tổng: " + sum)  // 30
}

main()
```

### Phép trừ (-)
```zenith
func main() {
    var price = 100
    var discount = 20
    var final_price = price - discount
    println("Giá sau giảm: " + final_price)  // 80
}

main()
```

### Phép nhân (*)
```zenith
func main() {
    var width = 5
    var height = 10
    var area = width * height
    println("Diện tích: " + area)  // 50
}

main()
```

### Phép chia (/)
```zenith
func main() {
    var total = 100
    var people = 4
    var per_person = total / people
    println("Mỗi người: " + per_person)  // 25
}

main()
```

### Phép chia lấy dư (%)
```zenith
func main() {
    var number = 17
    var divisor = 5
    var remainder = number % divisor
    println("Số dư: " + remainder)  // 2
}

main()
```

---

## 2. Thứ Tự Ưu Tiên

Zenith tuân theo thứ tự toán học chuẩn:
1. **()** - Ngoặc đơn (cao nhất)
2. **\* / %** - Nhân, chia, chia dư
3. **+ -** - Cộng, trừ (thấp nhất)

```zenith
func main() {
    var result1 = 10 + 5 * 2      // 20 (nhân trước, cộng sau)
    var result2 = (10 + 5) * 2    // 30 (ngoặc đơn trước)
    
    println("Không ngoặc: " + result1)
    println("Có ngoặc: " + result2)
}

main()
```

---

## 3. Làm Việc Với Số Thực

```zenith
func main() {
    var pi = 3.14159
    var radius = 5.0
    var circumference = 2.0 * pi * radius
    
    println("Chu vi: " + circumference)  // ~31.42
    
    // Tính diện tích hình tròn
    var area = pi * radius * radius
    println("Diện tích: " + area)  // ~78.54
}

main()
```

---

## 4. Toán Tử Gán Phức Hợp

```zenith
func main() {
    var score = 100
    
    score = score + 10  // Tăng 10 điểm
    println("Điểm sau bonus: " + score)  // 110
    
    score = score - 5   // Trừ 5 điểm
    println("Điểm sau penalty: " + score)  // 105
    
    score = score * 2   // Nhân đôi điểm
    println("Điểm x2: " + score)  // 210
}

main()
```

---

## 5. Biểu Thức Phức Tạp

```zenith
func main() {
    // Công thức tính nhiệt độ F sang C: C = (F - 32) * 5 / 9
    var fahrenheit = 98.6
    var celsius = (fahrenheit - 32) * 5 / 9
    println("Nhiệt độ cơ thể: " + celsius + "°C")  // ~37°C
    
    // Công thức tính BMI: BMI = weight / (height * height)
    var weight = 70.0      // kg
    var height_m = 1.75    // mét
    var bmi = weight / (height_m * height_m)
    println("Chỉ số BMI: " + bmi)  // ~22.86
}

main()
```

---

## 6. So Sánh Số

```zenith
func main() {
    var a = 10
    var b = 20
    
    println(a == b)  // false (bằng?)
    println(a != b)  // true (khác?)
    println(a < b)   // true (nhỏ hơn?)
    println(a > b)   // false (lớn hơn?)
    println(a <= b)  // true (nhỏ hơn hoặc bằng?)
    println(a >= b)  // false (lớn hơn hoặc bằng?)
}

main()
```

---

## 7. Bài Tập

### Bài 1: Máy Tính Đơn Giản
Tạo chương trình nhập 2 số và in ra tất cả phép toán:
```zenith
func main() {
    var num1 = 15
    var num2 = 4
    
    // In ra: cộng, trừ, nhân, chia, chia dư
}

main()
```

### Bài 2: Chuyển Đổi Đơn Vị
Viết chương trình chuyển đổi:
- Km sang Miles (1 km = 0.621371 miles)
- °C sang °F (F = C * 9/5 + 32)

### Bài 3: Tính Diện Tích
Viết chương trình tính:
- Diện tích hình vuông (cạnh = 5)
- Diện tích hình chữ nhật (rộng = 4, dài = 7)
- Diện tích hình tam giác (đáy = 6, cao = 8)
  - Công thức: area = (base * height) / 2

### Bài 4: Tính Tiền
Một món hàng giá 250,000đ, giảm 15%, thuế 10%.
Tính giá cuối cùng phải trả.

---

## 8. Lỗi Thường Gặp

❌ **Chia cho 0**:
```zenith
var x = 10 / 0  // ERROR!
```

❌ **Quên ngoặc đơn**:
```zenith
var wrong = 10 + 5 * 2    // = 20
var correct = (10 + 5) * 2  // = 30
```

❌ **Nhầm lẫn int vs float**:
```zenith
var a = 7 / 2      // = 3 (integer division)
var b = 7.0 / 2.0  // = 3.5 (float division)
```

---

## 9. Công Thức Hữu Ích

| Công thức | Code |
|-----------|------|
| Trung bình | `(a + b + c) / 3` |
| Tỷ lệ % | `(part / total) * 100` |
| Diện tích hình tròn | `3.14159 * r * r` |
| Chu vi hình tròn | `2 * 3.14159 * r` |
| Pythagoras | `sqrt(a*a + b*b)` |

---

## Tóm Tắt

| Operator | Ý nghĩa | Ví dụ |
|----------|---------|-------|
| `+` | Cộng | `10 + 5 = 15` |
| `-` | Trừ | `10 - 5 = 5` |
| `*` | Nhân | `10 * 5 = 50` |
| `/` | Chia | `10 / 5 = 2` |
| `%` | Chia dư | `10 % 3 = 1` |
| `==` | So sánh bằng | `5 == 5 → true` |
| `!=` | So sánh khác | `5 != 3 → true` |
| `<` | Nhỏ hơn | `3 < 5 → true` |
| `>` | Lớn hơn | `5 > 3 → true` |

---

## Bài Tiếp Theo
👉 [Bài 4: Chuỗi Ký Tự](04_strings.md)
