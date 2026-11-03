<script lang="ts">
  import { onMount } from 'svelte'
  import { exitApp, toAssetUrl } from '../lib/api'

  export interface RecentItem {
    path: string
    filename: string
    timestamp: number
  }

  let {
    recents = [],
    onOpenPhoto,
    onPickFile,
    onClearRecents,
  }: {
    recents: RecentItem[]
    onOpenPhoto: (path: string) => void
    onPickFile: () => void
    onClearRecents: () => void
  } = $props()

  function formatTime(ts: number) {
    const diff = Date.now() - ts
    const mins = Math.floor(diff / 60000)
    if (mins < 1) return '刚刚'
    if (mins < 60) return `${mins} 分钟前`
    const hours = Math.floor(mins / 60)
    if (hours < 24) return `${hours} 小时前`
    const days = Math.floor(hours / 24)
    return `${days} 天前`
  }

  onMount(() => {
    function handleKeyDown(e: KeyboardEvent) {
      if (e.key === 'Escape') {
        e.preventDefault()
        exitApp()
      }
    }
    window.addEventListener('keydown', handleKeyDown)
    return () => {
      window.removeEventListener('keydown', handleKeyDown)
    }
  })
</script>

<div class="relative w-full h-full flex flex-col items-center justify-center p-8 select-none text-neutral-300">
  <!-- Top Right Close Button -->
  <button
    onclick={() => exitApp()}
    class="absolute top-5 right-5 p-2 rounded-full text-neutral-400 hover:text-white hover:bg-white/10 transition z-20"
    title="退出应用 (Esc)"
  >
    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
    </svg>
  </button>
  <!-- Minimalist Centered Card -->
  <div class="flex flex-col items-center max-w-lg w-full text-center">
    <!-- Picasa-style colorful aperture logo icon -->
    <div class="w-20 h-20 rounded-3xl bg-neutral-900/90 border border-neutral-700/60 p-4 shadow-2xl flex items-center justify-center mb-6 group hover:scale-105 transition-transform duration-200">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="w-full h-full drop-shadow">
        <g transform="translate(256, 256)">
          <path d="M 0,0 L 40,-130 A 140 140 0 0 1 120,-70 Z" fill="#ef4444" opacity="0.95" />
          <path d="M 0,0 L 120,-70 A 140 140 0 0 1 140,20 Z" fill="#f59e0b" opacity="0.95" />
          <path d="M 0,0 L 140,20 A 140 140 0 0 1 60,125 Z" fill="#10b981" opacity="0.95" />
          <path d="M 0,0 L 60,125 A 140 140 0 0 1 -60,125 Z" fill="#06b6d4" opacity="0.95" />
          <path d="M 0,0 L -60,125 A 140 140 0 0 1 -140,20 Z" fill="#3b82f6" opacity="0.95" />
          <path d="M 0,0 L -140,20 A 140 140 0 0 1 -120,-70 Z" fill="#8b5cf6" opacity="0.95" />
          <path d="M 0,0 L -120,-70 A 140 140 0 0 1 40,-130 Z" fill="#ec4899" opacity="0.95" />
          <circle r="36" fill="#141416" stroke="#ffffff" stroke-width="3" stroke-opacity="0.2" />
          <circle r="16" fill="#3b82f6" opacity="0.8" />
        </g>
      </svg>
    </div>

    <h1 class="text-xl font-bold tracking-tight text-white mb-2">PicoView</h1>
    <p class="text-xs text-neutral-400 mb-6 max-w-xs leading-relaxed">
      极速轻量的系统看图工具。支持滚轮缩放、左右翻页与胶片导轨。
    </p>

    <!-- Shortcut and trigger pills -->
    <div class="flex items-center gap-3 mb-10">
      <button
        onclick={onPickFile}
        class="px-4 py-2 rounded-xl bg-neutral-800 hover:bg-neutral-700/80 border border-neutral-700 text-white text-xs font-medium transition shadow-sm hover:shadow flex items-center gap-2"
      >
        <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
        选择图片
        <kbd class="ml-1 px-1.5 py-0.5 bg-neutral-900 border border-neutral-700/80 rounded text-[10px] text-neutral-400 font-mono">⌘O</kbd>
      </button>

      <span class="text-xs text-neutral-500">或直接将图片拖入此窗口</span>
    </div>

    <!-- Recent History Section -->
    {#if recents.length > 0}
      <div class="w-full pt-6 border-t border-neutral-800/80 text-left">
        <div class="flex items-center justify-between mb-3 px-1">
          <span class="text-xs font-semibold text-neutral-400">近期浏览</span>
          <button
            onclick={onClearRecents}
            class="text-[11px] text-neutral-500 hover:text-neutral-400 transition"
          >
            清空记录
          </button>
        </div>

        <div class="grid grid-cols-3 gap-2.5">
          {#each recents.slice(0, 6) as item (item.path)}
            <button
              onclick={() => onOpenPhoto(item.path)}
              class="group relative flex items-center gap-2.5 p-2 rounded-xl bg-neutral-900/60 hover:bg-neutral-800/80 border border-neutral-800 hover:border-neutral-700 transition text-left overflow-hidden"
              title={item.path}
            >
              <div class="w-10 h-10 rounded-lg bg-neutral-950 overflow-hidden flex-shrink-0 flex items-center justify-center border border-neutral-800/60">
                <img
                  src={toAssetUrl(item.path)}
                  alt={item.filename}
                  loading="lazy"
                  class="w-full h-full object-cover group-hover:scale-105 transition-transform"
                />
              </div>
              <div class="flex-1 min-w-0">
                <div class="text-xs text-neutral-200 font-medium truncate group-hover:text-blue-400 transition-colors">
                  {item.filename}
                </div>
                <div class="text-[10px] text-neutral-500">
                  {formatTime(item.timestamp)}
                </div>
              </div>
            </button>
          {/each}
        </div>
      </div>
    {/if}
  </div>

  <!-- Bottom subtle hint -->
  <div class="absolute bottom-6 text-[11px] text-neutral-600 flex items-center gap-4">
    <span>按 <kbd class="px-1 py-0.5 bg-neutral-900 rounded border border-neutral-800 text-neutral-400 font-mono">Space</kbd> / <kbd class="px-1 py-0.5 bg-neutral-900 rounded border border-neutral-800 text-neutral-400 font-mono">←</kbd> <kbd class="px-1 py-0.5 bg-neutral-900 rounded border border-neutral-800 text-neutral-400 font-mono">→</kbd> 翻页</span>
    <span>•</span>
    <span>按 <kbd class="px-1 py-0.5 bg-neutral-900 rounded border border-neutral-800 text-neutral-400 font-mono">T</kbd> 呼出胶片栏</span>
    <span>•</span>
    <span>按 <kbd class="px-1 py-0.5 bg-neutral-900 rounded border border-neutral-800 text-neutral-400 font-mono">Esc</kbd> 退出</span>
  </div>
</div>
