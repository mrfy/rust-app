<script setup lang="ts">
import ServerList from "./components/ServerList.vue";
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
const links = ["Dashboard", "Messages", "Profile", "Updates"];
</script>
<template>
  <v-app id="inspire">
    <v-app-bar flat>
      <v-container class="mx-auto d-flex align-center justify-center">
        <v-avatar class="me-4" color="grey-darken-1" size="32"></v-avatar>

        <!-- <v-btn
          v-for="link in links"
          :key="link"
          :text="link"
          variant="text"
        ></v-btn> -->

        <v-spacer></v-spacer>

        <v-responsive max-width="160">
          <v-text-field
            density="compact"
            flat
            hide-details
            label="Search"
            rounded="lg"
            single-line
            variant="solo-filled"
          ></v-text-field>
        </v-responsive>
      </v-container>
    </v-app-bar>

    <v-main class="bg-grey-lighten-5">
      <v-container>
        <v-row>
          <v-col cols="2"> <ServerList /> </v-col>

          <v-col>
            <v-card elevation="3" min-height="70vh" rounded="lg"></v-card>
            <!-- <v-sheet> adsad </v-sheet> -->
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
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
