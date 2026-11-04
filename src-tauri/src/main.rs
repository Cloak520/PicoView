// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "windows")]
    {
        // 针对 Windows WebView2 启动内存占用深度优化：
        // 1. --renderer-process-limit=1: 限制渲染进程为 1，防止多渲染进程膨胀
        // 2. --js-flags=--max-old-space-size=128: 限制 V8 堆内存上限为 128MB，促使其更积极回收垃圾
        // 3. --disable-features=IsolateOrigins,site-per-process: 禁用跨域隔离多进程派生
        // 4. --disk-cache-size / --media-cache-size: 压缩缓存上限为 10MB
        // 5. 禁用 Edge 默认扩展、同步、遥测与后台服务
        let low_memory_args = [
            "--js-flags=--max-old-space-size=128",
            "--disable-features=IsolateOrigins,site-per-process,AudioServiceOutOfProcess",
            "--renderer-process-limit=1",
            "--disable-gpu-shader-disk-cache",
            "--disk-cache-size=10485760",
            "--media-cache-size=10485760",
            "--disable-background-networking",
            "--disable-component-update",
            "--disable-default-apps",
            "--disable-extensions",
            "--disable-sync",
        ]
        .join(" ");

        if let Ok(existing) = std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS") {
            std::env::set_var(
                "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
                format!("{} {}", existing, low_memory_args),
            );
        } else {
            std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", low_memory_args);
        }
    }

    picoview_lib::run();
}
