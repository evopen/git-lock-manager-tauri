import { createApp } from "vue";
import App from "./App.vue";
import Antd from "ant-design-vue";
import "ant-design-vue/dist/antd.css";

const app = createApp(App);

app.use(Antd);
// app.component('lock-listss', {template: `<a-list item-layout="horizontal" :data-source="lockEntries">
// <template v-slot:renderItem="{ item, index }">
//   <a-list-item>
//     <a-card>{{ item.file }}</a-card>
//     <a-card>{{ item.user }}</a-card>
//     <a-button type="primary">Unlock</a-button>
//   </a-list-item>
// </template>
// </a-list>`});
app.mount("#app");
