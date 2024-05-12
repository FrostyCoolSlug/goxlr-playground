<script>
import IconRail from '@/components/card/IconRail.vue'

export default {
  name: 'IconCard',
  components: { IconRail }
}
</script>

<template>
  <div class="card">
    <IconRail class="rail" />
    <div class="content">Hello World!</div>
  </div>
</template>

<style scoped>
.card {
  background-color: #353937;
  width: 400px;
  border: 1px solid #666;
  border-radius: 5px;

  display: flex;
  flex-direction: row;
  gap: 20px;
}

.rail {
  background-color: rgba(120, 120, 120, 0.7);
}

.content {
  padding: 4px;
}
</style>
