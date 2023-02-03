/*
 * @Author: tears 596231290@qq.com
 * @Date: 2023-01-31 20:14:11
 * @LastEditors: tears 596231290@qq.com
 * @LastEditTime: 2023-02-01 14:53:16
 * @FilePath: /tauri-app/src/router/index.js
 * @版权声明 保留文件所有权利: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
import { createRouter, createWebHistory } from "vue-router";
import nprogress from "nprogress";
import 'nprogress/nprogress.css'; // progress bar style
// 也可以从其他文件导入
const Home = { template: '<div>Home</div>' }
const About = { template: '<div>About</div>' }
import Greet from "@/components/Greet.vue"
nprogress.configure({ showSpinner: false }); // NProgress Configuration
// 2. 定义一些路由
// 每个路由都需要映射到一个组件。
// 我们后面再讨论嵌套路由。
const routes = [
  { path: '/', component: Greet },
  { path: '/about', component: About },
]

// 3. 创建路由实例并传递 `routes` 配置
// 你可以在这里输入更多的配置，但我们在这里
// 暂时保持简单
const router = createRouter({
  // 4. 内部提供了 history 模式的实现。为了简单起见，我们在这里使用 hash 模式。
  history: createWebHistory(),
  routes, // `routes: routes` 的缩写
})
router.beforeEach((to, from, next) => {
  nprogress.start();
  next();
})
router.afterEach(()=>{
  nprogress.done();
})
export default router;