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
import {appWindow, WebviewWindow} from '@tauri-apps/api/window'
import ids from 'virtual:svg-icons-names'

console.log(ids);
// emit an event that are only visible to the current window
appWindow.emit('click', {message: 'Tauri is awesome111!'})

// create a new webview window and emit an event only to that window
// const webview = new WebviewWindow('window')
// webview.emit('click', {message: '123456'})
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
let webview;
const open333 = () => {
  webview = new WebviewWindow('11111', {
        label: '1111',            // 窗口唯一label
        // title: '',              // 窗口标题
        url: 'tauri://localhost/about',                // 路由地址url
        // width: 900,             // 窗口宽度
        // height: 640,            // 窗口高度
        // minWidth: null,         // 窗口最小宽度
        // minHeight: null,        // 窗口最小高度
        // x: 0,                // 窗口相对于屏幕左侧坐标
        // y: 0,                // 窗口相对于屏幕顶端坐标
        // center: false,           // 窗口居中显示
        // resizable: true,        // 是否支持缩放
        // maximized: true,       // 最大化窗口
        // decorations: false,     // 窗口是否无边框及导航条
        // alwaysOnTop: false,     // 置顶窗口
      }
  )
  console.log(webview);
  webview.once('tauri://created', function (e) {
    console.log(e);
    // webview window successfully created
  })
  webview.once('tauri://error', function (e) {
    console.log(e);
    // an error happened creating the webview window
  })
}
const open444 = () => {
  const testWindow = WebviewWindow.getByLabel("1111");//这里就是获取label
  testWindow.close();
}
// invoke("init_process");
const greet = async () => {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", {name: name.value});
}
</script>

<template>
  <div class="card">
    <el-input id="greet-input" v-model="name" placeholder="Enter a name..."></el-input>
    <el-button type="primary" @click="greet()">Greet</el-button>
  </div>
  <svg-icon icon-class="404"/>
  <p>{{ greetMsg }}</p>
  <el-button type="primary" @click="open111">Primary</el-button>
  <el-button type="primary" @click="open222">Dialog</el-button>
  <el-button type="primary" @click="open333">webView</el-button>
  <el-button type="primary" @click="open444">webView</el-button>
</template>
