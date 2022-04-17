<template lang="pug" src="./index.pug"></template>
<style lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import store from '@/store'
import video from '@/assets/svgs/video.svg'
import videos from '@/assets/svgs/videos.svg'

const menus = [
  {
    name: '全部视频',
    icon: videos,
  },
  {
    name: '单个视频',
    icon: video,
  },
]

const onMenuClick = (index: number) => store.setActiveMenu(index)
</script>
