<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import { invoke } from "@tauri-apps/api";

async function invokeMessage() {
  invoke("my_custom_command", {
    number: 42,
  })
    .then((res) =>
      console.log(`Message: ${res.message}, Other Val: ${res.other_val}`)
    )
    .catch((e) => console.error(e));
}
</script>

<template>
  <div class="container">
    <h1>Welcome to Taasdasi!</h1>
    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
    </div>
    <div class="main-c">
      <button @click="invokeMessage">Clicke Me</button>
    </div>

    <Greet />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.main-c {
  padding: 2rem;
}
</style>
