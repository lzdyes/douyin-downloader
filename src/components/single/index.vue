<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { getVideo } from '@/api'
import { FileNameType } from '@/enums'
import { formatSize, generateVideoURL } from '@/utils'
import { dialog, invoke } from '@tauri-apps/api'
import { basename } from '@tauri-apps/api/path'
import { ElButton, ElForm, ElFormItem, ElInput, ElMessage, ElRadioGroup, ElRadio, ElRadioButton } from 'element-plus'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const form = ref({
  videoURL: '',
  ratio: '1080p',
  watermark: 0,
  audio: false,
  fileNameType: FileNameType.TitleTag,
  savePath: '',
})

const isDownloading = ref(false)
const diskFreeSize = ref('')
const status = ref('')

const onSaveClick = async () => {
  const path = (await dialog.open({ directory: true })) as string
  path && (form.value.savePath = path)

  const freeSize = (await invoke('disk_free_size', { path })) as number
  diskFreeSize.value = formatSize(freeSize)
}

const onSubmit = async () => {
  status.value = ''

  const { videoURL, ratio, watermark, audio, fileNameType, savePath } = form.value

  if (!videoURL) return ElMessage.error(t('message.empty_video_url'))
  if (!savePath) return ElMessage.error(t('message.empty_sava_path'))

  const { searchParams } = new URL(videoURL)
  const id = searchParams.get('modal_id')
  if (!id) return ElMessage.error(t('message.error_video_url'))

  isDownloading.value = true
  status.value = t('video.downloading')

  const data = await getVideo(id)
  if (!data) {
    isDownloading.value = false
    status.value = ''
    return ElMessage.error(t('message.error_video_info'))
  }

  const { desc, video } = data
  const { vid } = video
  const isAudio = !vid

  if (isAudio && !audio) return

  const url = isAudio ? video.play_addr.url_list[0] : generateVideoURL(vid, ratio, watermark)
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
  }

  invoke('download', { url, folder: savePath, nameType: fileNameType, name, ext })
    .then(() => (status.value = t('video.download_success')))
    .catch(() => (status.value = t('video.download_failure')))
    .finally(() => (isDownloading.value = false))
}

const onCancelClick = () => {
  isDownloading.value = false
}
</script>
