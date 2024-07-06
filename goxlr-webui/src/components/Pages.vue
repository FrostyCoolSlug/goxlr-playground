<script>
import { store } from '@/goxlr/store.js'

export default {
  name: 'Pages',

  methods: {
    getPages: function () {
      return store.getActiveDevice().config.device.pages.page_list
    },
    getActivePage: function () {
      return store.getActiveDevice().config.device.pages.current
    }
  }
}
</script>

<template>
  <h2 style="color: #fff">Pages:</h2>
  <div class="pages" v-for="(page, index) in getPages()" style="color: #fff">
    <div style="width: 20px">{{ index }}</div>
    <div style="width: 80px"><span v-if="getActivePage() === index">*ACTIVE*</span></div>
    <div style="width: 200px">A: {{ page.faders.A }}</div>
    <div style="width: 200px">B: {{ page.faders.B }}</div>
    <div style="width: 200px">C: {{ page.faders.C }}</div>
    <div style="width: 200px">D: {{ page.faders.D }}</div>
  </div>
</template>

<style scoped>
.pages > div {
  display: inline-block;
}
</style>
