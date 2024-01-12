<script setup>
import { RouterView } from 'vue-router'
import { runWebsocket } from '@/goxlr/sockets.js'
import { store } from '@/goxlr/store.js'
import DeviceSelector from '@/views/DeviceSelector.vue'

runWebsocket()

const isDeviceSet = () => {
  return store.hasActiveDevice() && store.isConnected()
}
</script>

<template>
  <DeviceSelector v-if="!isDeviceSet()" />

  <RouterView v-if="isDeviceSet()" />
</template>

<style scoped></style>
