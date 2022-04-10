<template lang="pug">
Main
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { onMounted } from 'vue'
import Main from './components/main/index.vue'

onMounted(() => invoke('show'))
</script>
