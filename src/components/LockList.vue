<template>
  <a-list item-layout="horizontal" :data-source="$parent.lockEntries">
    <template v-slot:renderItem="{ item }">
      <a-list-item>
        <a-card>{{ item.file }}</a-card>
        <a-card>{{ item.user }}</a-card>
        <a-button type="primary">Unlock</a-button>
      </a-list-item>
    </template>
  </a-list>
</template>

<script>
export default {
  name: 'LockList',
}
</script>
