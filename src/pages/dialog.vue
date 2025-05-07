<script setup lang="ts">

import {invoke} from "@tauri-apps/api/core";

const selectWindowClose = () => {
  invoke("close_select_window")
}

</script>

<template>
  <div class="w-fit px-4 py-2 bg-zinc-200 rounded-lg space-y-2 bg-opacity-25 cursor-pointer" data-tauri-drag-region>
    <div class="size-64px flex items-center justify-center" data-tauri-drag-region>
      <img src="/img.png" alt="" @dragstart.prevent data-tauri-drag-region>
    </div>
    <button class="cursor-pointer w-full bg-blue-500 text-zinc-200 px-3 py-1 rounded-md border-none" @click="selectWindowClose">
      确认
    </button>
  </div>
</template>