import { convertFileSrc, invoke, isTauri } from '@tauri-apps/api/core'
import type { ExifData, OpenTargetResult } from './types'

export { isTauri }

export async function invokeCommand<T>(cmd: string, args: Record<string, any> = {}): Promise<T> {
  if (isTauri()) {
    return invoke<T>(cmd, args)
  }
  console.warn(`[Mock] invokeCommand called: ${cmd}`, args)
  throw new Error(`Running outside Tauri runtime`)
}

export function toAssetUrl(filePath: string): string {
  if (isTauri()) {
    return convertFileSrc(filePath)
  }
  return filePath
}

export async function openTargetPath(targetPath: string): Promise<OpenTargetResult> {
  return invokeCommand<OpenTargetResult>('open_target_path', { targetPath })
}

export async function getInitialTarget(): Promise<OpenTargetResult | null> {
  return invokeCommand<OpenTargetResult | null>('get_initial_target')
}

export async function getExifData(photoPath: string): Promise<ExifData> {
  return invokeCommand<ExifData>('get_exif_data', { photoPath })
}

export async function deleteToTrash(photoPath: string): Promise<void> {
  return invokeCommand<void>('delete_to_trash', { photoPath })
}

export async function exitApp(): Promise<void> {
  return invokeCommand<void>('exit_app')
}

export async function fitToScreen(): Promise<void> {
  return invokeCommand<void>('fit_to_screen')
}

export async function copyImageToClipboard(filePath: string): Promise<boolean> {
  // 1. Invoke native backend command (registers file object on OS pasteboard for Finder/Explorer)
  if (isTauri()) {
    try {
      await invokeCommand<void>('copy_file_to_clipboard', { filePath })
    } catch (err) {
      console.warn('Native copy_file_to_clipboard warning:', err)
    }
  }

  // 2. Also write decoded PNG image blob to Web Clipboard (so pasting in chat apps, Word, etc. pastes image directly)
  try {
    const assetUrl = toAssetUrl(filePath)
    const img = new Image()
    img.crossOrigin = 'anonymous'
    await new Promise<void>((resolve, reject) => {
      img.onload = () => resolve()
      img.onerror = () => reject(new Error('Failed to load image for clipboard'))
      img.src = assetUrl
    })

    const canvas = document.createElement('canvas')
    canvas.width = img.naturalWidth
    canvas.height = img.naturalHeight
    const ctx = canvas.getContext('2d')
    if (ctx) {
      ctx.drawImage(img, 0, 0)
      const blob = await new Promise<Blob | null>((resolve) => canvas.toBlob(resolve, 'image/png'))
      if (blob && navigator.clipboard && window.ClipboardItem) {
        await navigator.clipboard.write([
          new ClipboardItem({
            'image/png': blob,
          }),
        ])
        return true
      }
    }
  } catch (err) {
    console.warn('Web clipboard write image warning:', err)
    // Fallback: write file path text to clipboard
    try {
      if (navigator.clipboard) {
        await navigator.clipboard.writeText(filePath)
        return true
      }
    } catch {}
  }

  return true
}
