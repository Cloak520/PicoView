export interface MediaItem {
  id?: number
  path: string
  filename: string
  parent_dir: string
  size_bytes: number
  modified_ms: number
  width: number
  height: number
  mime_type: string
  is_starred: boolean
  rating: number
  date_taken?: string
}

export interface ExifData {
  camera_make?: string
  camera_model?: string
  lens_model?: string
  focal_length?: string
  f_number?: string
  exposure_time?: string
  iso?: number
  date_time_original?: string
  latitude?: number
  longitude?: number
}

export interface ImageRecipe {
  photo_path: string
  crop?: [number, number, number, number]
  rotation: number
  brightness: number
  contrast: number
  saturation: number
  is_starred: boolean
}

export type OpenTargetResult =
  | {
      type: 'Folder'
      data: {
        folder_path: string
        items: MediaItem[]
      }
    }
  | {
      type: 'SinglePhoto'
      data: {
        siblings: MediaItem[]
        current_index: number
      }
    }
