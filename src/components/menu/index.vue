<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import aboutFill from '@/assets/img/about-fill.svg'
import aboutLine from '@/assets/img/about-line.svg'
import videoFill from '@/assets/img/video-fill.svg'
import videoLine from '@/assets/img/video-line.svg'
import videosFill from '@/assets/img/videos-fill.svg'
import videosLine from '@/assets/img/videos-line.svg'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()

const menus = {
  top: [
    {
      path: '/',
      key: 'menu.all_video',
      icon: [videosLine, videosFill],
    },
    {
      path: '/single',
      key: 'menu.single_video',
      icon: [videoLine, videoFill],
    },
  ],
  bottom: [
    {
      path: '/about',
      key: 'menu.about',
      icon: [aboutLine, aboutFill],
    },
  ],
}

const isActive = (path: string) => path === route.path

const onMenuSwitch = (path: string) => router.push(path)
</script>
