<template>
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-button">
      <router-link class="link-redirect" :to="home.path">
        <v-icon name="fa-home" scale="1" fill="black"/>
      </router-link>
    </div>
    <div class="titlebar-button">
      <router-link class="link-redirect" :to="map_edit.path">
        <v-icon name="fa-edit" scale="1" fill="black"/>
      </router-link>
    </div>
    <div class="titlebar-button" id="titlebar-minimize">
      <v-icon name="fa-regular-window-minimize" scale="1"/>
    </div>
    <div class="titlebar-button" id="titlebar-maximize">
      <v-icon name="fa-regular-window-maximize" scale="1"/>
    </div>
    <div class="titlebar-button" id="titlebar-close">
      <v-icon name="co-exit-to-app" scale="1"/>
    </div>

  </div>
</template>

<script lang="ts">
import {appWindow} from "@tauri-apps/api/window";
import {defineComponent} from "vue";
import {invoke} from "@tauri-apps/api";
import {home, map_edit} from "../app/Router";

export default defineComponent({
  name: "CustomBar",
  computed: {
    home() {
      return home
    },
    map_edit() {
      return map_edit
    }
  },
  mounted() {
    let titlebar_minimize = document.getElementById('titlebar-minimize');
    let titlebar_maximize = document.getElementById('titlebar-maximize');
    let titlebar_close = document.getElementById('titlebar-close');
    if (titlebar_minimize) {
      titlebar_minimize.addEventListener('click', () => {
        appWindow.minimize()
      })
    }
    if (titlebar_maximize) {
      titlebar_maximize.addEventListener('click', () => {
        appWindow.toggleMaximize()
      })
    }
    if (titlebar_close) {
      titlebar_close.addEventListener('click', async () => {
        let token = sessionStorage.getItem("token");
        if (token) {
          await invoke("logout", {"token": token}).then(() => {
            sessionStorage.removeItem("token")
            appWindow.close()
          }).catch(() => {
            console.error("Failed to logout the user")
          })
        } else {
          await appWindow.close()
        }
      })
    }
  }
})
</script>

<style scoped lang="scss">
.titlebar {
  height: 30px;
  background: #FFCC01;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;

  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
  }
}


.titlebar-button:hover {
  transform: scale(1.1);
}
</style>
