<script>
import ColourSettings from '@/components/channels/ColourSettings.vue'
import MuteState from '@/components/channels/MuteState.vue'
import ChannelRowVolume from '@/components/channels/ChannelRowVolume.vue'
import ScribbleImage from '@/components/channels/ScribbleImage.vue'
import { store } from '@/goxlr/store.js'
import ChannelColumnVolume from '@/components/channels/ChannelColumnVolume.vue'

export default {
  name: 'ChannelColumn',
  components: { ChannelColumnVolume, ColourSettings, MuteState, ChannelRowVolume, ScribbleImage },
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
      let colour = this.getChannel().display.fader_colours.bottom_colour
      return this.rgbToHex(colour.red, colour.green, colour.blue)
    },
    getColourTwo: function () {
      let colour = this.getChannel().display.fader_colours.top_colour
      return this.rgbToHex(colour.red, colour.green, colour.blue)
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
      console.log(store.getActiveDevice().config.profile.channels[this.getChannelName()])
      return store.getActiveDevice().config.profile.channels[this.getChannelName()]
    },
    getChannelName: function () {
      return this.channel === undefined ? this.title : this.channel
    },

    rgbToHex(r, g, b) {
      return '#' + ((1 << 24) | (r << 16) | (g << 8) | b).toString(16).slice(1)
    }
  }
}
</script>

<template>
  <div>
    <ChannelColumnVolume
      :current-value="getValue()"
      :colour1="getColourOne()"
      :colour2="getColourTwo()"
    />
  </div>
</template>

<style scoped></style>
