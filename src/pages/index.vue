<script setup lang="ts">
import {computed, reactive, ref} from 'vue';
import Trash from '../components/trash.vue';
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";

type TeamMember = {
  id: number;
  point: [number,number];
}

enum RootSelectType {
  LT,
  RB
}

const teamMember = ref<TeamMember[]>([]);
const rootPoint = reactive({
  x1: 0,
  y1: 0,
  x2: 0,
  y2: 0
})

const point1Text = computed(() => rootPoint.x1 > 0 && rootPoint.y1 > 0 ? `(${rootPoint.x1}, ${rootPoint.y1})` : '点击选择' )
const point2Text = computed(() => rootPoint.x2 > 0 && rootPoint.y2 > 0 ? `(${rootPoint.x2}, ${rootPoint.y2})` : '点击选择' )

const addTeamMember = () => {
  const last = teamMember.value[teamMember.value.length - 1];
  let id = last ? last.id + 1 : 1;
  teamMember.value.push({
    id,
    point: [0,0]
  })
}

const getMemberText = (id: number) => {
  const member = teamMember.value.filter(member => member.id == id)[0];
  if (!member){
    return;
  }
  const [x,y] = member.point;
  return x > 0 && y > 0 ? `(${x}, ${y})` : '请选择'
}

const removeTeamMember = (id: number) => {
  teamMember.value = teamMember.value.filter((member) => member.id !== id);
}
const onClickTeamMemberSelect = async (id: number) => {
  await invoke('open_select_window', {index: id + 1});
}

const rootSelectClick = async (type: RootSelectType) => {
  const index = type === RootSelectType.LT ? 0 : 1;
  await invoke('open_select_window', {index});
}

listen('set_detect_area1', (event) => {
  let payload = event.payload as number[];
  rootPoint.x1 = payload[0];
  rootPoint.y1 = payload[1];
})

listen('set_detect_area2', (event) => {
  let payload = event.payload as number[];
  rootPoint.x2 = payload[0];
  rootPoint.y2 = payload[1];
})

listen('set_click_position', (event) => {
  const positions = event.payload as [number, number][];
  positions.forEach(([x, y], index) => {
    const memberId = index + 1;
    const memberIndex = teamMember.value.findIndex(m => m.id === memberId);
    if (memberIndex !== -1) {
      teamMember.value[memberIndex].point = [x, y];
    }
  });
})

</script>

<template>
  <div class="w-full h-full bg-gradient-to-br from-slate-900 to-slate-800 font-sans text-base leading-normal">
  <div class="h-full max-w-400px overflow-auto mx-auto py-32px box-border px-4">
    <div class="w-full">
      <div class="w-full h-fit box-border p-6 bg-slate-100/10 rounded-3xl shadow-lg backdrop-blur-sm border border-slate-700/30">
        <h1 class="text-slate-200 text-base leading-none font-sans">队长设置</h1>
        <div class="w-full h-1px bg-zinc-500"></div>
        <p class="text-slate-200 font-sans">请检测队长的设置并同步</p>
        <div class="flex flex-col gap-4">
        <div class="w-full grid grid-cols-2 justify-between items-center">
            <span class="text-slate-200 font-sans">监控区域左上角</span>
            <button @click="()=>rootSelectClick(RootSelectType.LT)" class="min-h-48px hover:text-blue-400 hover:border-blue-400 transition-all duration-200 cursor-pointer rounded-xl text-slate-200 text-xl bg-slate-800/50 border border-2px border-solid border-slate-600 hover:bg-slate-700/50 active:scale-95 font-sans">
              {{ point1Text }}
            </button>
          </div>
          <div class="w-full grid grid-cols-2 justify-between items-center">
            <span class="text-slate-200 font-sans">监控区域右下角</span>
            <button @click="()=>rootSelectClick(RootSelectType.RB)" class="min-h-48px hover:text-blue-400 hover:border-blue-400 transition-all duration-200 cursor-pointer rounded-xl text-slate-200 text-xl bg-slate-800/50 border border-2px border-solid border-slate-600 hover:bg-slate-700/50 active:scale-95 font-sans">
              {{ point2Text }}
            </button>
          </div>
        </div>
      </div>
    </div>
    <div class="w-full space-y-4 mt-40px">
      <div class="w-full p-4 bg-slate-100/10 rounded-3xl box-border flex items-center justify-around gap-4 shadow-md backdrop-blur-sm border border-slate-700/30 transition-all hover:bg-slate-700/20" v-for="member of teamMember" :key="member.id">
        <div>
          <span class="text-zinc-200 font-sans">队员 {{ member.id }}</span>
        </div>
        <div class="grow flex">
        <button @click="()=>onClickTeamMemberSelect(member.id)" class="h-48px grow hover:text-blue-400 hover:border-blue-400 transition-all duration-200 cursor-pointer rounded-xl text-slate-200 text-xl bg-slate-800/50 border border-2px border-solid border-slate-600 hover:bg-slate-700/50 active:scale-95 font-sans">
          {{ getMemberText(member.id) }}
        </button>
        <button @click="() => removeTeamMember(member.id)" class="h-48px w-48px aspect-square bg-transparent border-none text-slate-200 font-sans">
          <Trash class="size-full hover:bg-red-500/20 cursor-pointer rounded" />
        </button>
      </div>
      </div>
    </div>
    <div class="w-full h-fit mt-4">
      <button class="w-full h-48px rounded-xl bg-gradient-to-r from-blue-600 to-blue-500 text-zinc-200 border-none cursor-pointer shadow-md hover:shadow-blue-500/30 transition-all hover:scale-[1.02] active:scale-95 font-sans text-xl" @click="addTeamMember">
        添加队员
      </button>
    </div>
  </div></div>
</template>