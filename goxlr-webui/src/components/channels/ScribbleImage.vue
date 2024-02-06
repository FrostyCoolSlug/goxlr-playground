<script>
export default {
  name: "ScribbleImage",

  props: {
    colour: { type: String, default: "#00ffff" }
  }
}
</script>

<template>
  <div class="scribble"></div>
</template>

<style scoped>
  .scribble {
    width: 60px;
    height: 40px;
    background-color: v-bind(colour);
  }
</style>