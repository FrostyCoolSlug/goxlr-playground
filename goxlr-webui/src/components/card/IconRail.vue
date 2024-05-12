<script>
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

export default {
  name: 'IconRail',
  components: { FontAwesomeIcon },

  data() {
    return {
      active: 1,
      buttons: [
        {
          icon: ['fas', 'volume-xmark']
        },
        {
          icon: ['fas', 'palette']
        }
      ]
    }
  },

  methods: {
    setActive: function (index) {
      this.active = index
    }
  }
}
</script>

<template>
  <div class="rail">
    <div
      v-for="(icon, index) in buttons"
      :key="index"
      class="button"
      :class="{ active: index === active }"
      @click="setActive(index)"
    >
      <font-awesome-icon :icon="icon.icon" />
    </div>
  </div>
</template>

<style scoped>
.rail {
  width: 30px;
  display: flex;
  flex-direction: column;
}

.button {
  width: 30px;
  height: 30px;
  font-size: 20px;

  display: flex;
  align-items: center;
  justify-content: center;

  border-bottom: 1px solid #252927;
}

.button:last-child {
  border-bottom: 0;
}

.button.active {
  border-right: 3px solid #59b1b6;
}

.button:not(.active) {
  border-right: 3px solid #666;
}
</style>
