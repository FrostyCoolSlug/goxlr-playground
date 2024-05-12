<script setup>
import Pages from '@/components/Pages.vue'
import Meter from '@/components/Meter.vue'
import ChannelColumn from '@/components/channels/ChannelColumn.vue'
import IconRail from '@/components/card/IconRail.vue'
import IconCard from '@/components/card/IconCard.vue'
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
      <ChannelColumn title="Mic Monitor" channel="MicrophoneMonitor" />
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
