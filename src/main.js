/*
 * @Author: tears 596231290@qq.com
 * @Date: 2023-01-31 10:41:51
 * @LastEditors: tears 596231290@qq.com
 * @LastEditTime: 2023-02-01 01:45:52
 * @FilePath: /tauri-app/src/main.js
 * @版权声明 保留文件所有权利: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import { storeToRefs, createPinia } from 'pinia';
import router from "./router";
import { ElButton } from "element-plus";
import "element-plus/dist/index.css"
createApp(App).use(ElButton).use(router).use(createPinia()).mount("#app");