# 🌐 Gemini Translator — Trợ lý AI Dịch thuật & Phân tích Đa nền tảng

[![Framework](https://img.shields.io/badge/Framework-Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=white)](#)
[![Backend](https://img.shields.io/badge/Backend-Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](#)
[![Frontend](https://img.shields.io/badge/Frontend-TypeScript%20%7C%20HTML%20%7C%20CSS-3178C6?style=for-the-badge&logo=typescript&logoColor=white)](#)
[![AI](https://img.shields.io/badge/AI-Gemini%202.5%20Flash%2FLite-8E75B2?style=for-the-badge&logo=googlebard&logoColor=white)](#)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-0078D6?style=for-the-badge&logo=windows&logoColor=white)](#)

[![GitHub](https://img.shields.io/badge/GitHub-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/anhnn83/geminitranslator)

> **Gemini Translator** là một ứng dụng Desktop siêu nhẹ được xây dựng trên nền tảng Tauri và sức mạnh của Google Gemini API. Ứng dụng cho phép bạn dịch văn bản bôi đen, trích xuất chữ từ ảnh chụp màn hình (OCR) và đặc biệt là tính năng phân tích mã nguồn/log lỗi hệ thống với tốc độ phản hồi cực nhanh, mọi lúc mọi nơi ngay trên màn hình của bạn.  
> *Dự án được thiết kế tối ưu hóa cho trải nghiệm thao tác một chạm bằng phím tắt toàn cầu (Global Shortcuts).*

---

> 💡 **Tip cho Coder:** Hãy copy-paste toàn bộ nội dung tài liệu này cho một AI (ChatGPT/Gemini/Claude...) để nó có ngữ cảnh và giúp bạn gỡ lỗi, deploy dự án hoặc phát triển tính năng mới nhanh chóng hơn!

---

## ✨ 1. Giới thiệu & Các tính năng cốt lõi

Dự án này là một "vũ khí bí mật" thay thế cho các ứng dụng từ điển hay công cụ dịch thuật cồng kềnh, giúp bạn tối ưu hóa hiệu suất làm việc bằng cách gắn kết trực tiếp AI vào mọi thao tác trên hệ điều hành.

**🚀 Tính năng**

* **Siêu nhẹ & Native:** Xây dựng bằng Rust (Tauri) thay vì Electron, giúp ứng dụng tiêu tốn cực ít bộ nhớ (RAM) và khởi động nền gần như tức thì.
* **Dịch Văn Bản "1 Chạm":** Chỉ cần bôi đen bất kỳ đoạn text nào trên màn hình, bấm phím tắt, popup dịch thuật sẽ hiện ra ngay tại vị trí con trỏ chuột.
* **Dịch Ảnh (OCR) Thần Tốc:** Hỗ trợ nhận diện chữ trên ảnh cực mạnh bằng mô hình Gemini Vision. Tự động tương tác với Snipping Tool (Windows) hoặc Flameshot/Spectacle (Linux) để lấy ảnh từ Clipboard và dịch mà không xả file rác ra ổ cứng.
* **Chế độ [Pro] Code/Terminal Analyzer:** Không chỉ là dịch thuật! Khi bật chế độ này, AI sẽ hóa thân thành System Admin/Developer để phân tích log lỗi Terminal đỏ ngầu, giải thích code snippet phức tạp, và đề xuất giải pháp fix bug trực tiếp.
* **Tùy biến Phím Tắt (Dynamic Shortcuts):** Lắng nghe và gán bất kỳ tổ hợp phím nào bạn muốn (VD: `Super+Alt+C`) ngay trong Runtime mà không sợ đụng độ với hệ điều hành.
* **Trải nghiệm UX/UI Hiện Đại:** Cửa sổ Popup xây dựng bằng Flexbox tự động co giãn, hỗ trợ ghim (Pin) trên cùng, lưu cấu hình LocalStorage và tự động biến mất thông minh khi click ra ngoài.

**👥 Đối tượng sử dụng**

* Lập trình viên, Quản trị viên hệ thống (SysAdmin) thường xuyên phải đọc tài liệu, tra cứu log lỗi trên hệ thống Linux/Windows.
* Người dùng cần một công cụ OCR trích xuất văn bản từ hình ảnh/PDF/Video nhanh chóng, phân tách rõ ràng song ngữ gốc - dịch.
* Những ai yêu thích các công cụ Minimalist (tối giản), cấu hình 1 lần và thao tác 100% bằng bàn phím.

---

## 🛠️ 2. Môi trường chuẩn bị

Nếu bạn chỉ muốn sử dụng, hãy tải bản cài đặt `.exe` hoặc `.deb` trong mục **Releases**.  
Nếu bạn muốn tự Build mã nguồn (Compile) trên máy của mình, bạn cần chuẩn bị:

1. **Gemini API Key:** Lấy hoàn toàn miễn phí tại [Google AI Studio](https://aistudio.google.com/).
2. **Node.js & npm:** Để quản lý các gói của Frontend.
3. **Rust (Cargo):** Ngôn ngữ lõi để biên dịch hệ thống Backend.
4. **Môi trường Linux (Nếu build trên Linux):** Bắt buộc cài đặt các thư viện `libwebkit2gtk-4.1-dev`, `build-essential` và các công cụ quản lý clipboard/chụp ảnh như `xclip`, `flameshot` (hoặc `gnome-screenshot`).

---

## 🚀 3. Hướng dẫn biên dịch (Build / Deploy)

Thực hiện tuần tự theo các bước dưới đây để khởi chạy môi trường Dev hoặc Build đóng gói ứng dụng:

**Bước 3.1: Tải mã nguồn về máy**  
Mở Terminal và chạy lệnh:
```bash
git clone https://github.com/anhnn83/geminitranslator.git
cd geminitranslator
```

**Bước 3.2: Cài đặt Dependencies**  
```bash
npm install
```

**Bước 3.3: Chạy môi trường phát triển (Dev Mode)**  
```bash
npm run tauri dev
```

Lệnh này sẽ khởi chạy Vite (Frontend) và biên dịch nóng Rust (Backend). Bất kỳ thay đổi nào trong code (kể cả Rust hay JS/CSS) sẽ được cập nhật ngay lập tức lên giao diện.

**Bước 3.4: Đóng gói ứng dụng (Production Build)**  
```bash
npm run tauri build
```

Rust sẽ thực hiện quá trình tối ưu hóa toàn diện (Release mode). Sau khi chạy xong (khoảng 3-5 phút), file cài đặt (`.exe`, `.msi`, `.deb` hoặc `.AppImage`) sẽ nằm gọn gàng trong thư mục `src-tauri/target/release/bundle/`.

* 💡 **Khuyến nghị:** Khuyến khích sử dụng GitHub Actions với file `.gitignore` chuẩn hóa để thiết lập quy trình CI/CD. Việc này giúp tự động Build chéo (Cross-compile) ra nhiều nền tảng Windows/macOS/Linux mỗi khi bạn Push code lên nhánh chính.

---

## 🔑 4. Thiết lập và sử dụng lần đầu

1. Mở ứng dụng, một biểu tượng nhỏ sẽ xuất hiện ở Khay hệ thống (System Tray). Click chuột phải chọn **Settings**.
2. Nhập **Gemini API Key** của bạn.
3. Chọn **Ngôn ngữ đích (Target Language)** (VD: Vietnamese, English...).
4. Click vào các ô input để **Gán phím tắt** cho chức năng Dịch Bôi đen và Dịch Ảnh chụp (Khuyến nghị dùng `Super+Alt+X` và `Super+Alt+C` để tránh đụng chạm phím Copy/Paste hệ thống).
5. *(Tùy chọn)* Tick vào ô **Start with Windows/Linux** để app tự chạy cùng hệ thống (Đã liên kết qua Registry/Autostart).
6. *(Tùy chọn)* Tick vào ô **[Pro] Code/Terminal Analyzer Mode** nếu bạn muốn AI đóng vai trò là chuyên gia phân tích kỹ thuật thay vì chỉ dịch thuật thông thường.
7. Bấm **Save Settings** và trải nghiệm "Phép thuật" AI ngay trên màn hình của bạn!

---

## ☎️ 5. Đóng góp & Mã nguồn mở

**Mọi đóng góp, tối ưu code & báo lỗi đều được hoan nghênh tại kho lưu trữ chính thức!**

**👨‍💻 Dev by ANHNN**

[![Telegram](https://img.shields.io/badge/Telegram-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white)](https://t.me/anhnn83)
[![Email](https://img.shields.io/badge/Email-D14836?style=for-the-badge&logo=gmail&logoColor=white)](mailto:anhnn@dgd.vn)
[![website](https://img.shields.io/badge/Website-anhnn.cronpost.com-181717?style=for-the-badge&logo=google-chrome&logoColor=white)](https://anhnn.cronpost.com)

<hr>
<div align="center">
  &copy; 2026 <a href="https://github.com/anhnn83">anhnn</a>. Mọi quyền được bảo lưu.<br>
  <b>Gemini Translator</b> được phát hành dưới giấy phép <a href="LICENSE">GNU GPLv3</a>.
</div>