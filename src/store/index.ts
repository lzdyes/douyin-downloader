import { reactive } from 'vue'

interface Updater {
  active: boolean
  version?: string
}

class Store {
  state = reactive({
    activeMenu: 0,
    updater: {
      active: false,
    },
  })

  setActiveMenu(index: number) {
    this.state.activeMenu = index
  }

  setUpdater(updater: Updater) {
    this.state.updater = updater
  }
}

export default new Store()
