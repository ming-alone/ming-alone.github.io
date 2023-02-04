<script setup>
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {useCounterStore} from "@/store";
import {ElMessage, ElMessageBox} from "element-plus";

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

  <p>{{ greetMsg }}</p>
  <el-button type="primary" @click="open111">Primary</el-button>
  <el-button type="primary" @click="open222">Dialog</el-button>
</template>
