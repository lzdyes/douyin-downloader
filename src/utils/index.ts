import { updater } from '@tauri-apps/api'
import store from '@/store'

export const sleep = (delay?: number) => new Promise<void>((resole) => setTimeout(resole, delay))

export const generateVideoURL = (vid: string, ratio: string, watermark: number) => {
  return `https://aweme.snssdk.com/aweme/v1/play/?video_id=${vid}&ratio=${ratio}&watermark=${watermark}`
}

export const checkUpdate = async () => {
  try {
    const { shouldUpdate, manifest } = await updater.checkUpdate()
    shouldUpdate && store.setUpdater({ active: true, version: manifest?.version })
  } catch (error) {
    console.error(error)
  }
}
