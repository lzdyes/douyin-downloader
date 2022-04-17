<template lang="pug" src="./index.pug"></template>
<style lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import { relaunch } from '@tauri-apps/api/process'
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { onMounted, ref } from 'vue'
import { ElButton, ElDialog } from 'element-plus'

const isDialogVisible = ref(false)
const version = ref('')

onMounted(() => {
  checkUpdate()
    .then(({ shouldUpdate, manifest }) => {
      version.value = manifest!.version
      isDialogVisible.value = shouldUpdate
    })
    .catch((error) => console.error(error))
})

const onUpdateClick = async () => {
  await installUpdate()
  await relaunch()
}
</script>
