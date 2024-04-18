<script>
/**
 * This is a 'Two Part' component, slider on the left and a text input on the right, both are synced here.
 */
import RangeSelector from '@/components/slider/RangeSelector.vue'

export default {
  name: 'ChannelRowVolume',
  components: { RangeSelector },
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
  <div style="margin-top: auto; margin-bottom: auto">
    <RangeSelector
      id="channel"
      aria-description=""
      aria-label=""
      aria-value=""
      :current-value="localFieldValue"
      :max-value="100"
      :min-value="0"
      v-on:input="change"
      :selected-colour="colour1"
      :deselected-colour="colour2"
    />
  </div>
  <div style="margin-top: auto; margin-bottom: auto">
    <input type="number" :min="0" :max="100" v-model="localFieldValue" />
  </div>
</template>

<style scoped>
input[type='number'] {
  width: 50px;

  background-color: #3b413f;
  color: #59b1b6;
  border: 1px solid #59b1b6;

  appearance: textfield;
  -moz-appearance: textfield;
}

input[type='number']::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
</style>
