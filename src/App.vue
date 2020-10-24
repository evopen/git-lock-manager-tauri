<template>
  <a-row>
    <input
      type="file"
      ref="file"
      style="display: none"
      v-on:change="selectRepo"
    />
    <a-button type="primary" shape="round" @click="$refs.file.click()">
      {{ repo }}
    </a-button>
  </a-row>
  <a-row type="flex">
    <a-col flex="auto">
      <a-input-search
        v-model:value="value"
        placeholder="input search text"
        @search="onSearch"
      />
    </a-col>
    <a-col flex="100px">
      <a-button type="primary">
        Unlock All
      </a-button>
    </a-col>
  </a-row>
  <a-list item-layout="horizontal"> </a-list>
</template>

<script lang="ts">
import { defineComponent } from "vue";

// import HelloWorld from "./components/HelloWorld.vue";

export default defineComponent({
  name: "App",
  components: {},
  data() {
    return {
      repo: "Select Repo",
      filter: ""
    };
  },
  methods: {
    selectRepo(file: File) {
      console.log(file.name);
      this.repo = file.name;
      // alert(this.repo);
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
