# PicoView 📷

> 灵感源自经典 **Google Picasa Photo Viewer**。采用 **Rust + Tauri v2 + Svelte 5** 构建的极轻量、秒开、无边框全屏遮罩照片查看器。

---

## 🌟 核心特性与设计理念

PicoView 摒弃了传统看图软件沉重的“相册大厅”与数据库索引机制，专注于**“即点即看、极速纯粹”**的系统级看图体验：

1. **全屏遮罩 Lightbox 体验（告别虚拟桌面切换）**：
   - 采用原生级无边框屏幕遮罩（Borderless Overlay Mask），**完全留在当前桌面工作区**，杜绝 macOS 原生 Spaces 全屏长达 500ms 的滑屏顿挫。
   - 经典暗色毛玻璃幕布（`rgba(13, 13, 16, 0.90)` + `backdrop-blur`），柔和暗化桌面并突出焦点照片。
   - 支持瞬时退出（`Esc` 键或右上角快速关闭），随开随走。

2. **零黑屏、零延迟（1ms 极致磁盘响应）**：
   - Rust 后端专为同级照片扫描优化，仅读取元数据，彻底剥离阻塞式磁盘探测，扫描百张同级照片仅耗时约 **1ms**。
   - 依赖本地系统 WebKit / WebView2 / Blink 硬件 GPU 解码管线与零拷贝自定义流协议，实现秒开大图与顺滑 60fps 缩放平移。

3. **零拖影、连续丝滑操作**：
   - **平移拖拽**：基于 GPU 硬件加速的 `translate3d` 与动态过渡切换，彻底消除鼠标拖动时的延迟与拖影。
   - **连续无损旋转**：角度采用平滑连续累加计算，告别 360° 旋转归零时的反向倒转动画。
   - **胶片导轨抽屉**：按 `T` 键可随时展开/收起底部悬浮胶片导轨，在同级照片间快速跳转。

4. **超轻量工程架构**：
   - 剔除传统 Chromium 打包（相较 Electron 内存占用降低 80%+，安装包仅数兆）。
   - 前端采用 **Svelte 5 Runes**（纯编译型响应式，无虚拟 DOM 开销）。
   - 后端仅保留核心 EXIF 解析与安全回收站调用，无冗余 C 语言 SQLite 依赖，冷启动与编译飞速。

---

## ⌨️ 快捷键速查表

| 操作 | 快捷键 / 鼠标动作 |
| :--- | :--- |
| **退出 / 返回** | `Esc` / 点击右上角 `✕` / 底栏关闭按钮 |
| **下一张 / 上一张** | `→` / `Space` / `PageDown` 或 `←` / `PageUp` / 侧边导航箭头 |
| **放大 / 缩小** | 鼠标滚轮 / `+` / `-` / 底栏缩放按钮 |
| **适应屏幕 / 1:1 切换** | `0` / `F` / 鼠标双击图片 |
| **顺时针 / 逆时针旋转** | `R` / `Shift + R` |
| **胶片导轨（缩略图抽屉）** | `T`（展开 / 收起） |
| **EXIF 拍摄参数面板** | `I`（打开 / 关闭） |
| **打开新图片** | `⌘O`（macOS）/ `Ctrl + O`（Windows/Linux） |
| **移入系统回收站** | `Delete` / `Backspace` |

---

## 📁 目录架构

```text
PicoView/
├── Cargo.toml                    # Rust Workspace 根配置
├── crates/
│   └── picoview-core/            # 极简核心库 (无 UI 依赖)
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs            # 模块导出定义
│           ├── models.rs         # 核心模型 (MediaItem, ExifData)
│           ├── scanner.rs        # 1ms 秒级同级目录扫描与排序
│           └── exif.rs           # EXIF / 拍摄参数解析 (kamadak-exif)
├── src-tauri/                    # Tauri v2 桌面宿主
│   ├── Cargo.toml
│   ├── tauri.conf.json           # 无边框全屏遮罩、协议与关联格式配置
│   ├── capabilities/             # Tauri 权限清单 (core, dialog)
│   └── src/
│       ├── lib.rs                # 6 个核心原子级 IPC 命令与屏幕贴合调度
│       └── main.rs
└── ui/                           # Svelte 5 + TailwindCSS 前端工程
    ├── package.json
    ├── vite.config.ts
    ├── svelte.config.js
    └── src/
        ├── App.svelte            # 单 IPC 启动分发与遮罩根视口
        ├── components/
        │   ├── Viewer.svelte     # 核心看图视口 (平移缩放翻页旋转)
        │   ├── Welcome.svelte    # 极简起始页 (拖拽投递与近期记录)
        │   ├── Filmstrip.svelte  # 底部悬浮胶片导轨
        │   ├── Toolbar.svelte    # 经典悬浮毛玻璃控制底栏
        │   └── ExifPanel.svelte  # 拍摄参数悬浮抽屉
        └── lib/
            ├── api.ts            # Tauri IPC 客户端与图片协议桥接
            └── types.ts          # 前端 TypeScript 类型定义
```

---

## 🚀 启动与构建

### 环境要求
- **Rust**: 1.78+ (`cargo --version`)
- **Node.js**: v20+ (`node -v`)

### 1. 本地开发调试
```bash
# 1. 安装前端依赖
cd ui && npm install && cd ..

# 2. 启动 Tauri 开发环境 (自动编译 Rust 与前端 Vite 热重载)
cargo tauri dev
```

直接带参数测试查看单张照片：
```bash
cargo tauri dev -- /path/to/your/photo.jpg
```

### 2. 构建跨平台安装包
```bash
cargo tauri build
```
输出位于 `target/release/bundle/`：
- **macOS**: `.dmg` / `.app`（已配置图片文件格式关联）
- **Windows**: `.msi` / `.exe`
- **Linux**: `.AppImage` / `.deb`

---

## 📄 关联打开支持格式

- `.jpg`, `.jpeg`, `.png`, `.webp`, `.gif`, `.bmp`, `.tiff`, `.tif`, `.avif`
- 支持系统文件右键“使用 PicoView 打开”直接全屏秒开。
