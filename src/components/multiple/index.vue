<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { getUser, getVideos } from '@/api'
import type { VideoItem } from '@/api'
import { FileNameType, FolderNameType } from '@/enums'
import { exists, formatSize, generateVideoURL } from '@/utils'
import { dialog, fs, invoke } from '@tauri-apps/api'
import { basename, resolve } from '@tauri-apps/api/path'
import { ElButton, ElForm, ElFormItem, ElInput, ElProgress, ElDivider, ElMessage, ElRadioGroup, ElRadio, ElRadioButton } from 'element-plus'
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const form = ref({
  homeURL: '',
  ratio: '1080p',
  watermark: 0,
  audio: false,
  folderNameType: FolderNameType.Nickname,
  fileNameType: FileNameType.TitleTag,
  savePath: '',
})

const isDownloading = ref(false)
const diskFreeSize = ref('')
const total = ref(0)
const successCount = ref(0)
const failureCount = ref(0)
const percent = computed(() => Math.floor(((successCount.value + failureCount.value) / total.value) * 100) || 0)

const onSaveClick = async () => {
  const path = (await dialog.open({ directory: true })) as string
  path && (form.value.savePath = path)

  const freeSize = (await invoke('disk_free_size', { path })) as number
  diskFreeSize.value = formatSize(freeSize)
}

const onSubmit = async () => {
  total.value = 0
  successCount.value = 0
  failureCount.value = 0

  const { homeURL, ratio, watermark, audio, folderNameType, fileNameType, savePath } = form.value

  if (!homeURL) return ElMessage.error(t('message.empty_home_url'))
  if (!savePath) return ElMessage.error(t('message.empty_sava_path'))

  const sec_uid = homeURL.split('/').pop()
  if (!sec_uid) return ElMessage.error(t('message.error_home_url'))

  isDownloading.value = true

  const folderPath = await createFolder(sec_uid, folderNameType, savePath)
  if (!folderPath) {
    isDownloading.value = false
    ElMessage.error(t('message.error_create_folder'))
    return
  }

  const videos = await getAllVideo(sec_uid, audio)
  total.value = videos.length

  console.log(videos)

  videos.forEach(async ({ desc, video }, i) => {
    const { vid, play_addr } = video
    const isAudio = !vid

    const url = isAudio ? play_addr.url_list[0] : generateVideoURL(vid, ratio, watermark)
    const ext = isAudio ? '.mp3' : '.mp4'

    let name = ''
    switch (fileNameType) {
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
        name = (i + 1).toString()
        break
    }

    invoke('download', { url, folder: folderPath, nameType: fileNameType, name, ext })
      .then(() => successCount.value++)
      .catch((error) => {
        failureCount.value++
        console.error(url, error)
      })
      .finally(() => percent.value === 100 && (isDownloading.value = false))
  })
}

const onCancelClick = () => {
  isDownloading.value = false
}

const createFolder = async (sec_uid: string, folderNameType: FolderNameType, savePath: string) => {
  const user = await getUser(sec_uid)
  if (!user) return

  const { uid, short_id, unique_id, nickname } = user

  let folderName = ''
  switch (folderNameType) {
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

  const folderPath = await resolve(savePath, folderName)
  const isExist = await exists(folderPath)
  !isExist && (await fs.createDir(folderPath, { recursive: true }))

  return folderPath
}

const getAllVideo = async (sec_uid: string, audio: boolean) => {
  const videos: Array<VideoItem> = []

  let data = await getVideos(sec_uid)
  data?.aweme_list.forEach((v) => !(!audio && !v.video.vid) && videos.push(v))

  while (data && data.has_more) {
    data = await getVideos(sec_uid, data.max_cursor.toString())
    data?.aweme_list.forEach((v) => !(!audio && !v.video.vid) && videos.push(v))
  }

  return videos
}
</script>
