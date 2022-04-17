<template lang="pug">
Main
Updater
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { onMounted } from 'vue'
import Main from './components/main/index.vue'
import Updater from './components/updater/index.vue'

onMounted(() => invoke('show'))
</script>
