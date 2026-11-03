<script lang="ts">
  let {
    zoom = 1,
    rotation = 0,
    showExif = false,
    showFilmstrip = false,
    isExternalOpen = false,
    onZoomIn,
    onZoomOut,
    onZoomReset,
    onRotateLeft,
    onRotateRight,
    onToggleExif,
    onToggleFilmstrip,
    onCopy,
    onDelete,
    onClose,
  }: {
    zoom: number
    rotation: number
    showExif: boolean
    showFilmstrip: boolean
    isExternalOpen?: boolean
    onZoomIn: () => void
    onZoomOut: () => void
    onZoomReset: () => void
    onRotateLeft: () => void
    onRotateRight: () => void
    onToggleExif: () => void
    onToggleFilmstrip: () => void
    onCopy?: () => void
    onDelete: () => void
    onClose: () => void
  } = $props()
</script>

<div class="fixed bottom-6 left-1/2 -translate-x-1/2 flex items-center gap-1 px-3 py-1.5 bg-neutral-900/85 backdrop-blur-xl border border-neutral-700/50 rounded-full shadow-2xl z-40 text-neutral-300">
  <!-- Exit / Back button -->
  <button
    onclick={onClose}
    title={isExternalOpen ? '关闭应用 (Esc)' : '退出浏览 (Esc)'}
    class="p-2 hover:bg-neutral-800 rounded-full hover:text-white transition"
  >
    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
    </svg>
  </button>

  <div class="w-[1px] h-4 bg-neutral-700/60 mx-1"></div>

  <!-- Zoom Controls -->
  <button
    onclick={onZoomOut}
    title="缩小 (-)"
    class="p-2 hover:bg-neutral-800 rounded-full hover:text-white transition"
  >
    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
    </svg>
  </button>

  <button
    onclick={onZoomReset}
    title="适应屏幕 / 1:1 (0 / 双击)"
    class="px-2.5 py-1 text-xs font-mono font-medium hover:bg-neutral-800 rounded-md hover:text-white transition"
  >
    {Math.round(zoom * 100)}%
  </button>

  <button
    onclick={onZoomIn}
    title="放大 (+)"
    class="p-2 hover:bg-neutral-800 rounded-full hover:text-white transition"
  >
    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
    </svg>
  </button>

  <div class="w-[1px] h-4 bg-neutral-700/60 mx-1"></div>

  <!-- Rotate Controls -->
  <button
    onclick={onRotateLeft}
    title="逆时针旋转 90° (Shift+R)"
    class="p-2 hover:bg-neutral-800 rounded-full hover:text-white transition"
  >
    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a5 5 0 0 1 5 5v2m0 0l-3-3m3 3l3-3M3 10l3-3M3 10l3 3" />
    </svg>
  </button>

  <button
    onclick={onRotateRight}
    title="顺时针旋转 90° (R)"
    class="p-2 hover:bg-neutral-800 rounded-full hover:text-white transition"
  >
    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 10H11a5 5 0 0 0-5 5v2m0 0l3-3m-3 3l-3-3m18-4l-3-3m3 3l-3 3" />
    </svg>
  </button>

  <div class="w-[1px] h-4 bg-neutral-700/60 mx-1"></div>

  <!-- Toggle Filmstrip Thumbnail Drawer -->
  <button
    onclick={onToggleFilmstrip}
    title="缩略图胶片导轨 (T)"
    class="p-2 hover:bg-neutral-800 rounded-full transition {showFilmstrip ? 'text-blue-400 bg-neutral-800' : 'hover:text-white'}"
  >
    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7" />
    </svg>
  </button>

  <!-- EXIF Info Toggle -->
  <button
    onclick={onToggleExif}
    title="属性与 EXIF (I)"
    class="p-2 hover:bg-neutral-800 rounded-full transition {showExif ? 'text-blue-400 bg-neutral-800' : 'hover:text-white'}"
  >
    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  </button>

  <!-- Copy to Clipboard -->
  <button
    onclick={onCopy}
    title="复制图片到剪贴板 (Cmd+C)"
    class="p-2 hover:bg-neutral-800 rounded-full hover:text-white transition"
  >
    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2" />
    </svg>
  </button>

  <div class="w-[1px] h-4 bg-neutral-700/60 mx-1"></div>

  <!-- Delete -->
  <button
    onclick={onDelete}
    title="移入回收站 (Delete / Backspace)"
    class="p-2 hover:bg-red-950/60 hover:text-red-400 rounded-full transition"
  >
    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
    </svg>
  </button>
</div>
