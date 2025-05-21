<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { ref } from "vue";

const imageData1 = ref("");
const imageCount1 = ref(0);
const imageData2 = ref("");
const imageCount2 = ref(0);
const state = ref("");
const last_key = ref("none");

interface PreviewEvent {
  image: string;
  count: number;
}

listen("update-preview-1", (event: { payload: PreviewEvent }) => {
  imageData1.value = `data:image/png;base64,${event.payload.image}`;
  imageCount1.value = event.payload.count;
});

listen("update-preview-2", (event: { payload: PreviewEvent }) => {
  imageData2.value = `data:image/png;base64,${event.payload.image}`;
  imageCount2.value = event.payload.count;
});

listen("detection_state", (event) => {
  state.value = event.payload as string;
})

listen("key_pressed", (event) => {
  let timestamp = new Date();
  last_key.value = `${timestamp}-${event.payload}` as string;
});
</script>

<template>
  <div class="w-full h-full bg-slate-900 flex items-center justify-center p-4">
    <div class="flex gap-4 w-full h-full">
      <div class="text-white mt-2" v-text="state" />
      <div class="text-white mt-2" v-text="last_key" />
      <div class="flex-1 min-w-0 flex items-center justify-center bg-slate-800/50 rounded-lg overflow-hidden">
        <div class="flex flex-col items-center">
          <img v-if="imageData1" :src="imageData1" class="max-w-full max-h-full object-contain" alt="preview 1" />
          <div class="text-white mt-2" v-text="imageCount1" />
        </div>
      </div>
      <div class="flex-1 min-w-0 flex items-center justify-center bg-slate-800/50 rounded-lg overflow-hidden">
        <div class="flex flex-col items-center">
          <img v-if="imageData2" :src="imageData2" class="max-w-full max-h-full object-contain" alt="preview 2" />
          <div class="text-white mt-2" v-text="imageCount2" />
        </div>
      </div>
    </div>
  </div>
</template>