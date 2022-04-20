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

export const getUser = async (sec_uid: string) => {
  const url = 'https://www.iesdouyin.com/web/api/v2/user/info'
  const response = await fetch<UserResponse>(url, { method: 'GET', query: { sec_uid } }).catch((error) => console.error(error))
  if (response && response.data && response.data.status_code === 0) return response.data.user_info
}
