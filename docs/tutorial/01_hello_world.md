# Zenith Tutorial - Bài 1: Chương Trình Đầu Tiên

## Mục tiêu
Trong bài này, bạn sẽ:
- Viết chương trình Zenith đầu tiên
- Hiểu cấu trúc cơ bản của một file `.zn`
- Chạy chương trình và xem kết quả

---

## 1. Hello World!

Tạo file `hello.zn`:

```zenith
// Đây là comment (chú thích)
func main() {
    println("Xin chào Zenith!")
}

main()
```

**Giải thích**:
- `//` là comment - dòng này sẽ bị bỏ qua khi chạy
- `func main()` định nghĩa một hàm tên là `main`
- `println()` in ra màn hình và xuống dòng
- `main()` gọi hàm main để chạy chương trình

---

## 2. Chạy chương trình

### Linux/macOS:
```bash
./zenith run hello.zn
```

### Windows:
```powershell
.\zenith.exe run hello.zn
```

**Kết quả**:
```
Xin chào Zenith!
```

---

## 3. In nhiều dòng

```zenith
func main() {
    println("=== Zenith Programming Language ===")
    println("Chào mừng bạn đến với Zenith!")
    println("Bắt đầu học lập trình thôi!")
    println("===================================")
}

main()
```

---

## 4. Sử dụng `print` thay vì `println`

```zenith
func main() {
    print("Hello ")
    print("World")
    println("!")  // println xuống dòng sau khi in
}

main()
```

**Kết quả**:
```
Hello World!
```

---

## 5. Bài tập

### Bài 1: Tự giới thiệu
Viết chương trình in ra:
- Tên bạn
- Tuổi
- Sở thích

### Bài 2: Vẽ hình
Sử dụng `println` để vẽ một hình vuông bằng ký tự `*`:
```
*****
*   *
*   *
*   *
*****
```

### Bài 3: Thử nghiệm
Thử thay đổi nội dung trong `println()` và xem chương trình chạy như thế nào!

---

## 6. Lỗi thường gặp

❌ **Quên dấu ngoặc kép**:
```zenith
println(Hello)  // SAI!
```
✅ **Đúng**:
```zenith
println("Hello")
```

❌ **Quên gọi hàm main**:
```zenith
func main() {
    println("Test")
}
// Thiếu: main()
```

---

## Bài tiếp theo
👉 [Bài 2: Biến và Kiểu Dữ Liệu](02_variables.md)

---

**Chúc mừng bạn đã hoàn thành bài học đầu tiên! 🎉**
