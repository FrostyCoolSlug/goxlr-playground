<script>
import VerticalRange from '@/components/inputs/VerticalRange.vue'

export default {
  name: 'ChannelColumnVolume',

  components: { VerticalRange },
  data() {
    return {
      localFieldValue: 50
    }
  },

  props: {
    currentValue: { type: Number, required: true },
    colour1: { type: String, default: '#00ffff' },
    colour2: { type: String, default: '#252927' }
  },

  methods: {
    change(e) {
      this.localFieldValue = parseInt(e.target.value)
    }
  },

  watch: {
    /**
     * Because changes can come from either the user interacting with the slider, or a reactive change coming from
     * elsewhere (Generally a value change in the Store), localFieldValue is used as a bind between them both.
     *
     * Here we watch for external changes, and update the local value to resync the slider to its new position.
     */
    currentValue: function (newValue) {
      this.localFieldValue = newValue
    }
  },

  mounted() {
    this.localFieldValue = this.currentValue
  }
}
</script>

<template>
  <VerticalRange
    id="channel"
    aria-description=""
    aria-label=""
    aria-value=""
    :current-value="localFieldValue"
    :max-value="100"
    :min-value="0"
    :selected-colour="colour1"
    :deselected-colour="colour2"
    :height="220"
  />
</template>

<style scoped></style>
