<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { ElButton, ElMessage } from 'element-plus'
import { checkUpdate } from '@/utils'
import { app, shell } from '@tauri-apps/api'
import { onMounted, ref } from 'vue'

const isCheckLoading = ref(false)
const version = ref('')

onMounted(async () => {
  version.value = await app.getVersion()
})

const onGithubClick = () => shell.open('https://github.com/lzdyes/douyin-downloader')

const onCheckUpdateClick = async () => {
  isCheckLoading.value = true
  const shouldUpdate = await checkUpdate()
  !shouldUpdate && ElMessage.success('已是最新版本')
  isCheckLoading.value = false
}
</script>
