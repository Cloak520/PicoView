<script lang="ts">
  import { onMount } from 'svelte'
  import type { MediaItem } from './lib/types'
  import { exitApp, invokeCommand, isTauri, openTargetPath, setFullscreen } from './lib/api'
  import Welcome, { type RecentItem } from './components/Welcome.svelte'
  import Viewer from './components/Viewer.svelte'

  const RECENTS_KEY = 'picoview_recent_history'

  let photoList = $state<MediaItem[]>([])
  let activeIndex = $state(0)
  let recents = $state<RecentItem[]>([])
  let isDragOver = $state(false)
  let isExternalOpen = $state(false)

  function loadRecents() {
    try {
      const raw = localStorage.getItem(RECENTS_KEY)
      if (raw) {
        recents = JSON.parse(raw)
      }
    } catch {
      recents = []
    }
  }

  function addRecent(path: string, filename: string) {
    const next = [
      { path, filename, timestamp: Date.now() },
      ...recents.filter((r) => r.path !== path),
    ].slice(0, 12)
    recents = next
    try {
      localStorage.setItem(RECENTS_KEY, JSON.stringify(next))
    } catch {}
  }

  function clearRecents() {
    recents = []
    try {
      localStorage.removeItem(RECENTS_KEY)
    } catch {}
  }

  async function handleOpenPath(path: string) {
    try {
      const res = await openTargetPath(path)
      if (res.type === 'SinglePhoto') {
        photoList = res.data.siblings
        activeIndex = res.data.current_index
        const current = res.data.siblings[res.data.current_index]
        if (current) {
          addRecent(current.path, current.filename)
        }
        if (isExternalOpen) {
          setFullscreen(true).catch(() => {})
        }
      } else if (res.type === 'Folder') {
        if (res.data.items.length > 0) {
          photoList = res.data.items
          activeIndex = 0
          addRecent(res.data.items[0].path, res.data.items[0].filename)
        }
      }
    } catch (err) {
      alert(`无法打开图片: ${err}`)
    }
  }

  async function handlePickFile() {
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
        title: '选择图片',
      })
      if (selected && typeof selected === 'string') {
        handleOpenPath(selected)
      }
    } catch {
      const path = prompt('请输入本地图片绝对路径:')
      if (path) handleOpenPath(path)
    }
  }

  function handleCloseViewer() {
    if (isExternalOpen) {
      exitApp()
    } else {
      photoList = []
      activeIndex = 0
    }
  }

  function handleGlobalKeyDown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && (e.key === 'o' || e.key === 'O')) {
      e.preventDefault()
      handlePickFile()
    }
  }

  onMount(() => {
    loadRecents()
    window.addEventListener('keydown', handleGlobalKeyDown)

    // Check CLI argument on launch (e.g. double clicked photo or picoview <file>)
    if (isTauri()) {
      invokeCommand<string | null>('get_cli_arg')
        .then((initialArg) => {
          if (initialArg) {
            isExternalOpen = true
            handleOpenPath(initialArg)
          }
        })
        .catch(() => {})

      // Listen for system "Open With" file events (macOS Finder / OS right click)
      import('@tauri-apps/api/event').then(({ listen }) => {
        listen<string>('open-file', (event) => {
          if (event.payload) {
            isExternalOpen = true
            handleOpenPath(event.payload)
          }
        })
      })

      // Listen for window file drop
      import('@tauri-apps/api/window').then(({ getCurrentWindow }) => {
        getCurrentWindow().onDragDropEvent((event) => {
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
      })
    }

    return () => {
      window.removeEventListener('keydown', handleGlobalKeyDown)
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
  aria-label="PicoView"
>
  {#if photoList.length > 0}
    <!-- Fullscreen Ultra-Fast Photo Viewer -->
    <div class="w-full h-full">
      <Viewer
        items={photoList}
        currentIndex={activeIndex}
        {isExternalOpen}
        onClose={handleCloseViewer}
        onPickNewPhoto={handlePickFile}
      />
    </div>
  {:else}
    <!-- Minimalist Welcome Canvas -->
    <Welcome
      {recents}
      onOpenPhoto={handleOpenPath}
      onPickFile={handlePickFile}
      onClearRecents={clearRecents}
    />
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
          <p class="text-xs text-neutral-400">直接进入极速看图与胶片导轨模式</p>
        </div>
      </div>
    </div>
  {/if}
</div>
