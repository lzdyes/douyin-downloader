<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { FileNameType } from '@/enums'
import { generateVideoURL } from '@/utils'
import { dialog, http, invoke } from '@tauri-apps/api'
import { basename } from '@tauri-apps/api/path'
import { ElButton, ElForm, ElFormItem, ElInput, ElMessage, ElRadioGroup, ElRadio, ElRadioButton } from 'element-plus'
import { ref } from 'vue'

const form = ref({
  url: '',
  savePath: '',
  ratio: '1080p',
  watermark: 0,
  nameType: FileNameType.TitleTag,
  audio: false,
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
  const item_ids = searchParams.get('modal_id')

  if (item_ids) {
    isDownloading.value = true
    status.value = '下载中'
    requestVideo(item_ids)
  } else ElMessage.error('请输入正确的视频地址')
}

const onCancelClick = () => {
  isDownloading.value = false
}

const requestVideo = (item_ids: string) => {
  const url = 'https://www.douyin.com/web/api/v2/aweme/iteminfo'

  http
    .fetch(url, { method: 'GET', query: { item_ids } })
    .then(async (response) => {
      console.log(response)

      const data = response.data as any

      if (data.status_code === 0) {
        const { desc, video } = data.item_list[0]
        const { vid } = video
        const isAudio = !vid

        /** skip audio */
        if (isAudio && !form.value.audio) return

        const url = isAudio ? video.play_addr.url_list[0] : generateVideoURL(vid, form.value.ratio, form.value.watermark)

        let name = ''
        switch (form.value.nameType) {
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
        const path = `${form.value.savePath}/${name}${ext}`
        invoke('download', { url, path })
          .then(() => (status.value = '下载成功'))
          .catch(() => (status.value = '下载失败'))
          .finally(() => (isDownloading.value = false))
      }
    })
    .catch((error) => console.error(error))
}
</script>
