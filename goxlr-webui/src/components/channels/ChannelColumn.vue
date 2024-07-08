<script>
import ColourSettings from '@/components/channels/ColourSettings.vue'
import MuteState from '@/components/channels/MuteState.vue'
import ChannelRowVolume from '@/components/channels/ChannelRowVolume.vue'
import ScribbleImage from '@/components/channels/ScribbleImage.vue'
import { store } from '@/goxlr/store.js'
import { isDeviceMini } from '@/goxlr/util.js'
import ChannelColumnVolume from '@/components/channels/ChannelColumnVolume.vue'

export default {
  name: 'ChannelColumn',
  components: { ChannelColumnVolume, ColourSettings, MuteState, ChannelRowVolume, ScribbleImage },
  props: {
    channel: { type: String, required: false },
    title: { type: String, required: true }
  },

  data() {
    return {
      localValue: 50,
      window_size: window.innerHeight
    }
  },

  mounted() {
    this.$nextTick(() => {
      window.addEventListener('resize', this.onResize)
    })
  },

  beforeUnmount() {
    window.removeEventListener('resize', this.onResize)
  },

  methods: {
    onResize: function () {
      this.window_size = window.innerHeight
    },

    getTopColour: function () {
      if (isDeviceMini()) {
        return {
          red: 102,
          green: 102,
          blue: 102
        }
      }

      // Only assignable channels have screens..
      let config = store.getActiveDevice().config.device.channels.configs[this.getChannelName()]
      if (config === undefined) {
        return {
          red: 102,
          green: 102,
          blue: 102
        }
      }

      return config.display.screen_display.colour
    },
    getBottomColour: function () {
      let config = store.getActiveDevice().config.device.channels.configs[this.getChannelName()]
      if (config === undefined) {
        return {
          red: 102,
          green: 102,
          blue: 102
        }
      }

      // Get the Mute Behaviours
      let mute_state = config.mute_state
      let mute_colours = config.display.mute_colours

      return mute_colours.active_colour
    },

    isMutePressActive: function () {
      return this.getMute().mute_state === 'Pressed'
    },
    isMuteHoldActive: function () {
      return this.getMute().mute_state === 'Held'
    },
    getMutePressTargets: function () {
      return this.getMute().mute_actions.Press
    },
    getMuteHoldTargets: function () {
      return this.getMute().mute_actions.Hold
    },

    calculateHeight: function () {
      // We'll start with a base 'full' slider height
      let size = Math.max(this.window_size - 400, 220)

      // If sub-mixes are available, cut 30px for the link icon
      if (this.submixEnabled() && this.hasMix()) {
        size -= 30
      }

      // Cut 30 for the first button (if applicable)
      if (this.hasBasicMute()) {
        size -= 30
      }

      // Cut 30 for the Second Button (if applicable)
      if (this.hasComplexMute()) {
        size -= 30
      }

      // If we're showing two buttons, cut 5 more for the 'gap' between them
      if (this.hasBasicMute() && this.hasComplexMute()) {
        size -= 5
      }

      // Done :)
      return size
    },

    getVolume: function () {
      return store.getActiveDevice().config.device.channels.volumes[this.getChannelName()]
    },
    getMixVolume: function () {
      let mix = store.getActiveDevice().config.device.channels.sub_mix[this.getChannelName()]
      if (mix === undefined) {
        return 0
      }

      return mix.volume
    },
    getMute: function () {
      return store.getActiveDevice().config.device.channels.configs[this.getChannelName()]
    },

    getChannelName: function () {
      return this.channel === undefined ? this.title : this.channel
    },

    submixEnabled: function () {
      return store.getActiveDevice().config.device.configuration.submix_enabled
    },
    hasMix: function () {
      let mix = store.getActiveDevice().config.device.channels.sub_mix[this.getChannelName()]
      return mix !== undefined
    },
    isLinked: function () {
      let mix = store.getActiveDevice().config.device.channels.sub_mix[this.getChannelName()]
      return mix.linked === 1
    },
    canAssign: function () {
      let config = store.getActiveDevice().config.device.channels.configs[this.getChannelName()]
      return config !== undefined
    },
    hasBasicMute: function () {
      return this.canAssign()
    },
    hasComplexMute: function () {
      let ch = this.getChannelName()
      let config = store.getActiveDevice().config.device.channels.mute_actions[ch]
      return config !== undefined
    },

    rgbToHex(r, g, b) {
      return '#' + ((1 << 24) | (r << 16) | (g << 8) | b).toString(16).slice(1)
    }
  },
  computed: {
    topColour: function () {
      let colour = this.getTopColour()
      return `rgb(${colour.red}, ${colour.green}, ${colour.blue})`
    },

    bottomColour: function () {
      let colour = this.getBottomColour()
      return `rgb(${colour.red}, ${colour.green}, ${colour.blue})`
    },

    titleBackground: function () {
      // Get the Screen colour..
      let colour = this.getTopColour()
      let base = `rgba(${colour.red}, ${colour.green}, ${colour.blue}, 0.1)`
      return `linear-gradient(rgba(0,0,0,0), ${base})`
    },

    muteBackground: function () {
      let colour = this.getBottomColour()
      let base = `rgba(${colour.red}, ${colour.green}, ${colour.blue}, 0.3)`
      return `linear-gradient(${base}, rgba(0,0,0,0))`
    },

    topHeight: function () {
      if (isDeviceMini()) {
        return '1px'
      }
      return '5px'
    }
  }
}
</script>

<template>
  <div ref="mix" class="mix">
    <div class="title">{{ title }}</div>
    <div class="top"></div>
    <div class="faders">
      <ChannelColumnVolume
        :height="calculateHeight()"
        :current-value="getVolume()"
        colour1="#59b1b6"
        colour2="#252927"
      />
      <ChannelColumnVolume
        v-if="submixEnabled() && hasMix()"
        :height="calculateHeight()"
        :current-value="getMixVolume()"
        colour1="#E07C24"
        colour2="#252927"
      />
    </div>
    <div class="link" v-if="submixEnabled() && hasMix()">
      <img v-if="isLinked()" src="/images/submix/linked-white.png" alt="Linked" />
      <img v-else src="/images/submix/unlinked-dimmed.png" alt="Unlinked" />
    </div>
    <div class="bottom"></div>
    <div class="mute" v-if="hasComplexMute()">
      <div class="buttons">
        <div class="fill">
          <span style="display: inline-block; margin-left: 4px; margin-right: 5px">
            <img src="/images/hold.svg" alt="Press" style="width: 24px; fill: #fff" />
          </span>
          <span>Mute to All</span>
        </div>
        <div>
          <font-awesome-icon :icon="['fas', 'angle-down']" />
        </div>
      </div>
      <div class="buttons">
        <div class="fill">
          <span style="display: inline-block; margin-left: 4px; margin-right: 5px">
            <img src="/images/press.svg" alt="Press" style="width: 24px; fill: #fff" />
          </span>
          <span class="label">Mute to Headphones</span>
        </div>
        <div>
          <font-awesome-icon :icon="['fas', 'angle-down']" />
        </div>
      </div>
    </div>
    <div class="mute small" v-else-if="hasBasicMute()">
      <div class="buttons">
        <div class="fill">
          <span style="display: inline-block; margin-left: 4px; margin-right: 5px">
            <img src="/images/hold.svg" alt="Press" style="width: 24px; fill: #fff" />
          </span>
          <span>Mute to All</span>
        </div>
        <div>
          <font-awesome-icon :icon="['fas', 'angle-down']" />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mix {
  min-width: 150px;
  background-color: #353937;
  border: 1px solid #666666;
  border-radius: 5px;
}

.title {
  padding: 8px;
  text-align: center;
  font-weight: bold;
  background: v-bind(titleBackground);
}

.top {
  background-color: v-bind(topColour);
  height: v-bind(topHeight);
}

.faders {
  padding: 15px;
  display: flex;
  flex-direction: row;
  justify-content: center;
  gap: 35px;
}

.link {
  text-align: center;
  height: 30px;
}

.link img {
  height: 20px;
}

.bottom {
  background-color: v-bind(bottomColour);
  height: 5px;
}

.mute {
  height: 65px;
  background: v-bind(muteBackground);

  display: flex;
  flex-direction: column;
  gap: 5px;
}

.mute.small {
  height: 30px;
}

.buttons {
  display: flex;
  flex-direction: row;
}

.buttons .fill {
  flex: 1;
}

.buttons .fill button {
  width: 100%;
}

.mute > div {
  font-size: 1em;
  flex-grow: 1;
}

.mute .buttons div {
  background-color: rgba(80, 80, 80, 0.8);
  overflow: hidden;

  border: 1px solid #666;
  border-left: 0;
  border-right: 0;

  display: flex;
  align-items: center;
}

.label {
  width: 90px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  text-align: left;
}

.mute .buttons div:first-child {
  border-right: 1px solid #666;
}

.mute .buttons div:last-child {
  padding: 4px;
  border-left: 1px solid #555;
}

.mute :first-child > div {
  border-top: 0;
}

.mute :last-child > div {
  border-bottom: 0;
}
</style>
