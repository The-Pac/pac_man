export default class Cell {
  orientation: Array<string>
  x: number
  y: number
  block_group: BlockGroup


  constructor(orientation: Array<string>, x: number, y: number, block_group: BlockGroup) {
    this.orientation = orientation;
    this.x = x;
    this.y = y;
    this.block_group = block_group;
  }

}

export enum BlockGroup {
  VOID,
  WALL,
  DOT,
  SUPERDOT,
  OBJECT,
}