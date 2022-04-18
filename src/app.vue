<template lang="pug">
Menu
router-view(v-slot='{ Component }')
  keep-alive
    component(:is='Component')
Updater
</template>

<style lang="stylus">
html, body
  height 100%

#app
  height 100%
  display flex
</style>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { onMounted } from 'vue'
import Menu from './components/menu/index.vue'
import Updater from './components/updater/index.vue'

onMounted(() => invoke('show'))
</script>
