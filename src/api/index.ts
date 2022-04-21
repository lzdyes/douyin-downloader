import { fetch } from '@tauri-apps/api/http'

interface UserResponse {
  status_code: number
  user_info: {
    uid: string
    sec_uid: string
    short_id: string
    unique_id: string
    nickname: string
    signature: string
  }
}

interface VideoResonse {
  status_code: number
  item_list: Array<{
    desc: string
    video: {
      vid: string
      play_addr: {
        url_list: Array<string>
      }
    }
  }>
}

export const getUser = async (sec_uid: string) => {
  const url = 'https://www.iesdouyin.com/web/api/v2/user/info'
  const response = await fetch<UserResponse>(url, { method: 'GET', query: { sec_uid } }).catch((error) => console.error(error))
  if (response && response.data && response.data.status_code === 0) return response.data.user_info
}

export const getVideo = async (item_ids: string) => {
  const url = 'https://www.douyin.com/web/api/v2/aweme/iteminfo'
  const response = await fetch<VideoResonse>(url, { method: 'GET', query: { item_ids } }).catch((error) => console.error(error))
  if (response && response.data && response.data.status_code === 0) return response.data.item_list[0]
}
