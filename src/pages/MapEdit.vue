<template>
  <div id="map-edit">
    <div id="map-container">
      <Map v-model:map_cells="map_cells"
           :selected_block_group_index="selected_block_group_index"
           :list_block_group="list_block_group"
           :map_cells="map_cells" :on_edit="true"/>
    </div>
    <div id="buttons-block-group-container">
      <button v-for="(block_group,index_block_group) in list_block_group"
              @click="selected_block_group_index = index_block_group">
        <label>{{ block_group }}</label>
      </button>
    </div>
    <br/>
    <div>
      <button @click="save_map">
        Save
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import Cell, {BlockGroup} from "../app/models/Cell";
import router, {home} from "../app/Router";
import Map from "../components/Map.vue";
import {invoke} from "@tauri-apps/api";


export default defineComponent({
  name: "MapEdit",
  components: {Map},
  data() {
    return {
      map_cells: Array<Array<Cell>>(),
      selected_block_group_index: Number(),
      list_block_group: Array<BlockGroup>()
    }
  },
  mounted() {
    this.list_block_group.push(
        BlockGroup.DOT,
        BlockGroup.OBJECT,
        BlockGroup.WALL,
        BlockGroup.VOID,
        BlockGroup.SUPERDOT,
    )

    invoke('new_map').then((response: any) => {
      this.map_cells = response
    }).catch(() => {

    })
  },
  methods: {
    save_map() {
      invoke('save_map', {'mapCells': JSON.stringify(this.map_cells)}).then(() => {
        router.push(home).catch((error: string) => console.error(error));
      }).catch(() => {

      })
    }
  }
})
</script>

<style scoped lang="scss">
#map-edit {
  display: flex;
  flex-direction: column;
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

  #buttons-block-group-container {
    width: 70%;
    height: 5%;
    display: flex;
    justify-content: space-around;
    align-content: center;

    button {
      width: 10%;
      height: 100%;
    }
  }
}
</style>