# 🖼️ Polygon Scanline Filler (Rust + Raylib)

This project implements a **scan-line polygon fill algorithm** in Rust using the [Raylib](https://www.raylib.com/) library. It supports filling multiple complex polygons, including polygons with **holes**. The result is rendered directly onto an image and saved as `out.png`.

---

## 📌 Features

- ✅ Fill convex and concave polygons
- ✅ Support for holes (non-filled inner polygons)
- ✅ Custom fill and outline colors
- ✅ Cartesian coordinate system (Y-axis grows upward)
- ✅ Exports to `.png` without displaying a window

---

## 🧠 How It Works

- Polygons are defined using a vector of points `(x, y)`.
- All Y-coordinates are adjusted to match a top-left origin image (flipping the Y-axis).
- The **scan-line algorithm** scans each horizontal line and fills between intersection points.
- Points within any hole are **excluded** from the fill.
- After filling, polygon edges are drawn as outlines.
- The final result is saved to a file named `out.png`.

---

## 🛠️ Requirements

- Rust installed: https://www.rust-lang.org/tools/install
- `raylib` and `raylib-sys` dependencies
- C compiler (GCC, Clang, etc.)

Install Raylib with:
```sh
brew install raylib          # macOS
sudo apt install libraylib-dev  # Ubuntu/Debian
```

## 🚀 Running the Project

1. Build the project:
```sh
cargo build
```

2. Run the project:
```sh
cargo run
```

The program will generate an `out.png` file with the filled polygon.


