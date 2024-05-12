import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import {
  faAngleDown,
  faClock,
  faGear,
  faHandPointer,
  faPalette,
  faVolumeXmark
} from '@fortawesome/free-solid-svg-icons'

const app = createApp(App)
library.add(faAngleDown, faHandPointer, faClock, faPalette, faGear, faVolumeXmark)

app.component('font-awesome-icon', FontAwesomeIcon)
app.use(router)
app.mount('#app')
