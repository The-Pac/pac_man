<template>
  <div id="map-edit">
    <div id="map-container">
      <Map v-model:map_cells="map_cells" :map_cells="map_cells"/>
    </div>
  </div>

</template>

<script lang="ts">
import {defineComponent} from "vue";
import Cell from "../app/models/Cell";
import Map from "../components/Map.vue";
import {invoke} from "@tauri-apps/api";


export default defineComponent({
  name: "MapEdit",
  components: {Map},
  data() {
    return {
      map_cells: Array<Cell>()
    }
  },
  mounted() {
    invoke('new_map').then((response: any) => {
      this.map_cells = response
      console.log(response)
    }).catch(() => {

    })
  }
})
</script>

<style scoped lang="scss">
#map-edit {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  width: 100%;

  #map-container {
    width: 70%;
    height: 70%;
    display: flex;
    justify-content: center;
    align-items: center;
  }
}


</style>