/*
 * @Author: tears 596231290@qq.com
 * @Date: 2023-01-31 10:41:51
 * @LastEditors: tears 596231290@qq.com
 * @LastEditTime: 2023-02-04 14:44:38
 * @FilePath: /tauri-app/src/main.js
 * @版权声明 保留文件所有权利: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import { createPinia } from 'pinia';
import router from "./router";
import { ElButton } from "element-plus";
import "element-plus/dist/index.css"
import { emit, listen } from '@tauri-apps/api/event'

// listen to the `click` event and get a function to remove the event listener
// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
const unlisten = await listen('click', (event) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
  console.log('click', event);
})

// emits the `click` event with the object payload
emit('click', {
  theMessage: 'Tauri is awesome!',
})
createApp(App).use(ElButton).use(router).use(createPinia()).mount("#app");