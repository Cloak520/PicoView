<script lang="ts">
  import { onMount } from 'svelte'
  import type { MediaItem } from '../lib/types'
  import { toAssetUrl } from '../lib/api'

  let {
    items = [],
    activeIndex = 0,
    onSelect,
  }: {
    items: MediaItem[]
    activeIndex: number
    onSelect: (index: number) => void
  } = $props()

  let containerRef = $state<HTMLDivElement | null>(null)

  function scrollToActive() {
    if (!containerRef) return
    const activeEl = containerRef.children[activeIndex] as HTMLElement | undefined
    if (activeEl) {
      activeEl.scrollIntoView({ behavior: 'smooth', inline: 'center', block: 'nearest' })
    }
  }

  $effect(() => {
    if (activeIndex >= 0) {
      scrollToActive()
    }
  })

  function handleWheel(e: WheelEvent) {
    if (!containerRef) return
    if (Math.abs(e.deltaY) > Math.abs(e.deltaX)) {
      e.preventDefault()
      containerRef.scrollLeft += e.deltaY * 1.5
    }
  }
</script>

<div
  class="w-full flex justify-center pointer-events-auto px-4 select-none"
  onwheel={handleWheel}
  role="region"
  aria-label="Thumbnail Filmstrip"
>
  <div
    bind:this={containerRef}
    class="max-w-4xl flex items-center gap-2 p-2 bg-neutral-900/80 backdrop-blur-2xl border border-neutral-700/50 rounded-2xl shadow-2xl overflow-x-auto scrollbar-none"
    style="scrollbar-width: none;"
  >
    {#each items as item, idx (item.path)}
      <button
        onclick={() => onSelect(idx)}
        class="relative flex-shrink-0 w-14 h-14 rounded-xl overflow-hidden border-2 transition-all duration-150 group {idx === activeIndex ? 'border-blue-500 scale-105 shadow-lg shadow-blue-500/30' : 'border-transparent opacity-60 hover:opacity-100 hover:border-neutral-500'}"
        title={item.filename}
      >
        <img
          src={toAssetUrl(item.path)}
          alt={item.filename}
          loading="lazy"
          class="w-full h-full object-cover pointer-events-none"
        />
      </button>
    {/each}
  </div>
</div>
