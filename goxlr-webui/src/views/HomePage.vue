<script setup>
import ChannelRow from '@/components/channels/ChannelRow.vue'
import Pages from '@/components/Pages.vue'
import Meter from '@/components/Meter.vue'
</script>

<template>
  <ChannelRow title="Microphone" colour1="#0000ff" colour2="#000050" />
  <ChannelRow title="Chat" colour1="#00ff00" colour2="#005000" />
  <ChannelRow title="Music" colour1="#00ffff" colour2="#000000" />
  <ChannelRow title="Game" colour1="#82CFD0" colour2="#252927" />
  <ChannelRow title="Console" colour1="#82CFD0" colour2="#252927" />
  <ChannelRow title="Line In" channel="LineIn" colour1="#82CFD0" colour2="#252927" />
  <ChannelRow title="System" colour1="#ff0000" colour2="#500000" />
  <ChannelRow title="Sample" colour1="#ff0000" colour2="#500000" />
  <ChannelRow title="Headphones" colour1="#ff0000" colour2="#500000" />
  <ChannelRow title="Line Out" channel="LineOut" colour1="#ff0000" colour2="#500000" />
  <ChannelRow title="Mic Monitor" channel="MicrophoneMonitor" colour1="#ff0000" colour2="#500000" />

  <Pages />

  <Meter />
</template>

<style scoped></style>
