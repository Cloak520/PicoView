<script lang="ts">
  let {
    zoom = 1,
    rotation = 0,
    isStarred = false,
    showExif = false,
    onZoomIn,
    onZoomOut,
    onZoomReset,
    onRotateLeft,
    onRotateRight,
    onToggleStar,
    onToggleExif,
    onDelete,
    onBackToLibrary,
  }: {
    zoom: number
    rotation: number
    isStarred: boolean
    showExif: boolean
    onZoomIn: () => void
    onZoomOut: () => void
    onZoomReset: () => void
    onRotateLeft: () => void
    onRotateRight: () => void
    onToggleStar: () => void
    onToggleExif: () => void
    onDelete: () => void
    onBackToLibrary: () => void
  } = $props()
</script>

<div class="fixed bottom-6 left-1/2 -translate-x-1/2 flex items-center gap-1.5 px-4 py-2 bg-neutral-900/85 backdrop-blur-xl border border-neutral-700/50 rounded-full shadow-2xl z-40 text-neutral-300">
  <!-- Back to Library -->
  <button
    onclick={onBackToLibrary}
    title="返回相册网格 (Esc / L)"
    class="p-2 hover:bg-neutral-800 rounded-full hover:text-white transition"
  >
    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
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

  <!-- Star Favorite -->
  <button
    onclick={onToggleStar}
    title="收藏 / 星标 (S)"
    class="p-2 hover:bg-neutral-800 rounded-full transition {isStarred ? 'text-amber-400' : 'hover:text-white'}"
  >
    <svg class="w-4 h-4 {isStarred ? 'fill-current' : ''}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z" />
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
