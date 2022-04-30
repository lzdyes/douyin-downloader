import { createI18n } from 'vue-i18n'

import cn from './locales/cn.yml'
import en from './locales/en.yml'

const i18n = createI18n({
  legacy: false,
  locale: localStorage.getItem('language') || 'cn',
  fallbackLocale: 'cn',
  messages: { cn, en },
})

export default i18n
