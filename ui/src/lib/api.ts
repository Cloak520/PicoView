import type { ExifData, MediaItem } from './types'

// Check if running inside Tauri
export const isTauri = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window

export async function invokeCommand<T>(cmd: string, args: Record<string, any> = {}): Promise<T> {
  if (isTauri()) {
    const { invoke } = await import('@tauri-apps/api/core')
    return invoke<T>(cmd, args)
  }
  console.warn(`[Mock] invokeCommand called: ${cmd}`, args)
  throw new Error(`Running outside Tauri runtime`)
}

export function toAssetUrl(filePath: string): string {
  if (isTauri()) {
    // In Tauri v2, convertFileSrc converts absolute paths to asset:// protocol
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const { convertFileSrc } = (window as any).__TAURI_INTERNALS__ || {}
    if (convertFileSrc) {
      return convertFileSrc(filePath)
    }
    // Fallback standard Tauri v2 asset URL
    return `asset://localhost/${encodeURIComponent(filePath)}`
  }
  return filePath
}

export async function scanFolder(folderPath: string): Promise<MediaItem[]> {
  return invokeCommand<MediaItem[]>('scan_folder', { folderPath })
}

export async function getSiblingPhotos(photoPath: string): Promise<MediaItem[]> {
  return invokeCommand<MediaItem[]>('get_sibling_photos', { photoPath })
}

export async function getExifData(photoPath: string): Promise<ExifData> {
  return invokeCommand<ExifData>('get_exif_data', { photoPath })
}

export async function generateThumbnail(photoPath: string, maxSize = 320): Promise<string> {
  return invokeCommand<string>('generate_thumbnail', { photoPath, maxSize })
}

export async function deleteToTrash(photoPath: string): Promise<void> {
  return invokeCommand<void>('delete_to_trash', { photoPath })
}
