<template>
  <a-row>
    <a-button type="primary" shape="round" v-on:click="selectRepo">
      {{ repo }}
    </a-button>
  </a-row>
  <a-row type="flex">
    <a-col flex="auto">
      <a-input-search
        v-model:value="filter"
        placeholder="input search text"
        v-on:change="filterChange"
      />
    </a-col>
    <a-col flex="100px">
      <a-button type="primary">
        Unlock All
      </a-button>
    </a-col>
  </a-row>
  <a-list item-layout="horizontal" :data-source="lockEntries">
    <template v-slot:renderItem="{ item }">
      <a-list-item>
        <a-card>{{ item.file }}</a-card>
        <a-card>{{ item.user }}</a-card>
        <a-button type="primary">Unlock</a-button>
      </a-list-item>
    </template>
  </a-list>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { promisified } from "tauri/api/tauri";
// import { open } from "tauri/api-src/dialog";

// interface OpenDialogOptions {
//   filter?: string;
//   defaultPath?: string;
//   multiple?: boolean;
//   directory?: boolean;
// }

interface LockEntry {
  file: string;
  user: string;
}

// import HelloWorld from "./components/HelloWorld.vue";

export default defineComponent({
  name: "App",
  components: {},
  data() {
    return {
      repo: "Select Repo",
      filter: "",
      lockEntries: Array<LockEntry>()
    };
  },
  methods: {
    selectRepo() {
      console.log("hello");
      // const options: OpenDialogOptions = { directory: true };
      // promisified({
      //   cmd: "openDialog",
      //   options
      // }).then(response => {
      //   console.log(response);
      //   this.repo = response as string;
      // });
      // open({ directory: true }).then(response => {
      //   console.log(response);
      //   this.repo = response as string;
      // });
      promisified({
        cmd: "selectRepo"
      }).then(response => {
        console.log(response);
        this.repo = response as string;
      });
    },
    queryLocks() {
      promisified({
        cmd: "queryLocks"
      }).then(lockEntries => {
        this.lockEntries = lockEntries as Array<LockEntry>;
        console.log(this.lockEntries[0]);
      });
    },
    filterChange(e: InputEvent) {
      console.log(this.filter);
      if (this.filter.length == 0) {
        this.queryLocks();
      }
      // this.filter = newValue;
    }
  }
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
#unlock-all {
  margin-left: 10px;
}
.split {
  width: 50%;
  z-index: 1;
}

/* Control the left side */
.left {
  left: 0;
}

/* Control the right side */
.right {
  right: 0;
}
</style>
