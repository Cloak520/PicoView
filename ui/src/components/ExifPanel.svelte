<script lang="ts">
  import type { ExifData, MediaItem } from '../lib/types'

  let {
    item,
    exif,
    naturalWidth = 0,
    naturalHeight = 0,
    onClose,
  }: {
    item: MediaItem
    exif: ExifData | null
    naturalWidth?: number
    naturalHeight?: number
    onClose: () => void
  } = $props()

  const displayWidth = $derived(naturalWidth || item.width || 0)
  const displayHeight = $derived(naturalHeight || item.height || 0)

  function formatBytes(bytes: number) {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
  }
</script>

<div class="absolute right-4 top-16 w-80 bg-neutral-900/90 backdrop-blur-md border border-neutral-700/60 rounded-xl p-5 shadow-2xl text-xs z-50 transition-all duration-200">
  <div class="flex items-center justify-between pb-3 border-b border-neutral-800">
    <h3 class="font-semibold text-sm text-neutral-200">属性与 EXIF 信息</h3>
    <button onclick={onClose} class="text-neutral-400 hover:text-white p-1 rounded-md hover:bg-neutral-800 transition">
      ✕
    </button>
  </div>

  <div class="mt-4 space-y-3">
    <!-- Basic File Info -->
    <div>
      <span class="text-neutral-500 block mb-0.5">文件名称</span>
      <span class="text-neutral-200 font-medium break-all">{item.filename}</span>
    </div>

    <div class="grid grid-cols-2 gap-2">
      <div>
        <span class="text-neutral-500 block mb-0.5">尺寸分辨率</span>
        <span class="text-neutral-200">
          {displayWidth && displayHeight ? `${displayWidth} × ${displayHeight}` : '未知'}
        </span>
      </div>
      <div>
        <span class="text-neutral-500 block mb-0.5">文件大小</span>
        <span class="text-neutral-200">{formatBytes(item.size_bytes)}</span>
      </div>
    </div>

    <div>
      <span class="text-neutral-500 block mb-0.5">文件路径</span>
      <span class="text-neutral-400 text-[11px] break-all select-text">{item.path}</span>
    </div>

    {#if exif}
      <div class="pt-3 border-t border-neutral-800/80 space-y-2.5">
        {#if exif.camera_make || exif.camera_model}
          <div>
            <span class="text-neutral-500 block mb-0.5">拍摄设备</span>
            <span class="text-blue-400 font-medium">
              {[exif.camera_make, exif.camera_model].filter(Boolean).join(' ')}
            </span>
          </div>
        {/if}

        {#if exif.lens_model}
          <div>
            <span class="text-neutral-500 block mb-0.5">镜头型号</span>
            <span class="text-neutral-300">{exif.lens_model}</span>
          </div>
        {/if}

        <div class="grid grid-cols-3 gap-2 py-1 px-2 bg-neutral-800/50 rounded-lg text-center">
          <div>
            <span class="text-neutral-500 block text-[10px]">光圈</span>
            <span class="text-neutral-200 font-mono font-medium">{exif.f_number || '--'}</span>
          </div>
          <div>
            <span class="text-neutral-500 block text-[10px]">快门</span>
            <span class="text-neutral-200 font-mono font-medium">{exif.exposure_time || '--'}</span>
          </div>
          <div>
            <span class="text-neutral-500 block text-[10px]">ISO</span>
            <span class="text-neutral-200 font-mono font-medium">{exif.iso || '--'}</span>
          </div>
        </div>

        {#if exif.focal_length}
          <div class="flex justify-between">
            <span class="text-neutral-500">焦距</span>
            <span class="text-neutral-300">{exif.focal_length}</span>
          </div>
        {/if}

        {#if exif.date_time_original}
          <div class="flex justify-between">
            <span class="text-neutral-500">拍摄时间</span>
            <span class="text-neutral-300">{exif.date_time_original}</span>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>
