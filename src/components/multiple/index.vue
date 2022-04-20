<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { getUser } from '@/api'
import { FileNameType, FolderNameType } from '@/enums'
import { exists, generateVideoURL } from '@/utils'
import { invoke, http, dialog, fs } from '@tauri-apps/api'
import { basename, resolve } from '@tauri-apps/api/path'
import { ElButton, ElForm, ElFormItem, ElInput, ElProgress, ElDivider, ElMessage, ElRadioGroup, ElRadio, ElRadioButton } from 'element-plus'
import { computed, ref } from 'vue'

const form = ref({
  homeURL: '',
  savePath: '',
  ratio: '1080p',
  watermark: 0,
  folderNameType: FolderNameType.Nickname,
  fileNameType: FileNameType.TitleTag,
  audio: false,
})

const isDownloading = ref(false)

const total = ref(0)
const successCount = ref(0)
const failureCount = ref(0)
const percent = computed(() => Math.floor(((successCount.value + failureCount.value) / total.value) * 100) || 0)

let isListCompleted = false

const onSaveClick = async () => {
  const path = (await dialog.open({ directory: true })) as string
  path && (form.value.savePath = path)
}

const onSubmit = async () => {
  total.value = 0
  successCount.value = 0
  failureCount.value = 0
  isListCompleted = false

  if (!form.value.homeURL) return ElMessage.error('请输入用户主页地址')
  if (!form.value.savePath) return ElMessage.error('请选择保存位置')

  const sec_uid = form.value.homeURL.split('/').pop()
  if (!sec_uid) return ElMessage.error('请输入正确的用户主页地址')

  isDownloading.value = true

  const folderPath = await createFolder(sec_uid)
  if (!folderPath) {
    isDownloading.value = false
    ElMessage.error('文件夹创建失败，请重试')
    return
  }

  requsetList(sec_uid, '0', folderPath)
}

const onCancelClick = () => {
  isDownloading.value = false
}

const createFolder = async (sec_uid: string) => {
  const user = await getUser(sec_uid)
  if (!user) return

  const { uid, short_id, unique_id, nickname } = user

  let folderName = ''
  switch (form.value.folderNameType) {
    case FolderNameType.Nickname:
      folderName = nickname
      break
    case FolderNameType.ID:
      folderName = uid
      break
    case FolderNameType.DouyinID:
      folderName = unique_id || short_id
      break
  }

  const folderPath = await resolve(form.value.savePath, folderName)
  const isExist = await exists(folderPath)
  !isExist && (await fs.createDir(folderPath))

  return folderPath
}

const requsetList = (sec_uid: string, max_cursor: string, folderPath: string) => {
  const listURL = 'https://www.iesdouyin.com/web/api/v2/aweme/post'

  http
    .fetch(listURL, {
      method: 'GET',
      query: { sec_uid, max_cursor, count: '100' },
    })
    .then((response) => {
      console.log(response)

      if (!isDownloading.value) return

      const data = response.data as any

      if (data.has_more) {
        total.value += data.aweme_list.length

        requsetList(sec_uid, data.max_cursor.toString(), folderPath)

        data.aweme_list.forEach(async (v: any, i: number) => {
          const { desc, video } = v
          const { vid } = video
          const isAudio = !vid

          /** skip audio */
          if (isAudio && !form.value.audio) return

          const url = isAudio ? video.play_addr.url_list[0] : generateVideoURL(vid, form.value.ratio, form.value.watermark)

          let name = ''
          switch (form.value.fileNameType) {
            case FileNameType.TitleTag:
              name = desc
              break
            case FileNameType.Title:
              const index = desc.indexOf('#')
              if (index === -1) name = desc.trim()
              else name = desc.substring(0, index).trim()
              break
            case FileNameType.ID:
              name = isAudio ? await basename(url, '.mp3') : vid
              break
            case FileNameType.Index:
              name = (total.value - data.aweme_list.length + i + 1).toString()
              break
          }

          const ext = isAudio ? '.mp3' : '.mp4'
          const path = await resolve(folderPath, name + ext)
          invoke('download', { url, path })
            .then(() => successCount.value++)
            .catch(() => failureCount.value++)
            .finally(() => isListCompleted && percent.value === 100 && (isDownloading.value = false))
        })
      } else {
        console.log('over')
        isListCompleted = true
        percent.value === 100 && (isDownloading.value = false)
      }
    })
    .catch((error) => console.error(error))
}
</script>
