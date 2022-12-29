import { createApp } from 'vue'
import App from './About.vue'
import Antd from 'ant-design-vue'
import 'ant-design-vue/dist/antd.dark.less'
import '../../style.css'

createApp(App).use(Antd).mount('#app')
