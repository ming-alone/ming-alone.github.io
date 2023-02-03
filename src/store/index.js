/*
 * @Author: tears 596231290@qq.com
 * @Date: 2023-01-31 17:42:20
 * @LastEditors: tears 596231290@qq.com
 * @LastEditTime: 2023-01-31 17:42:23
 * @FilePath: /tauri-app/src/store/index.js
 * @版权声明 保留文件所有权利: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
import { defineStore } from "pinia";
export const useCounterStore = defineStore('counter', {
  state: () => ({ count: 0 }),
  getters: {
    double: (state) => state.count * 2,
  },
  actions: {
    increment() {
      this.count++
    },
  },
})