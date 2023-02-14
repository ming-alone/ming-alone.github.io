<!--
 * @Author: tears 596231290@qq.com
 * @Date: 2023-02-03 21:05:36
 * @LastEditors: tears 596231290@qq.com
 * @LastEditTime: 2023-02-04 14:45:16
 * @FilePath: /tauri-app/src/components/Greet.vue
 * @版权声明 保留文件所有权利: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
-->
<script setup>
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {useCounterStore} from "@/store";
import {ElMessage, ElMessageBox} from "element-plus";
import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
import ids from 'virtual:svg-icons-names'

console.log(ids);
// emit an event that are only visible to the current window
appWindow.emit('click', { message: 'Tauri is awesome111!' })

// create a new webview window and emit an event only to that window
const webview = new WebviewWindow('window')
webview.emit('click', { message: '123456'})
const store = useCounterStore()
const open111 = () => {
  store.increment();
  ElMessage({
    message: `hello world! ${store.count}`,
    grouping: true,
    type: "success"
  });
}
const open222 = () => {
  ElMessageBox.alert('This is a message', 'Title', {
    // if you want to disable its autofocus
    // autofocus: false,
    confirmButtonText: 'OK',
    callback: (action) => {
      ElMessage({
        type: 'info',
        message: `action: ${action}`
      })
    }
  })
}
const greetMsg = ref("");
const name = ref("");
invoke("init_process");
const greet = async () => {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", {name: name.value});
}
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..."/>
    <button type="button" @click="greet()">Greet</button>
  </div>
  <svg-icon icon-class="404"/>
  <p>{{ greetMsg }}</p>
  <el-button type="primary" @click="open111">Primary</el-button>
  <el-button type="primary" @click="open222">Dialog</el-button>
</template>
