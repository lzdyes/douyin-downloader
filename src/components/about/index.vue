<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { ElButton } from 'element-plus'
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
  await checkUpdate()
  isCheckLoading.value = false
}
</script>
