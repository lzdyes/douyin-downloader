<template lang="pug">
Menu
component(:is='components[store.state.activeMenu]')
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
import store from '@/store'
import { invoke } from '@tauri-apps/api/tauri'
import { onMounted } from 'vue'
import About from './components/about/index.vue'
import Menu from './components/menu/index.vue'
import Multiple from './components/multiple/index.vue'
import Single from './components/single/index.vue'
import Updater from './components/updater/index.vue'

const components = [Multiple, Single, About]

onMounted(() => invoke('show'))
</script>
