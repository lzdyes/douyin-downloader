<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { invoke, http, dialog } from '@tauri-apps/api'
import { ElButton, ElForm, ElFormItem, ElInput, ElProgress, ElDivider, ElMessage } from 'element-plus'
import { computed, ref } from 'vue'

const form = ref({
  homeURL: '',
  savePath: '',
})

const isDownloading = ref(false)

const total = ref(0)
const successCount = ref(0)
const failCount = ref(0)
const percent = computed(() => Math.floor(((successCount.value + failCount.value) / total.value) * 100) || 0)

let isListCompleted = false

const onSaveClick = async () => {
  const path = (await dialog.open({ directory: true })) as string
  path && (form.value.savePath = path)
}

const onSubmit = async () => {
  total.value = 0
  successCount.value = 0
  failCount.value = 0

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

        data.aweme_list.forEach(async (v: any) => {
          const name = v.desc
          const { vid } = v.video

          if (vid) {
            const url = `https://aweme.snssdk.com/aweme/v1/play/?video_id=${vid}&ratio=1080p`
            const path = `${form.value.savePath}/${name}.mp4`
            invoke('download', { url, path })
              .then(() => {
                successCount.value++
              })
              .catch((err) => {
                console.error(err)
                failCount.value++
              })
              .finally(() => {
                console.log('percent.value =', percent.value)

                if (isListCompleted && percent.value === 100) isDownloading.value = false
              })
          } else {
            console.error('vid is null', v)
            failCount.value++
          }
        })
      } else {
        console.log('over')
        isListCompleted = true

        if (percent.value === 100) isDownloading.value = false
      }
    })
    .catch((error) => console.error(error))
}
</script>
