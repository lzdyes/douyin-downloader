import { reactive } from 'vue'

class Store {
  state = reactive({
    activeMenu: 0,
  })

  setActiveMenu(index: number) {
    this.state.activeMenu = index
  }
}

export default new Store()
