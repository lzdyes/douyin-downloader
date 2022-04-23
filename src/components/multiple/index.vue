<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { getUser, getVideos } from '@/api'
import { FileNameType, FolderNameType } from '@/enums'
import { exists, generateVideoURL } from '@/utils'
import { dialog, fs, invoke } from '@tauri-apps/api'
import { basename, resolve } from '@tauri-apps/api/path'
import { ElButton, ElForm, ElFormItem, ElInput, ElProgress, ElDivider, ElMessage, ElRadioGroup, ElRadio, ElRadioButton } from 'element-plus'
import { computed, ref } from 'vue'

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

const total = ref(0)
const successCount = ref(0)
const failureCount = ref(0)
const percent = computed(() => Math.floor(((successCount.value + failureCount.value) / total.value) * 100) || 0)

const onSaveClick = async () => {
  const path = (await dialog.open({ directory: true })) as string
  path && (form.value.savePath = path)
}

const onSubmit = async () => {
  total.value = 0
  successCount.value = 0
  failureCount.value = 0

  const { homeURL, ratio, watermark, audio, folderNameType, fileNameType, savePath } = form.value

  if (!homeURL) return ElMessage.error('请输入用户主页地址')
  if (!savePath) return ElMessage.error('请选择保存位置')

  const sec_uid = homeURL.split('/').pop()
  if (!sec_uid) return ElMessage.error('请输入正确的用户主页地址')

  isDownloading.value = true

  const folderPath = await createFolder(sec_uid, folderNameType, savePath)
  if (!folderPath) {
    isDownloading.value = false
    ElMessage.error('文件夹创建失败，请重试')
    return
  }

  const videos = await getAllVideo(sec_uid)
  total.value = videos.length

  console.log(videos)

  videos.forEach(async ({ desc, video }, i) => {
    const { vid, play_addr } = video
    const isAudio = !vid

    if (isAudio && !audio) return

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
        name = i.toString()
        break
    }

    invoke('download', { url, folder: folderPath, nameType: fileNameType, name, ext })
      .then(() => successCount.value++)
      .catch(() => failureCount.value++)
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

const getAllVideo = async (sec_uid: string) => {
  const videos = []

  let data = await getVideos(sec_uid)
  data && videos.push(...data.aweme_list)

  while (data && data.has_more) {
    data = await getVideos(sec_uid, data.max_cursor.toString())
    data && videos.push(...data.aweme_list)
  }

  return videos
}
</script>
