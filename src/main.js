/**
 * Author: tears 596231290@qq.com
 * Date: 2023-02-03 21:05:36
 * LastEditors: tears 596231290@qq.com
 * LastEditTime: 2023-02-19 16:57:05
 * FilePath: /tauri-app/src/main.js
 * Description:
 *
 * Copyright (c) 2023 by tears, All Rights Reserved.
 */


import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import { createPinia } from 'pinia';
import router from "./router";
import { ElButton, ElInput } from "element-plus";
import "element-plus/dist/index.css"
import { emit, listen } from '@tauri-apps/api/event'
import 'virtual:svg-icons-register';
import Svg from "./config/svg.js";
// listen to the `click` event and get a function to remove the event listener
// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
/*******
 * @description:
 * @param {*} await
 * @return {*}
//  */
const unlisten = await listen('click', (event) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
  console.log('click', event);
})
// navigator.libs.requestDevice({acceptAllDevices: true}).then(r => console.log(r))
// emits the `click` event with the object payload
emit('click', {
  theMessage: 'Tauri is awesome!',
})
createApp(App).use(Svg).use(ElButton, ElInput).use(router).use(createPinia()).mount("#app");