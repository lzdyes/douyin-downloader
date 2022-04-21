<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { getVideo } from '@/api'
import { FileNameType } from '@/enums'
import { generateVideoURL } from '@/utils'
import { dialog, http, invoke } from '@tauri-apps/api'
import { basename } from '@tauri-apps/api/path'
import { ElButton, ElForm, ElFormItem, ElInput, ElMessage, ElRadioGroup, ElRadio, ElRadioButton } from 'element-plus'
import { ref } from 'vue'

interface DownloadParams {
  id: string
  ratio: string
  watermark: number
  audio: boolean
  nameType: FileNameType
  savePath: string
}

const form = ref({
  url: '',
  ratio: '1080p',
  watermark: 0,
  audio: false,
  nameType: FileNameType.TitleTag,
  savePath: '',
})

const isDownloading = ref(false)
const status = ref('')

const onSaveClick = async () => {
  const path = (await dialog.open({ directory: true })) as string
  path && (form.value.savePath = path)
}

const onSubmit = async () => {
  status.value = ''

  if (!form.value.url) return ElMessage.error('请输入视频地址')
  if (!form.value.savePath) return ElMessage.error('请选择保存位置')

  const { searchParams } = new URL(form.value.url)
  const id = searchParams.get('modal_id')

  if (id) {
    isDownloading.value = true
    status.value = '下载中'
    downloadVideo({ id, ...form.value })
  } else ElMessage.error('请输入正确的视频地址')
}

const onCancelClick = () => {
  isDownloading.value = false
}

const downloadVideo = async (parms: DownloadParams) => {
  const { id, ratio, watermark, audio, nameType, savePath } = parms
  const data = await getVideo(id)
  if (data) {
    const { desc, video } = data
    const { vid } = video
    const isAudio = !vid

    if (isAudio && !audio) return

    const url = isAudio ? video.play_addr.url_list[0] : generateVideoURL(vid, ratio, watermark)

    let name = ''
    switch (nameType) {
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

    const ext = isAudio ? '.mp3' : '.mp4'
    const path = `${savePath}/${name}${ext}`
    invoke('download', { url, path })
      .then(() => (status.value = '下载成功'))
      .catch(() => (status.value = '下载失败'))
      .finally(() => (isDownloading.value = false))
  } else ElMessage.error('获取视频信息失败')
}
</script>
