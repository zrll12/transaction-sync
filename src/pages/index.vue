<script setup lang="ts">
import {computed, reactive, ref} from 'vue';
import Trash from '../components/trash.vue';
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";

// 类型定义
enum Type {
  IDLE,
  PENDING,
}

type TeamMember = {
  id: number;
  point: [
    [number, number],
    [number, number]
  ];
  key: string | null;
}

enum RootSelectType {
  LT1,
  RB1,
  LT2,
  RB2
}

// 状态管理
const bindType = ref(Type.IDLE);
let curKey: string | null = null;
let curBind = -1;
const teamMember = ref<TeamMember[]>([]);

// 区域坐标状态
const rootPoint = reactive({
  area1: { x1: 0, y1: 0, x2: 0, y2: 0 },
  area2: { x1: 0, y1: 0, x2: 0, y2: 0 }
})

// 计算属性
const point1Text = computed(() => formatPointText(rootPoint.area1.x1, rootPoint.area1.y1));
const point2Text = computed(() => formatPointText(rootPoint.area1.x2, rootPoint.area1.y2));
const point3Text = computed(() => formatPointText(rootPoint.area2.x1, rootPoint.area2.y1));
const point4Text = computed(() => formatPointText(rootPoint.area2.x2, rootPoint.area2.y2));

// 工具函数
function formatPointText(x: number, y: number): string {
  return x > 0 && y > 0 ? `(${x}, ${y})` : '点击选择';
}

// 队员管理
const addTeamMember = () => {
  const last = teamMember.value[teamMember.value.length - 1];
  const id = last ? last.id + 1 : 1;
  teamMember.value.push({
    id,
    point: [
      [0,0],
      [0,0]
    ],
    key: null
  });
}

const getMemberText = (id: number, type: 'TL' | 'BR') => {
  const member = teamMember.value.filter(member => member.id == id)[0];
  if (!member){
    return;
  }
  const [tl,br] = member.point;
  if (type ==='TL') {
    const [x,y] = tl;
    return x > 0 && y > 0 ? `(${x}, ${y})` : '请选择'
  }
  const [x,y] = br;
  return x > 0 && y > 0 ? `(${x}, ${y})` : '请选择'
}

const removeTeamMember = (id: number) => {
  teamMember.value = teamMember.value.filter((member) => member.id !== id);
  invoke("delete_click_position", {index: id});
}
const onClickTeamMemberSelect = async (id: number) => {
  await invoke('open_select_window', {index: id, labelType: "left"});
}

const rootSelectClick = async (type: RootSelectType) => {
  let index;
  switch(type) {
    case RootSelectType.LT1:
      index = 0;
      break;
    case RootSelectType.RB1:
      index = 1;
      break;
    case RootSelectType.LT2:
      index = 2;
      break;
    case RootSelectType.RB2:
      index = 3;
      break;
  }
  await invoke('open_select_window', {index, labelType: "caption"});
}

const testClick = async () => {
  await invoke('move_mouse');
}
const onClickBind = (id: number) => {
  curKey = null;
  bindType.value = Type.PENDING;
  curBind = id;
  let abort = new AbortController();
  window.addEventListener('keydown', (ev) => {
    bindKey(id,ev.key);
    abort.abort();
  }, {signal: abort.signal})
}

const bindKey = (id: number, char: string) => {
  if(bindType.value === Type.IDLE) {
    return;
  }
  teamMember.value = teamMember.value.map((member) => {
    if (member.id === id) {
      return {
        ...member,
        key: char
      }
    }
    return member;
  })
  curKey = null;
  curBind = -1;
  bindType.value = Type.IDLE;
}

listen('set_detect_area1', (event) => {
  let payload = event.payload as number[];
  rootPoint.area1.x1 = payload[0];
  rootPoint.area1.y1 = payload[1];
})

listen('set_detect_area2', (event) => {
  let payload = event.payload as number[];
  rootPoint.area1.x2 = payload[0];
  rootPoint.area1.y2 = payload[1];
})

listen('set_detect_area3', (event) => {
  let payload = event.payload as number[];
  rootPoint.area2.x1 = payload[0];
  rootPoint.area2.y1 = payload[1];
})

listen('set_detect_area4', (event) => {
  let payload = event.payload as number[];
  rootPoint.area2.x2 = payload[0];
  rootPoint.area2.y2 = payload[1];
})

listen('set_left_click_position', (event) => {
  const positions = event.payload as [number, number][];
  positions.forEach(([x, y], index) => {
    const memberId = index;
    const memberIndex = teamMember.value.findIndex(m => m.id === memberId);
    if (memberIndex !== -1) {
      teamMember.value[memberIndex].point = [
        [x,y],
        [0,0]
      ];
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
          <div class="space-y-2">
            <h2 class="text-slate-200 font-sans text-lg">监控区域1</h2>
            <div class="w-full grid grid-cols-2 justify-between items-center">
              <span class="text-slate-200 font-sans">左上角</span>
              <button @click="()=>rootSelectClick(RootSelectType.LT1)" class="min-h-48px hover:text-blue-400 hover:border-blue-400 transition-all duration-200 cursor-pointer rounded-xl text-slate-200 text-xl bg-slate-800/50 border border-2px border-solid border-slate-600 hover:bg-slate-700/50 active:scale-95 font-sans">
                {{ point1Text }}
              </button>
            </div>
            <div class="w-full grid grid-cols-2 justify-between items-center">
              <span class="text-slate-200 font-sans">右下角</span>
              <button @click="()=>rootSelectClick(RootSelectType.RB1)" class="min-h-48px hover:text-blue-400 hover:border-blue-400 transition-all duration-200 cursor-pointer rounded-xl text-slate-200 text-xl bg-slate-800/50 border border-2px border-solid border-slate-600 hover:bg-slate-700/50 active:scale-95 font-sans">
                {{ point2Text }}
              </button>
            </div>
          </div>
          <div class="space-y-2">
            <h2 class="text-slate-200 font-sans text-lg">监控区域2</h2>
            <div class="w-full grid grid-cols-2 justify-between items-center">
              <span class="text-slate-200 font-sans">左上角</span>
              <button @click="()=>rootSelectClick(RootSelectType.LT2)" class="min-h-48px hover:text-blue-400 hover:border-blue-400 transition-all duration-200 cursor-pointer rounded-xl text-slate-200 text-xl bg-slate-800/50 border border-2px border-solid border-slate-600 hover:bg-slate-700/50 active:scale-95 font-sans">
                {{ point3Text }}
              </button>
            </div>
            <div class="w-full grid grid-cols-2 justify-between items-center">
              <span class="text-slate-200 font-sans">右下角</span>
              <button @click="()=>rootSelectClick(RootSelectType.RB2)" class="min-h-48px hover:text-blue-400 hover:border-blue-400 transition-all duration-200 cursor-pointer rounded-xl text-slate-200 text-xl bg-slate-800/50 border border-2px border-solid border-slate-600 hover:bg-slate-700/50 active:scale-95 font-sans">
                {{ point4Text }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="w-full space-y-4 mt-40px">
      <div class="w-full p-4 bg-slate-100/10 rounded-3xl box-border flex flex-col justify-around gap-4 shadow-md backdrop-blur-sm border border-slate-700/30 transition-all hover:bg-slate-700/20" v-for="member of teamMember" :key="member.id">
        <div class="flex items-center justify-between w-full">
          <span class="text-zinc-200 font-sans">队员 {{ member.id }}</span>
          <button @click="() => removeTeamMember(member.id)" class="h-48px w-48px aspect-square bg-transparent border-none text-slate-200 font-sans">
            <Trash class="size-full hover:bg-red-500/20 cursor-pointer rounded" />
          </button>
        </div>
        <div class="w-full flex justify-between">
          <span class="text-white">左上角</span>
          <button @click="()=>onClickTeamMemberSelect(member.id)" class="h-48px px-4 hover:text-blue-400 hover:border-blue-400 transition-all duration-200 cursor-pointer rounded-xl text-slate-200 text-xl bg-slate-800/50 border border-2px border-solid border-slate-600 hover:bg-slate-700/50 active:scale-95 font-sans">
            {{ getMemberText(member.id, 'TL') }}
          </button>
        </div>
        <div class="w-full flex justify-between">
          <span class="text-white">右下角</span>
          <button @click="()=>onClickTeamMemberSelect(member.id)" class="h-48px px-4 hover:text-blue-400 hover:border-blue-400 transition-all duration-200 cursor-pointer rounded-xl text-slate-200 text-xl bg-slate-800/50 border border-2px border-solid border-slate-600 hover:bg-slate-700/50 active:scale-95 font-sans">
            {{ getMemberText(member.id, 'BR') }}
          </button>
        </div>
        <div class="w-full flex justify-between">
          <span class="text-white">按键绑定</span>
          <button @click="()=>onClickBind(member.id)" class="h-48px px-4 hover:text-blue-400 hover:border-blue-400 transition-all duration-200 cursor-pointer rounded-xl text-slate-200 text-xl bg-slate-800/50 border border-2px border-solid border-slate-600 hover:bg-slate-700/50 active:scale-95 font-sans">
            <span v-if="member.id !== curBind && member.key === null">点击绑定</span>
            <span v-if="member.id === curBind">等待键入</span>
            <span v-if="member.id !== curBind && member.key">{{member.key}}</span>
          </button>
        </div>
      </div>
    </div>
    <div class="w-full h-fit mt-4 space-y-4">
      <button class="w-full h-48px rounded-xl bg-gradient-to-r from-blue-600 to-blue-500 text-zinc-200 border-none cursor-pointer shadow-md hover:shadow-blue-500/30 transition-all hover:scale-[1.02] active:scale-95 font-sans text-xl" @click="addTeamMember">
        添加队员
      </button>
      <button class="w-full h-48px rounded-xl bg-gradient-to-r from-green-600 to-green-500 text-zinc-200 border-none cursor-pointer shadow-md hover:shadow-green-500/30 transition-all hover:scale-[1.02] active:scale-95 font-sans text-xl" @click="testClick">
        测试点击
      </button>
    </div>
  </div></div>
</template>