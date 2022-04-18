import { reactive } from 'vue'

interface Updater {
  active: boolean
  version?: string
}

class Store {
  state = reactive({
    updater: {
      active: false,
    },
  })

  setUpdater(updater: Updater) {
    this.state.updater = updater
  }
}

export default new Store()
