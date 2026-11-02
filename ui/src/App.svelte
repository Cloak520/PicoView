<script lang="ts">
  import type { MediaItem } from './lib/types'
  import Library from './components/Library.svelte'
  import Viewer from './components/Viewer.svelte'

  let mode = $state<'library' | 'viewer'>('library')
  let photoList = $state<MediaItem[]>([])
  let activeIndex = $state(0)

  function handleOpenViewer(items: MediaItem[], index: number) {
    photoList = items
    activeIndex = index
    mode = 'viewer'
  }

  function handleCloseViewer() {
    mode = 'library'
  }
</script>

<div class="w-full h-full overflow-hidden bg-[#121214]">
  {#if mode === 'library'}
    <Library onSelectPhoto={handleOpenViewer} />
  {:else}
    <Viewer
      items={photoList}
      currentIndex={activeIndex}
      onClose={handleCloseViewer}
    />
  {/if}
</div>
