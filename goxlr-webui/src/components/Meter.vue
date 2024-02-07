<script>
import { store } from '@/goxlr/store.js'
import { websocket } from '@/goxlr/sockets.js'

export default {
  name: 'Meter',
  data() {
    return {
      active: false,

      canvas: undefined,
      canvas_size: {
        width: 0,
        height: 0
      },

      minimum_value: -60,

      points: [],
      point_count: 30,

      poll_rate: 100,
      last_paint: 0
    }
  },

  methods: {
    tickChange: function (e) {
      this.active = e.target.checked
      if (this.active) {
        this.pollData()
      }
    },

    pollData: function () {
      let self = this
      let command = { Microphone: 'GetMicLevel' }

      websocket.send_command(store.getActiveSerial(), command).then((data) => {
        let value = data.DeviceCommand.MicLevel
        if (value < this.minimum_value) {
          value = this.minimum_value
        }

        let minimum_value = this.minimum_value * -1
        let y = (value / minimum_value) * -1 * self.canvas_size.height

        self.points.push({
          x: self.canvas_size.width,
          y: y
        })

        let point_count = self.canvas_size.width / self.rate
        while (self.points.length > this.point_count) {
          self.points.shift()
        }

        if (self.active) {
          setTimeout(this.pollData, this.poll_rate)
        } else {
          this.points = []
        }
      })
    },

    draw: function (timestamp) {
      // Work out if we should draw...
      if (!this.active || this.points.length === 0) {
        requestAnimationFrame(this.draw)
        return
      }

      // Create a delta since last frame..
      let delta = timestamp - this.last_paint

      // Shift the Canvas...
      this.move_canvas(delta)

      this.draw_peaking()
      this.draw_good()

      this.draw_db_lines()

      // Draw the Points..
      this.canvas.strokeStyle = 'white'
      // for (let index in this.points) {
      //   let point = this.points[index]
      //   this.canvas.fillRect(point.x, point.y, 2, 2)
      // }

      // Begin the Path..
      let f = 0.3
      let t = 0.6
      let dx1 = 0,
        dy1 = 0

      this.canvas.beginPath()
      this.canvas.moveTo(this.points[0].x, this.points[0].y)

      let previous_point = this.points[0]
      for (let i = 0; i < this.points.length; i++) {
        let dx2 = 0,
          dy2 = 0

        let current_point = this.points[i]
        let next_point = this.points[i + 1]
        if (next_point !== undefined) {
          let m = this.gradient(previous_point, next_point)
          dx2 = (next_point.x - current_point.x) * -f
          dy2 = dx2 * m * t
        }

        this.canvas.bezierCurveTo(
          previous_point.x - dx1,
          previous_point.y - dy1,
          current_point.x + dx2,
          current_point.y + dy2,
          current_point.x,
          current_point.y
        )
        dx1 = dx2
        dy1 = dy2
        previous_point = current_point
      }
      this.canvas.stroke()

      // Update the last paint time..
      this.last_paint = timestamp

      // Call back on the next animation frame
      requestAnimationFrame(this.draw)
    },

    draw_peaking: function () {
      this.canvas.fillStyle = 'rgba(255, 0, 0, 0.2)'
      let top = this.get_db_position(-0)
      let bottom = this.get_db_position(-10) - top
      //console.log(`${top} - ${bottom}`)

      // Find out where -10db is..
      this.canvas.fillRect(0, top, this.canvas_size.width, bottom)
    },

    draw_good: function () {
      this.canvas.fillStyle = 'rgba(0, 255, 0, 0.2)'

      let top = this.get_db_position(-10)
      let bottom = this.get_db_position(-20) - top
      this.canvas.fillRect(0, top, this.canvas_size.width, bottom)
    },

    draw_db_lines: function () {
      for (let i = 0; i < 7; i++) {
        let value = i * -10
        let position = this.get_db_position(value)

        this.canvas.strokeStyle = 'rgb(50, 50, 50)'
        this.canvas.beginPath()
        this.canvas.moveTo(0, position)
        this.canvas.lineTo(this.canvas_size.width, position)
        this.canvas.stroke()
      }
    },

    get_db_position: function (dB) {
      let minimum_value = this.minimum_value * -1
      return (dB / minimum_value) * -1 * this.canvas_size.height
    },

    get_alignment_position: function (ref, dB) {
      let top = this.get_db_position(dB)
      let reference = this.$refs[ref]
      if (reference === undefined) {
        // Probably not rendered yet..
        return 0
      }

      return top - reference.clientHeight / 2
    },

    get_label_position: function (ref, min, max) {
      let top = this.get_db_position(max)
      let bottom = this.get_db_position(min)

      console.log(`${top} - ${bottom}`)

      let middle = (bottom - top) / 2

      let reference = this.$refs[ref]
      if (reference === undefined) {
        // Probably Not rendered..
        return 0
      }

      console.log(middle - reference / 2)
      return top + (middle - reference.clientHeight / 2)
    },

    // A simple method which will move the canvas X pixels, dictated by 'speed'
    move_canvas: function (delta) {
      let w = this.canvas_size.width
      let h = this.canvas_size.height

      // Clear the Canvas..
      this.canvas.clearRect(0, 0, w, h)

      // How long it should take a point to move across the screen..
      // 100ms * 30 points = 3 seconds
      let time_to_cross = this.poll_rate * this.point_count

      // How much time has passed based on the delta?
      let frame_movement = (delta / time_to_cross) * w

      this.points.forEach((value) => {
        value.x -= frame_movement
      })
    },

    gradient: function (a, b) {
      return (b.y - a.y) / (b.x - a.x)
    }
  },

  mounted() {
    let canvas = this.$refs['canvas']
    this.canvas_size = {
      width: canvas.width,
      height: canvas.height
    }

    this.canvas = canvas.getContext('2d', { willReadFrequently: true })

    // Ok, configure the canvas..
    this.canvas.strokeStyle = '#00ff00'
    this.canvas.fillStyle = '#00ff00'
    this.canvas.lineWidth = 2

    this.draw()
  },

  computed: {}
}
</script>

<template>
  <input type="checkbox" @change="tickChange" />
  <div style="display: flex; flex-direction: column">
    <div style="position: relative; display: flex">
      <div style="height: 300px; display: inline-block; color: #fff">
        <div
          ref="10dBText"
          v-bind:style="{ top: get_alignment_position('10dBText', -0) + 'px' }"
          class="decibels"
        >
          0dB
        </div>
        <div
          ref="10dBText"
          v-bind:style="{ top: get_alignment_position('10dBText', -10) + 'px' }"
          class="decibels"
        >
          10dB
        </div>
        <div
          ref="20dBText"
          v-bind:style="{ top: get_alignment_position('20dBText', -20) + 'px' }"
          class="decibels"
        >
          20dB
        </div>
        <div
          ref="30dBText"
          v-bind:style="{ top: get_alignment_position('30dBText', -30) + 'px' }"
          class="decibels"
        >
          30dB
        </div>
        <div
          ref="40dBText"
          v-bind:style="{ top: get_alignment_position('40dBText', -40) + 'px' }"
          class="decibels"
        >
          40dB
        </div>
        <div
          ref="50dBText"
          v-bind:style="{ top: get_alignment_position('50dBText', -50) + 'px' }"
          class="decibels"
        >
          50dB
        </div>
        <div
          ref="60dBText"
          v-bind:style="{ top: get_alignment_position('60dBText', -60) + 'px' }"
          class="decibels"
        >
          60dB
        </div>

        <!-- Lets Fuck Around -->
        <div
          ref="peak"
          v-bind:style="{ top: get_label_position('peak', -10, -0) + 'px' }"
          class="label"
        >
          Peak Area
        </div>
        <div
          ref="speaking"
          v-bind:style="{ top: get_label_position('speaking', -20, -10) + 'px' }"
          class="label"
        >
          Speaking Area
        </div>
      </div>
      <div style="display: flex; margin-left: 60px; width: 640px; overflow: hidden">
        <canvas
          ref="canvas"
          id="canvas"
          width="700"
          height="300"
          style="background-color: rgba(0, 0, 0, 0.2)"
        ></canvas>
      </div>
    </div>
  </div>
</template>

<style scoped>
.decibels {
  text-align: right;
  position: absolute;
  font-size: 12px;
  width: 50px;
}

.label {
  position: absolute;
  left: 70px;
  font-size: 12px;
}
</style>
