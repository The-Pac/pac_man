<template>
  <div id="pac-man">
    <div id="score-container">
      <div class="item">
        <h2>1UP</h2>
        <h3>{{ lifes }}</h3>
      </div>
      <div class="item">
        <h2>HIGH SCORE</h2>
        <h3>{{ scores }}</h3>
      </div>

    </div>
    <div id="map-container">

    </div>
    <div id="items-container">
      <div id="lifes-container">
        <img class="pac-man-lifes" v-for="(life,index) in lifes" src="src/assets/img/pacman-open-mouth.svg" :key="index"
             alt="pac-man life"/>
      </div>
      <div id="objects-container">

      </div>
    </div>
  </div>

</template>

<script lang="ts">
import {defineComponent} from "vue";
import {invoke} from "@tauri-apps/api";
import Cell from "../app/models/Cell";

export default defineComponent({
  name: "PacMan",
  data() {
    return {
      scores: Number(0),
      lifes: Number(3),
      objects: Number(0),
      map_cells: Array<Cell>()
    }
  },
  mounted() {
    invoke('get_map').then((response: any) => {
      this.map_cells = response
    }).catch(() => {
      console.error("Failed to get the map")
    })
  }
})
</script>

<style scoped lang="scss">
#pac-man {
  height: 100%;
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;

  #score-container {
    width: 100%;
    height: 10%;
    display: flex;
    justify-content: space-around;
    align-items: center;

    .item {
      width: 100%;
      height: 100%;
      display: flex;
      flex-direction: column;
      align-items: center;
    }
  }

  #map-container {
    width: 100%;
    height: 70%;
    display: flex;
    justify-content: space-around;
    align-items: center;
  }

  #items-container {
    width: 100%;
    height: 10%;
    display: flex;
    justify-content: space-around;
    align-items: center;

    #lifes-container {
      height: 100%;
      width: 20%;
      display: flex;
      justify-content: center;
      align-items: center;

      .pac-man-lifes {
        height: 50%;
      }
    }

    #objects-container {
      height: 100%;
      width: 70%;
      display: flex;
      justify-content: center;
      align-items: center;

      .pac-man-lifes {
        height: 50%;
      }
    }
  }
}

</style>