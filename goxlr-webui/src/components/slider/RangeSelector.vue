<script>
/**
 * The Range Selector at its core is a simple <input type="range" />, which is designed to fill all available space
 * in the containing parent. For practicality purposes this component no longer supports vertical orientation, if this
 * is needed, it should be managed and rotated by its parent container.
 *
 * This component is purely a presentation component, events for value changes and other events should be passed in
 * by their parents, and handled there. We simply draw it, and leave everything else.
 *
 * NOTE: Vertical Sliders are currently a nightmare in HTML, a lot of browsers don't support them, and even those
 * which do often cannot style them properly. A transform: rotate(-90) is the best way to handle this, but that may
 * also require manually implementing touch controls for chrome mobile... You have been warned!
 */
export default {
  name: "RangeSelector",

  data() {
    return {
      localFieldValue: 0,
    }
  },

  props: {
    // Minimum Value for the Slider
    minValue: {type: Number, required: true},

    // Maximum Value for the Slider
    maxValue: {type: Number, required: true},

    // The current value of the Slider
    currentValue: {type: Number, required: true},

    // The stepping of the input.
    step: {type: Number, required: false, default: 1},

    // Whether the control is disabled
    disabled: {type: Boolean, required: false, default: false},

    // A Unique Identifier for reporting value changes
    id: {type: String, required: true},

    // Colours for the thumb and 'active' section, and the unselected colour
    selectedColour: {type: String, required: false, default: "#82CFD0"},
    deselectedColour: {type: String, required: false, default: "#252927"},

    // The value to report to Screen Readers
    ariaValue: {type: String, required: true},
    ariaLabel: {type: String, required: true},
    ariaDescription: {type: String, required: true},
  },

  methods: {
  },

  watch: {
    /**
     * Because changes can come from either the user interacting with the slider, or a reactive change coming from
     * elsewhere (Generally a value change in the Store), localFieldValue is used as a bind between them both.
     *
     * Here we watch for external changes, and update the local value to resync the slider to its new position.
     */
    currentValue: function (newValue) {
      this.localFieldValue = newValue;
    }
  },
  mounted() {
    this.localFieldValue = this.currentValue;
  },
  computed: {
    currentWidth() {
      // This code essentially adjusts the background position to keep it below the 'thumb'..
      let distance = this.maxValue - this.minValue;
      let position = 0;

      for (let i = this.minValue; i <= this.maxValue; i += this.step, position += this.step) {
        if (i === parseFloat(this.localFieldValue)) {
          break;
        }
      }

      let width = (position / distance) * 100;
      return width + "%";
    }
  }
}
</script>

<template>
  <input class="slider" ref="slider" type="range" v-bind:min="minValue" v-bind:max="maxValue"
         v-model="localFieldValue" v-bind:step="step" v-bind:disabled="disabled"
         :aria-label="ariaLabel" :aria-description="ariaDescription" :aria-valuetext="ariaValue"

  />
</template>

<style scoped>
.slider {
  background: linear-gradient(
      to right,
      v-bind(selectedColour) 0%,
      v-bind(selectedColour) v-bind(currentWidth),
      v-bind(deselectedColour) v-bind(currentWidth),
      v-bind(deselectedColour) 100%
  );

  position: relative;
  border-radius: 2px;
  height: 3px;
  width: 100%;
  outline: none;
  transition: background 450ms ease-in;

  appearance: none;
  -webkit-appearance: none;

  display: block;
  touch-action: none;
}

input[type='range']::-webkit-slider-thumb {
  width: 16px;
  height: 16px;
  border-radius: 8px;
  background: v-bind(selectedColour);
  -webkit-appearance: none;
}

input[type='range']::-moz-range-thumb {
  width: 16px;
  height: 16px;
  border-radius: 7px;
  background: v-bind(selectedColour);
  border: 0;
}
</style>