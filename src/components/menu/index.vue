<template lang="pug" src="./index.pug"></template>
<style scoped lang="stylus" src="./index.styl"></style>
<script setup lang="ts">
import store from '@/store'
import videoLine from '@/assets/img/video-line.svg'
import videoFill from '@/assets/img/video-fill.svg'
import videosLine from '@/assets/img/videos-line.svg'
import videosFill from '@/assets/img/videos-fill.svg'
import aboutLine from '@/assets/img/about-line.svg'
import aboutFill from '@/assets/img/about-fill.svg'

enum MenuType {
  Multiple,
  Single,
  About,
}

const menus = {
  top: [
    {
      type: MenuType.Multiple,
      name: '全部视频',
      icon: [videosLine, videosFill],
    },
    {
      type: MenuType.Single,
      name: '单个视频',
      icon: [videoLine, videoFill],
    },
  ],
  bottom: [
    {
      type: MenuType.About,
      name: '关于',
      icon: [aboutLine, aboutFill],
    },
  ],
}

const isActive = (type: MenuType) => store.state.activeMenu === type

const onMenuClick = (index: number) => store.setActiveMenu(index)
</script>
