<template>
  <nav class="nav-menu">
    <div class="nav-item">
      <div class="link-container">
        <router-link class="link-redirect box-with-shadow" to="/utopac-home">
          <v-icon name="co-media-play" scale="3"/>
        </router-link>
      </div>
    </div>
    <div class="nav-item">
      <div class="link-container">
        <router-link class="link-redirect box-with-shadow" to="/friends">
          <v-icon name="bi-people-fill" scale="3"/>
        </router-link>
      </div>
    </div>
    <div class="nav-item">
      <div class="link-container">
        <router-link class="link-redirect box-with-shadow" @click="onLogout" to="/">
          <v-icon name="co-exit-to-app" scale="3"/>
        </router-link>
      </div>
    </div>
  </nav>
</template>

<script lang="ts">

import {defineComponent} from "vue";
import {invoke} from "@tauri-apps/api";
import router, {authentication} from "../app/Router";

export default defineComponent({
  name: "Main-Navigation-Component",
  methods: {
    onLogout() {
      let token = sessionStorage.getItem("token")
      if (token) {
        invoke("logout", {"token": token}).then(() => {
          router.push(authentication)
        })
      } else {
        console.error("No token found")
      }
    }
  }
})
</script>

<style scoped lang="scss">

.nav-menu {
  display: flex;
  justify-content: center;
  align-items: center;

  .nav-item {
    display: flex;
    justify-content: center;
    align-items: center;

    .link-container {
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;

      .link-redirect {
        display: flex;
        align-items: center;
        justify-content: center;
        color: white;
        margin: 20px;
        width: 70px;
        height: 70px;
        padding: 0;
        border-radius: 50%;
        border: 5px solid rgb(255, 204, 0);
        background: linear-gradient(180deg, #1c2026, rgb(255, 204, 0));
        background-size: 800% 800%;

      }

      .link-redirect:hover {
        cursor: pointer;
        animation: gradiant 2s ease;
        transform: scale(1.1);
        transition-duration: 500ms;
        background-position: 50% 100%
      }

    }
  }
}
</style>
