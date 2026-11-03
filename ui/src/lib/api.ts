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
