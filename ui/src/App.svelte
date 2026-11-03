<script lang="ts">
  import { onMount } from 'svelte'
  import type { MediaItem } from './lib/types'
  import { isTauri, openTargetPath } from './lib/api'
  import Library from './components/Library.svelte'
  import Viewer from './components/Viewer.svelte'

  let mode = $state<'library' | 'viewer'>('library')
  let photoList = $state<MediaItem[]>([])
  let activeIndex = $state(0)

  // Preserved Library state
  let libraryItems = $state<MediaItem[]>([])
  let currentFolderPath = $state<string>('')
  let isScanning = $state(false)

  // Drag and drop overlay state
  let isDragOver = $state(false)

  function handleOpenViewer(items: MediaItem[], index: number) {
    photoList = items
    activeIndex = index
    mode = 'viewer'
  }

  function handleCloseViewer() {
    mode = 'library'
  }

  async function handleOpenPath(path: string) {
    isScanning = true
    try {
      const res = await openTargetPath(path)
      if (res.type === 'SinglePhoto') {
        photoList = res.data.siblings
        activeIndex = res.data.current_index
        mode = 'viewer'
        // If library has no folder open yet, populate it with this photo's folder
        if (libraryItems.length === 0 && res.data.siblings.length > 0) {
          libraryItems = res.data.siblings
          currentFolderPath = res.data.siblings[0].parent_dir
        }
      } else if (res.type === 'Folder') {
        libraryItems = res.data.items
        currentFolderPath = res.data.folder_path
        mode = 'library'
      }
    } catch (err) {
      alert(`无法打开: ${err}`)
    } finally {
      isScanning = false
    }
  }

  onMount(() => {
    let unlistenDragDrop: (() => void) | undefined

    if (isTauri()) {
      import('@tauri-apps/api/window').then(({ getCurrentWindow }) => {
        getCurrentWindow()
          .onDragDropEvent((event) => {
            const payload = event.payload
            if (payload.type === 'over') {
              isDragOver = true
            } else if (payload.type === 'leave') {
              isDragOver = false
            } else if (payload.type === 'drop') {
              isDragOver = false
              const paths = (payload as any).paths
              if (paths && paths.length > 0) {
                handleOpenPath(paths[0])
              }
            }
          })
          .then((unlisten) => {
            unlistenDragDrop = unlisten
          })
      })
    }

    return () => {
      unlistenDragDrop?.()
    }
  })
</script>

<div
  class="relative w-full h-full overflow-hidden bg-[#121214]"
  ondragover={(e) => {
    e.preventDefault()
    isDragOver = true
  }}
  ondragleave={(e) => {
    e.preventDefault()
    isDragOver = false
  }}
  ondrop={(e) => {
    e.preventDefault()
    isDragOver = false
  }}
  role="region"
  aria-label="PicoView Main"
>
  <!-- Library View: permanently kept in layout tree so scroll and thumbnails are warm with ZERO ESC lag -->
  <div
    class="w-full h-full"
    class:pointer-events-none={mode === 'viewer'}
    aria-hidden={mode === 'viewer'}
  >
    <Library
      bind:items={libraryItems}
      bind:currentFolderPath
      isLoading={isScanning}
      onSelectPhoto={handleOpenViewer}
      onOpenPath={handleOpenPath}
    />
  </div>

  <!-- Viewer View: rendered when in viewer mode -->
  {#if mode === 'viewer'}
    <div class="fixed inset-0 z-40 bg-[#121214]">
      <Viewer
        items={photoList}
        currentIndex={activeIndex}
        onClose={handleCloseViewer}
      />
    </div>
  {/if}

  <!-- Drag & Drop Hover Overlay -->
  {#if isDragOver}
    <div class="fixed inset-0 z-50 bg-blue-950/40 backdrop-blur-md border-4 border-dashed border-blue-500/80 flex flex-col items-center justify-center pointer-events-none transition-all duration-150">
      <div class="p-8 bg-neutral-900/90 rounded-2xl border border-neutral-700/80 shadow-2xl flex flex-col items-center gap-4">
        <div class="w-16 h-16 rounded-full bg-blue-500/20 flex items-center justify-center text-blue-400">
          <svg class="w-8 h-8 animate-bounce" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
          </svg>
        </div>
        <div class="text-center">
          <h2 class="text-base font-semibold text-white mb-1">松开鼠标即可打开</h2>
          <p class="text-xs text-neutral-400">支持拖入单张图片或相册文件夹</p>
        </div>
      </div>
    </div>
  {/if}
</div>
