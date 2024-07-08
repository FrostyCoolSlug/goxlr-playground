<script setup>
import Pages from '@/components/Pages.vue'
import ChannelColumn from '@/components/channels/ChannelColumn.vue'
import IconCard from '@/components/card/IconCard.vue'
import { store } from '@/goxlr/store.js'

function subMixEnabled() {
  console.log(store.getActiveDevice())
  return store.getActiveDevice().hardware.features.includes('SubMix')
}
</script>

<template>
  <div class="wrapper">
    <div class="mixer">
      <ChannelColumn title="Microphone" />
      <ChannelColumn title="Chat" />
      <ChannelColumn title="Music" />
      <ChannelColumn title="Game" />
      <ChannelColumn title="Console" />
      <ChannelColumn title="Line In" channel="LineIn" />
      <ChannelColumn title="System" />
      <ChannelColumn title="Sample" />
    </div>
    <div class="mixer">
      <ChannelColumn title="Headphones" />
      <ChannelColumn title="Line Out" channel="LineOut" />
      <ChannelColumn v-if="!subMixEnabled()" title="Mic Monitor" channel="MicrophoneMonitor" />
    </div>
  </div>
  <Pages />

  <IconCard />
</template>

<style scoped>
.wrapper {
  display: flex;
  flex-direction: row;
  gap: 20px;
}

.mixer {
  background-color: #2d3230;
  padding: 15px;
  display: flex;
  flex-direction: row;
  gap: 25px;
}
</style>
