<script lang="ts">
  import { onMount } from 'svelte'
  import type { ExifData, MediaItem } from '../lib/types'
  import { deleteToTrash, getExifData, toAssetUrl } from '../lib/api'
  import Toolbar from './Toolbar.svelte'
  import ExifPanel from './ExifPanel.svelte'

  let {
    items = [],
    currentIndex = 0,
    onClose,
  }: {
    items: MediaItem[]
    currentIndex: number
    onClose: () => void
  } = $props()

  let activeIndex = $state(0)

  $effect(() => {
    activeIndex = currentIndex
  })
  let zoom = $state(1)
  let rotation = $state(0)
  let panX = $state(0)
  let panY = $state(0)
  let isDragging = $state(false)
  let startX = $state(0)
  let startY = $state(0)
  let showExif = $state(false)
  let exifData = $state<ExifData | null>(null)

  const currentItem = $derived(items[activeIndex])
  const prevItem = $derived(activeIndex > 0 ? items[activeIndex - 1] : null)
  const nextItem = $derived(activeIndex < items.length - 1 ? items[activeIndex + 1] : null)

  function resetViewport() {
    zoom = 1
    rotation = 0
    panX = 0
    panY = 0
  }

  function nextPhoto() {
    if (activeIndex < items.length - 1) {
      activeIndex++
      resetViewport()
      loadExif()
    }
  }

  function prevPhoto() {
    if (activeIndex > 0) {
      activeIndex--
      resetViewport()
      loadExif()
    }
  }

  function handleWheel(e: WheelEvent) {
    e.preventDefault()
    const zoomFactor = 1.15
    if (e.deltaY < 0) {
      zoom = Math.min(zoom * zoomFactor, 15)
    } else {
      zoom = Math.max(zoom / zoomFactor, 0.2)
      if (zoom <= 1) {
        panX = 0
        panY = 0
      }
    }
  }

  function handleMouseDown(e: MouseEvent) {
    if (e.button === 0 && zoom > 1) {
      isDragging = true
      startX = e.clientX - panX
      startY = e.clientY - panY
    }
  }

  function handleMouseMove(e: MouseEvent) {
    if (isDragging) {
      panX = e.clientX - startX
      panY = e.clientY - startY
    }
  }

  function handleMouseUp() {
    isDragging = false
  }

  function handleDoubleClick() {
    if (zoom === 1) {
      zoom = 2.5
    } else {
      resetViewport()
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    switch (e.key) {
      case 'ArrowRight':
      case 'PageDown':
      case ' ':
        nextPhoto()
        break
      case 'ArrowLeft':
      case 'PageUp':
        prevPhoto()
        break
      case 'ArrowUp':
      case '+':
      case '=':
        zoom = Math.min(zoom * 1.2, 15)
        break
      case 'ArrowDown':
      case '-':
        zoom = Math.max(zoom / 1.2, 0.2)
        break
      case '0':
        resetViewport()
        break
      case 'r':
      case 'R':
        if (e.shiftKey) {
          rotation = (rotation - 90) % 360
        } else {
          rotation = (rotation + 90) % 360
        }
        break
      case 'i':
      case 'I':
        showExif = !showExif
        if (showExif) loadExif()
        break
      case 's':
      case 'S':
        toggleStar()
        break
      case 'Delete':
      case 'Backspace':
        deleteCurrent()
        break
      case 'Escape':
      case 'l':
      case 'L':
        onClose()
        break
    }
  }

  async function loadExif() {
    if (!currentItem) return
    try {
      exifData = await getExifData(currentItem.path)
    } catch {
      exifData = null
    }
  }

  function toggleStar() {
    if (!currentItem) return
    currentItem.is_starred = !currentItem.is_starred
  }

  async function deleteCurrent() {
    if (!currentItem) return
    const confirmed = confirm(`确定将照片 "${currentItem.filename}" 移入系统回收站吗？`)
    if (confirmed) {
      try {
        await deleteToTrash(currentItem.path)
        items.splice(activeIndex, 1)
        if (activeIndex >= items.length) {
          activeIndex = Math.max(0, items.length - 1)
        }
        resetViewport()
      } catch (err) {
        alert(`删除失败: ${err}`)
      }
    }
  }

  $effect(() => {
    if (showExif) {
      loadExif()
    }
  })

  onMount(() => {
    window.addEventListener('keydown', handleKeyDown)
    return () => {
      window.removeEventListener('keydown', handleKeyDown)
    }
  })
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  class="relative w-full h-full bg-[#121214] flex flex-col justify-between overflow-hidden select-none"
  onwheel={handleWheel}
  onmousedown={handleMouseDown}
  onmousemove={handleMouseMove}
  onmouseup={handleMouseUp}
  role="region"
  aria-label="Photo Viewer"
>
  <!-- Top Bar (Picasa style minimalist breadcrumb & info) -->
  <div class="absolute top-0 left-0 right-0 h-12 flex items-center justify-between px-5 bg-gradient-to-b from-black/70 to-transparent z-30 pointer-events-none">
    <div class="flex items-center gap-3 pointer-events-auto">
      <button
        onclick={onClose}
        class="text-neutral-400 hover:text-white p-1 rounded-md hover:bg-white/10 transition"
        title="返回相册 (Esc)"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
        </svg>
      </button>
      <span class="text-sm font-medium text-neutral-300 drop-shadow truncate max-w-md">
        {currentItem?.filename ?? ''}
      </span>
    </div>

    <div class="flex items-center gap-4 text-xs text-neutral-400 font-mono pointer-events-auto">
      {#if currentItem}
        <span>{activeIndex + 1} / {items.length}</span>
        <span>{currentItem.width} × {currentItem.height}</span>
      {/if}
    </div>
  </div>

  <!-- Main Viewport Canvas -->
  <div class="relative w-full h-full flex items-center justify-center overflow-hidden">
    {#if currentItem}
      <img
        src={toAssetUrl(currentItem.path)}
        alt={currentItem.filename}
        ondblclick={handleDoubleClick}
        draggable="false"
        class="max-w-full max-h-full object-contain pointer-events-auto transition-transform duration-75 select-none"
        style="
          transform: translate({panX}px, {panY}px) scale({zoom}) rotate({rotation}deg);
          cursor: {zoom > 1 ? (isDragging ? 'grabbing' : 'grab') : 'default'};
        "
      />
    {:else}
      <div class="text-neutral-500 text-sm">没有选中的图片</div>
    {/if}
  </div>

  <!-- Adjacent Photos Silent Preloader -->
  <div class="hidden">
    {#if prevItem}
      <img src={toAssetUrl(prevItem.path)} alt="preload-prev" />
    {/if}
    {#if nextItem}
      <img src={toAssetUrl(nextItem.path)} alt="preload-next" />
    {/if}
  </div>

  <!-- Edge Navigation Buttons (Picasa style large hover targets) -->
  {#if activeIndex > 0}
    <button
      onclick={prevPhoto}
      class="absolute left-2 top-1/2 -translate-y-1/2 p-3 rounded-full bg-black/40 hover:bg-black/70 text-white/60 hover:text-white transition backdrop-blur-sm z-30"
      title="上一张 (Left Arrow)"
    >
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
    </button>
  {/if}

  {#if activeIndex < items.length - 1}
    <button
      onclick={nextPhoto}
      class="absolute right-2 top-1/2 -translate-y-1/2 p-3 rounded-full bg-black/40 hover:bg-black/70 text-white/60 hover:text-white transition backdrop-blur-sm z-30"
      title="下一张 (Right Arrow / Space)"
    >
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </button>
  {/if}

  <!-- Floating EXIF Inspector -->
  {#if showExif && currentItem}
    <ExifPanel item={currentItem} exif={exifData} onClose={() => (showExif = false)} />
  {/if}

  <!-- Bottom Floating Toolbar -->
  {#if currentItem}
    <Toolbar
      {zoom}
      {rotation}
      isStarred={currentItem.is_starred}
      {showExif}
      onZoomIn={() => (zoom = Math.min(zoom * 1.25, 15))}
      onZoomOut={() => (zoom = Math.max(zoom / 1.25, 0.2))}
      onZoomReset={resetViewport}
      onRotateLeft={() => (rotation = (rotation - 90) % 360)}
      onRotateRight={() => (rotation = (rotation + 90) % 360)}
      onToggleStar={toggleStar}
      onToggleExif={() => {
        showExif = !showExif
        if (showExif) loadExif()
      }}
      onDelete={deleteCurrent}
      onBackToLibrary={onClose}
    />
  {/if}
</div>
