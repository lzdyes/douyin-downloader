<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { ElButton, ElMessage, ElSelect, ElOption } from 'element-plus'
import { checkUpdate } from '@/utils'
import { app, shell } from '@tauri-apps/api'
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const languages = [
  { label: '中文', value: 'cn' },
  { label: 'English', value: 'en' },
]

const { t, locale } = useI18n()

const isCheckLoading = ref(false)
const version = ref('')

onMounted(async () => (version.value = await app.getVersion()))

const onGithubClick = () => shell.open('https://github.com/lzdyes/douyin-downloader')

const onCheckUpdateClick = async () => {
  isCheckLoading.value = true
  const shouldUpdate = await checkUpdate()
  !shouldUpdate && ElMessage.success(t('message.latest_version'))
  isCheckLoading.value = false
}

const onLanguageChange = (language: string) => localStorage.setItem('language', language)
</script>
