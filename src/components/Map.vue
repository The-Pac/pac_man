<template>
  <div id="map-container">
    <div class="block" v-for="rows in map_cells">
      <div class="block" v-for="cell in rows">
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import Cell from "../app/models/Cell";

export default defineComponent({
  name: "Map",
  props: {
    map_cells: Array<Cell>
  }
})
</script>

<style scoped lang="scss">

#map-container {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;

  .block {
    height: 10px;
    width: 10px;
    border: 2px solid white;
  }

  .void {

  }

  .wall {

  }

  .dot {

  }

  .super-dot {

  }

  .object {

  }
}
</style>