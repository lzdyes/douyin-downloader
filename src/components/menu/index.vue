<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import aboutFill from '@/assets/img/about-fill.svg'
import aboutLine from '@/assets/img/about-line.svg'
import videoFill from '@/assets/img/video-fill.svg'
import videoLine from '@/assets/img/video-line.svg'
import videosFill from '@/assets/img/videos-fill.svg'
import videosLine from '@/assets/img/videos-line.svg'
import { useRoute, useRouter } from 'vue-router'

const router = useRouter()
const route = useRoute()

const menus = {
  top: [
    {
      path: '/',
      name: '全部视频',
      icon: [videosLine, videosFill],
    },
    {
      path: '/single',
      name: '单个视频',
      icon: [videoLine, videoFill],
    },
  ],
  bottom: [
    {
      path: '/about',
      name: '关于',
      icon: [aboutLine, aboutFill],
    },
  ],
}

const isActive = (path: string) => path === route.path

const onMenuSwitch = (path: string) => router.push(path)
</script>
