# PicoView (跨平台极速看图与相册管理工具)

> 灵感源自经典 **Google Picasa 3**，采用 **Rust + Tauri v2 + Svelte 5** 构建的极轻量、秒开、低内存占用跨平台照片查看器。

---

## 🌟 核心特性与设计理念

1. **极致轻量与秒开**：
   - 弃用臃肿的 Chromium 内核打包（对比常规 Electron 节省 90% 体积与内存）。
   - 前端采用 **Svelte 5** 纯编译型响应式机制（无虚拟 DOM 运行时），打包体积仅数兆，冷启动近乎零延迟。
2. **Rust 原生算力引擎 (`crates/picoview-core`)**：
   - 多线程并发目录遍历（`jwalk`）与硬件加速图片解码（`image` + `fast_image_resize`）。
   - 零拷贝自定义流协议传输本地高分辨率图片。
   - 完整的 EXIF/GPS 元数据解析（`kamadak-exif`）。
   - SQLite 嵌入式缓存与 WAL 高并发模式。
3. **Picasa 经典看图体验**：
   - 深色磨砂背景、居中视口。
   - 鼠标滚轮平滑缩放、双击适应/放大、拖拽平移。
   - 键盘快速翻页（`Space` / 左右方向键）、旋转（`R`）、星标收藏（`S`）、属性面板（`I`）。
   - 左右相邻照片预测式静默预加载。

---

## 📁 目录架构

```text
PicoView/
├── Cargo.toml                    # Rust Workspace 根配置
├── crates/
│   └── picoview-core/            # 核心业务引擎（无 UI 依赖，可独立单元测试）
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── models.rs         # 核心数据模型 (MediaItem, ExifData, Recipe)
│           ├── scanner.rs        # 并发目录扫描与文件过滤
│           ├── decoder.rs        # 快速规格探测与解码
│           ├── thumbnail.rs      # SIMD 缩略图生成与缓存
│           ├── exif.rs           # EXIF / 拍摄参数解析
│           └── db.rs             # SQLite 索引与配方持久化
├── src-tauri/                    # Tauri v2 桌面宿主
│   ├── Cargo.toml
│   ├── tauri.conf.json           # 窗口尺寸、协议与权限配置
│   ├── capabilities/             # Tauri 权限清单
│   └── src/
│       ├── lib.rs                # Tauri IPC Command 接口
│       └── main.rs
└── ui/                           # Svelte 5 + TailwindCSS 前端工程
    ├── package.json
    ├── vite.config.ts
    ├── svelte.config.js
    └── src/
        ├── App.svelte            # 视图模式调度器
        ├── components/
        │   ├── Viewer.svelte     # 核心看图视口 (平移缩放翻页)
        │   ├── Welcome.svelte    # 极简起始页 (拖拽投递与近期记录)
        │   ├── Filmstrip.svelte  # 底部胶片导轨缩略图
        │   ├── Toolbar.svelte    # 经典悬浮毛玻璃底栏
        │   └── ExifPanel.svelte  # 拍摄参数悬浮抽屉
        └── lib/
            ├── api.ts            # Tauri IPC 客户端与图片协议桥接
            └── types.ts          # 前端 TypeScript 类型定义
```

---

## 🚀 启动与开发

### 环境要求
- **Rust**: 1.78+ (`cargo --version`)
- **Node.js**: v20+ (`node -v`)

### 1. 启动纯前端预览（快速调试 UI）
```bash
cd ui
npm install
npm run dev
```

### 2. 启动桌面完整应用 (Tauri 开发模式)
```bash
# 确保已安装 cargo-tauri CLI: cargo install tauri-cli --version "^2.0.0"
cargo tauri dev
```

### 3. 构建跨平台发布包
```bash
cargo tauri build
```
输出位于 `target/release/bundle/`：
- **macOS**: `.dmg` / `.app`
- **Windows**: `.msi` / `.exe`
- **Linux**: `.AppImage` / `.deb`
