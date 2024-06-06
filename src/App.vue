<template>
  <div class="flex h-screen">
    <div class="w-1/4 border-r overflow-y-auto" @scroll="handleScroll">
      <button @click="selectFolder" class="w-full p-2 bg-blue-500 text-white">Open DB</button>
      <input type="text" v-model="searchQuery" @input="fetchKeys(true)" placeholder="Search" class="w-full p-2 border-b"/>
      <ul>
        <li
          v-for="key in keys"
          :key="key"
          @click="selectKey(key)"
          class="p-2 cursor-pointer hover:bg-gray-200"
        >
          {{ key }}
        </li>
      </ul>
    </div>
    <div class="w-3/4 p-4">
      <div v-if="selectedKey && value !== null">
        <h2 class="text-2xl mb-4">{{ selectedKey }}</h2>
        <pre class="bg-gray-100 p-4">{{ value }}</pre>
      </div>
    </div>
  </div>
</template>

<script>
import { open } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';

export default {
  data() {
    return {
      dbPath: null,
      keys: [],
      searchQuery: '',
      selectedKey: null,
      value: null,
      limit: 20,
      start: 0,
      loading: false,
      hasMore: true,
    };
  },
  methods: {
    async selectFolder() {
      const selected = await open({
        directory: true,
        multiple: false,
      });

      if (selected) {
        this.dbPath = selected;
        await invoke('open_db', {path: selected});
        this.start = 0; // Reset start index
        this.keys = []; // Reset keys
        this.hasMore = true; // Reset hasMore flag
        this.fetchKeys(); // Загрузка ключей после открытия базы данных
      }
    },
    async fetchKeys(reset = false) {
      if (this.dbPath && !this.loading && this.hasMore) {
        this.loading = true;
        if (reset) {
          this.start = 0;
          this.keys = [];
          this.hasMore = true;
        }
        const keys = await invoke('get_keys', {
          start: this.start,
          limit: this.limit,
          query: this.searchQuery || null
        });
        console.log('Fetched keys:', keys, Array.isArray(keys)); // Отладочный вывод ключей
        if (keys.length < this.limit) {
          this.hasMore = false;
        }
        this.keys = [...this.keys, ...keys];
        this.start += this.limit;
        this.loading = false;
      }
    },
    async selectKey(key) {
      this.selectedKey = key;
      if (this.dbPath) {
        this.value = await invoke('get_value', {path: this.dbPath, key});
      }
    },
    handleScroll(event) {
      const bottom = event.target.scrollHeight - event.target.scrollTop === event.target.clientHeight;
      if (bottom && this.hasMore) {
        this.fetchKeys();
      }
    }
  }
};
</script>

<style>
@import 'tailwindcss/tailwind.css';
</style>
