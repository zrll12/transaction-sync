<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { ref } from "vue";

const imageData = ref("");

listen("update-preview", (event) => {
  imageData.value = `data:image/png;base64,${event.payload as string}`;
  console.log(imageData.value);
});
</script>

<template>
  <div class="w-full h-full bg-slate-900 flex items-center justify-center">
    <img v-if="imageData" :src="imageData" class="max-w-full max-h-full object-contain" />
  </div>
</template>