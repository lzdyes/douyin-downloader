<template lang="pug" src="./index.pug"></template>
<style lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { relaunch } from '@tauri-apps/api/process'
import { installUpdate } from '@tauri-apps/api/updater'
import { onMounted, ref } from 'vue'
import { ElButton, ElDialog } from 'element-plus'
import { checkUpdate } from '@/utils'
import store from '@/store'

onMounted(() => checkUpdate())

const onCancelClick = () => {
  store.setUpdater({ active: false })
}

const onUpdateClick = async () => {
  await installUpdate()
  await relaunch()
}
</script>
