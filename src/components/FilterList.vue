<template>
  <a-list item-layout="horizontal" :data-source="$parent.fileList">
    <template v-slot:renderItem="{ item }">
      <a-list-item>
        <a-card>{{ item }}</a-card>
        <a-button type="primary">Unlock</a-button>
      </a-list-item>
    </template>
  </a-list>
</template>

<script>
export default {
  name: 'FilterList',
  props: {
    msg: String
  }
}
</script>
