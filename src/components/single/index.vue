<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { NameType } from '@/enums'
import { generateVideoURL } from '@/utils'
import { invoke, http, dialog } from '@tauri-apps/api'
import { basename } from '@tauri-apps/api/path'
import { ElButton, ElForm, ElFormItem, ElInput, ElProgress, ElDivider, ElMessage, ElRadioGroup, ElRadio, ElRadioButton } from 'element-plus'
import { computed, ref } from 'vue'

const form = ref({
  homeURL: '',
  savePath: '',
  ratio: '1080p',
  watermark: 0,
  nameType: NameType.TitleTag,
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
  const max_cursor = '0'

  if (sec_uid) {
    isDownloading.value = true
    requsetList(sec_uid, max_cursor)
  } else ElMessage.error('请输入正确的用户主页地址')
}

const onCancelClick = () => {
  isDownloading.value = false
}

const requsetList = (sec_uid: string, max_cursor: string) => {
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

        requsetList(sec_uid, data.max_cursor.toString())

        data.aweme_list.forEach(async (v: any, i: number) => {
          const { desc, video } = v
          const { vid } = video
          const isAudio = !vid

          /** skip audio */
          if (isAudio && !form.value.audio) return

          const url = isAudio ? video.play_addr.url_list[0] : generateVideoURL(vid, form.value.ratio, form.value.watermark)

          let name = ''
          switch (form.value.nameType) {
            case NameType.TitleTag:
              name = desc
              break
            case NameType.Title:
              const index = desc.indexOf('#')
              if (index === -1) name = desc.trim()
              else name = desc.substring(0, index).trim()
              break
            case NameType.ID:
              name = isAudio ? await basename(url, '.mp3') : vid
              break
            case NameType.Index:
              name = (total.value - data.aweme_list.length + i + 1).toString()
              break
          }

          const ext = isAudio ? '.mp3' : '.mp4'
          const path = `${form.value.savePath}/${name}${ext}`
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
