<script setup>
import { store } from '@/goxlr/store.js'
import PushButton from '@/components/Button.vue'

function hasConnected() {
  return store.hasConnected()
}

function isConnected() {
  return store.isConnected()
}

function hasConfig() {
  return store.getConfig() !== undefined
}

function getMixers() {
  return store.status.devices
}

function setDevice(serial) {
  store.setActiveSerial(serial)
}

function deviceCount() {
  return store.getDeviceCount()
}

function getLabel2(device) {
  return '[' + device.serial + '] GoXLR ' + ' connected to USB bus ' + ' address '
}
</script>

<template>
  <div class="wrapper">
    <div class="buttonList">
      <div>
        <div class="label">Select Device</div>

        <!-- If we've never connected before, and we're not connected now.. -->
        <div v-if="!hasConnected() && !isConnected()">
          <div class="no-device">Attempting to Connect to the GoXLR Utility..</div>
        </div>

        <!-- We *HAVE* connected before, but we're not connected now.. -->
        <div v-else-if="hasConnected() && !isConnected()">
          <div class="no-device">
            Unable to connect to the GoXLR Utility, please check it's running.<br /><br />
            This page will automatically try to reconnect..
          </div>
        </div>

        <!-- We should be connected here! -->
        <div v-else>
          <div class="buttonHolder" v-if="deviceCount() > 0">
            <PushButton
              v-for="(device, key) in getMixers()"
              :key="key"
              :button-id="key"
              :is-active="false"
              :label="getLabel2(device)"
              @button-pressed="setDevice(key)"
            />
          </div>
          <div v-else class="no-device">No GoXLR Devices Found</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.wrapper {
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
}

.buttonList {
  height: 220px;
  width: 700px;
  margin: 3px;
  background-color: #353937;
}

.buttonList:not(:last-child) {
  margin-right: 20px;
}

.buttonHolder {
  height: 170px;
  width: 700px;

  box-sizing: border-box;

  overflow-y: auto;
}

.buttonHolder::-webkit-scrollbar {
  width: 3px;
}

.buttonHolder::-webkit-scrollbar-track {
  background-color: transparent;
}

.buttonHolder::-webkit-scrollbar-thumb {
  background-color: #dfdfdf;
  border-radius: 3px;
}

.label {
  width: 680px;
  padding: 10px;
  color: #fff;
  background-color: #3b413f;

  text-transform: uppercase;

  margin-bottom: 8px;
}

.no-device {
  color: #fff;
}
</style>
