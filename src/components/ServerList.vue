<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function invokeMessage() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("list_databases");
}
</script>

<template>
  <!-- <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <form class="row" @submit.prevent="greet">
    <button type="submit" @click="invokeMessage">aa</button>
  </form> -->
  <div class="d-flex-column">
    <div class="d-flex">
      <v-btn class="flex-1-1 mb-2"> Button </v-btn>
    </div>

    <v-sheet rounded="lg">
      <v-list rounded="lg">
        <v-list-item
          v-for="n in 5"
          :key="n"
          link
          :title="`List Item ${n}`"
        ></v-list-item>

        <v-divider class="my-2"></v-divider>

        <v-list-item color="grey-lighten-4" link title="Refresh"></v-list-item>
      </v-list>
    </v-sheet>
  </div>
</template>
