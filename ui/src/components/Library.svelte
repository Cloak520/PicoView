<script lang="ts">
  import type { MediaItem } from '../lib/types'
  import { toAssetUrl } from '../lib/api'

  let {
    items = $bindable([]),
    currentFolderPath = $bindable(''),
    isLoading = false,
    onSelectPhoto,
    onOpenPath,
  }: {
    items: MediaItem[]
    currentFolderPath: string
    isLoading?: boolean
    onSelectPhoto: (items: MediaItem[], index: number) => void
    onOpenPath: (path: string) => void
  } = $props()

  let filterStarred = $state(false)
  let searchQuery = $state('')
  let thumbSize = $state(180) // 120px to 320px

  const filteredItems = $derived(
    items.filter((item) => {
      if (filterStarred && !item.is_starred) return false
      if (searchQuery && !item.filename.toLowerCase().includes(searchQuery.toLowerCase())) {
        return false
      }
      return true
    })
  )

  async function handleOpenSingleImage() {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog')
      const selected = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: '图片文件',
            extensions: ['jpg', 'jpeg', 'png', 'webp', 'gif', 'bmp', 'tiff', 'tif', 'avif'],
          },
        ],
        title: '选择图片文件',
      })
      if (selected && typeof selected === 'string') {
        onOpenPath(selected)
      }
    } catch {
      const path = prompt('请输入本地图片绝对路径:')
      if (path) onOpenPath(path)
    }
  }

  async function handleOpenFolder() {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog')
      const selected = await open({
        directory: true,
        multiple: false,
        title: '选择图片文件夹',
      })
      if (selected && typeof selected === 'string') {
        onOpenPath(selected)
      }
    } catch {
      const path = prompt('请输入本地文件夹绝对路径 (例如 /Users/username/Pictures):')
      if (path) onOpenPath(path)
    }
  }
</script>

<div class="w-full h-full flex flex-col bg-[#141416] select-none text-neutral-300">
  <!-- Top Navigation Header -->
  <header class="h-14 border-b border-neutral-800 flex items-center justify-between px-5 bg-neutral-900/40 backdrop-blur-md">
    <div class="flex items-center gap-3">
      <div class="w-7 h-7 rounded-lg bg-gradient-to-tr from-blue-600 to-indigo-500 flex items-center justify-center shadow-lg shadow-blue-500/20 text-white font-bold text-sm">
        P
      </div>
      <span class="font-semibold text-white tracking-wide text-sm">PicoView</span>
      <span class="text-xs text-neutral-500 font-mono ml-2">Picasa 体验版</span>
    </div>

    <!-- Center Search & Filter -->
    <div class="flex items-center gap-3 flex-1 max-w-md mx-6">
      <div class="relative w-full">
        <input
          type="text"
          placeholder="搜索当前相册文件名..."
          bind:value={searchQuery}
          class="w-full bg-neutral-800/80 border border-neutral-700/60 rounded-lg px-3 py-1.5 text-xs text-white placeholder-neutral-500 focus:outline-none focus:border-blue-500 transition"
        />
        {#if searchQuery}
          <button onclick={() => (searchQuery = '')} class="absolute right-2.5 top-1.5 text-neutral-400 hover:text-white text-xs">
            ✕
          </button>
        {/if}
      </div>

      <button
        onclick={() => (filterStarred = !filterStarred)}
        class="px-3 py-1.5 rounded-lg border text-xs flex items-center gap-1.5 transition whitespace-nowrap {filterStarred ? 'bg-amber-500/20 border-amber-500/50 text-amber-300' : 'border-neutral-700/60 hover:bg-neutral-800 text-neutral-400'}"
      >
        ★ 仅看收藏
      </button>
    </div>

    <!-- Actions & Thumbnail Size Slider -->
    <div class="flex items-center gap-3">
      <div class="flex items-center gap-2 mr-1">
        <span class="text-[11px] text-neutral-500">网格</span>
        <input
          type="range"
          min="120"
          max="320"
          step="10"
          bind:value={thumbSize}
          class="w-20 accent-blue-500 h-1 bg-neutral-800 rounded-lg cursor-pointer"
        />
      </div>

      <!-- Open Single Photo -->
      <button
        onclick={handleOpenSingleImage}
        class="px-3 py-1.5 rounded-lg bg-neutral-800 hover:bg-neutral-700 border border-neutral-700/70 text-white text-xs font-medium transition flex items-center gap-1.5"
        title="选择单张图片打开并进入看图模式"
      >
        <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
        打开图片
      </button>

      <!-- Open Folder -->
      <button
        onclick={handleOpenFolder}
        class="px-3.5 py-1.5 rounded-lg bg-blue-600 hover:bg-blue-500 text-white text-xs font-medium shadow-sm transition flex items-center gap-1.5"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
        </svg>
        打开文件夹
      </button>
    </div>
  </header>

  <!-- Folder Path Status Bar -->
  {#if currentFolderPath}
    <div class="px-5 py-2 text-xs border-b border-neutral-800/60 bg-neutral-900/20 flex items-center justify-between text-neutral-400">
      <div class="flex items-center gap-2 truncate">
        <span class="text-neutral-500">当前目录:</span>
        <span class="font-mono text-neutral-300 truncate">{currentFolderPath}</span>
      </div>
      <span class="text-neutral-500 whitespace-nowrap">共 {filteredItems.length} 张照片</span>
    </div>
  {/if}

  <!-- Main Content Grid -->
  <main class="flex-1 overflow-y-auto p-5">
    {#if isLoading}
      <div class="h-full flex flex-col items-center justify-center gap-3 text-neutral-500">
        <div class="w-8 h-8 border-2 border-blue-500/20 border-t-blue-500 rounded-full animate-spin"></div>
        <span class="text-xs">正在极速扫描与索引照片...</span>
      </div>
    {:else if items.length === 0}
      <div class="h-full flex flex-col items-center justify-center text-center max-w-md mx-auto">
        <div class="w-16 h-16 rounded-2xl bg-neutral-800/60 border border-neutral-700/40 flex items-center justify-center text-neutral-400 mb-4 shadow-inner">
          <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
        </div>
        <h3 class="text-sm font-semibold text-neutral-200 mb-1">开始探索你的照片</h3>
        <p class="text-xs text-neutral-400 mb-6">可直接拖拽单张图片或相册文件夹到窗口，或点击下方按钮打开：</p>
        <div class="flex items-center gap-3">
          <button
            onclick={handleOpenSingleImage}
            class="px-4 py-2 bg-neutral-800 hover:bg-neutral-700 border border-neutral-700 text-white rounded-lg text-xs font-medium transition flex items-center gap-2"
          >
            <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
            打开单张图片
          </button>
          <button
            onclick={handleOpenFolder}
            class="px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded-lg text-xs font-medium transition flex items-center gap-2"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
            选择图片目录
          </button>
        </div>
      </div>
    {:else}
      <!-- Photo Grid -->
      <div
        class="grid gap-3"
        style="grid-template-columns: repeat(auto-fill, minmax({thumbSize}px, 1fr));"
      >
        {#each filteredItems as item, idx (item.path)}
          <div
            class="group relative bg-neutral-900 rounded-lg overflow-hidden border border-neutral-800/80 hover:border-neutral-600 transition shadow-sm hover:shadow-md cursor-pointer flex flex-col"
            onclick={() => onSelectPhoto(filteredItems, idx)}
            role="button"
            tabindex="0"
            onkeydown={(e) => e.key === 'Enter' && onSelectPhoto(filteredItems, idx)}
          >
            <!-- Image Aspect Box -->
            <div class="w-full aspect-square bg-neutral-950/60 overflow-hidden relative flex items-center justify-center">
              <img
                src={toAssetUrl(item.path)}
                alt={item.filename}
                loading="lazy"
                class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-200 pointer-events-none"
              />

              <!-- Star Badge -->
              {#if item.is_starred}
                <div class="absolute top-2 right-2 text-amber-400 drop-shadow">
                  ★
                </div>
              {/if}

              <!-- Hover Overlay with Resolution -->
              <div class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-end p-2">
                <span class="text-[10px] text-neutral-300 font-mono drop-shadow">
                  {item.width} × {item.height}
                </span>
              </div>
            </div>

            <!-- Filename Footer -->
            <div class="px-2.5 py-2 text-[11px] text-neutral-300 truncate bg-neutral-900 border-t border-neutral-800/50">
              {item.filename}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </main>
</div>
