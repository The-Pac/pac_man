<template>
  <div id="map-container">
    <div class="column" v-for="(rows,index_rows) in map_cells">
      <div :class="this.on_edit ? 'row-edit' : 'row'"
           v-for="(cell ,index_cell) in rows"
           @click="on_click(index_rows,index_cell)">
        <div class="block" :class="get_block(cell)">
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";
import Cell, {BlockGroup, Orientation} from "../app/models/Cell";

export default defineComponent({
  name: "Map",
  props: {
    map_cells: {
      type: Array<Array<Cell>>
    },
    on_edit: {
      type: Boolean
    },
    list_block_group: {
      type: Array<BlockGroup>
    },
    selected_block_group_index: {
      type: Number
    },
  },
  methods: {
    on_click(index_rows: number, index_cell: number): void {
      if (this.on_edit
          && this.selected_block_group_index != undefined
          && Number.isInteger(this.selected_block_group_index)
          && this.map_cells
          && this.list_block_group) {
        this.map_cells[index_rows][index_cell].block_group = this.list_block_group[this.selected_block_group_index]
      }
    },
    get_block(cell: Cell): string {
      let style_block: Array<string> = [];
      switch (cell.block_group) {
        case BlockGroup.WALL:
          style_block.push("wall")
          cell.orientations.forEach((orientation: Orientation) => {
            switch (orientation) {
              case Orientation.LEFT:
                style_block.push("left")
                break
              case Orientation.RIGHT:
                style_block.push("right")
                break
              case Orientation.BOTTOM:
                style_block.push("bottom")
                break
              case Orientation.TOP:
                style_block.push("top")
                break
            }
          })
          break;
        case BlockGroup.DOT:
          style_block.push("dot")
          break;
        case BlockGroup.SUPERDOT:
          style_block.push("super-dot")
          break;
        case BlockGroup.OBJECT:
          style_block.push("object")
          break;
      }
      return style_block.join(" ");
    }
  }
})
</script>

<style scoped lang="scss">

#map-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;

  .column {
    display: flex;

    .row, .row-edit {
      background-color: rgba(0, 0, 0, 0.5);
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-content: center;
      align-items: center;
      height: 2dvh;
      width: 2dvh;
      position: relative;

      .block {
        height: 100%;
        width: 100%;
        position: relative;
      }

      .wall {
        background: blue;

        .left {

        }

        .right {
        }

        .bottom {
        }

        .top {
        }
      }

      .dot, .super-dot {
        border-radius: 5px;
      }

      .dot {
        height: 25%;
        width: 25%;
        background-color: yellow;
      }

      .super-dot {
        height: 50%;
        width: 50%;
        background-color: orange;
      }

      .object {

      }
    }

    .row-edit {
      border: 1px solid white;
    }
  }


}
</style>