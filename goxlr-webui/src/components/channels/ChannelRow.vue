<script>
import ScribbleImage from '@/components/channels/ScribbleImage.vue'
import ChannelVolume from '@/components/channels/ChannelVolume.vue'
import MuteState from '@/components/channels/MuteState.vue'
import ColourSettings from '@/components/channels/ColourSettings.vue'
import { store } from '@/goxlr/store.js'

export default {
  name: 'ChannelRow',
  components: { ColourSettings, MuteState, ChannelVolume, ScribbleImage },
  props: {
    channel: { type: String, required: false },
    title: { type: String, required: true },

    colour1: { type: String, required: false },
    colour2: { type: String, required: false }
  },

  data() {
    return {
      localValue: 50
    }
  },

  methods: {
    getColourOne: function () {
      return this.toRGB(this.getChannel().display.fader_colours.bottom_colour)
    },
    getColourTwo: function () {
      return this.toRGB(this.getChannel().display.fader_colours.top_colour)
    },
    toRGB: function (value) {
      return `rgb(${value.red}, ${value.green}, ${value.blue})`
    },

    getValue: function () {
      let value = this.getChannel().volume

      // Get this value as a percent of 255..
      return Math.round((value / 255) * 100)
    },

    isMutePressActive: function () {
      return this.getChannel().mute_state === 'Pressed'
    },
    isMuteHoldActive: function () {
      return this.getChannel().mute_state === 'Held'
    },
    getMutePressTargets: function () {
      return this.getChannel().mute_actions.Press
    },
    getMuteHoldTargets: function () {
      return this.getChannel().mute_actions.Hold
    },

    getChannel: function () {
      return store.getActiveDevice().config.profile.channels[this.getChannelName()]
    },
    getChannelName: function () {
      return this.channel === undefined ? this.title : this.channel
    }
  }
}
</script>

<template>
  <div class="stuff" style="display: flex; flex-direction: row">
    <ScribbleImage :colour="getColourOne()" />
    <div style="width: 120px; color: #fff">
      {{ title }}
    </div>
    <ChannelVolume
      :current-value="getValue()"
      :colour1="getColourOne()"
      :colour2="getColourTwo()"
    />
    <MuteState :selected="isMutePressActive()" :targets="getMutePressTargets()" />
    <MuteState :selected="isMuteHoldActive()" :targets="getMuteHoldTargets()" />
    <ColourSettings />
  </div>
</template>

<style scoped>
.stuff {
  height: 40px;
  display: flex;
  flex-direction: row;
  gap: 20px;
  margin: 10px;
}

.stuff > div {
  margin-top: auto;
  margin-bottom: auto;
}
</style>
