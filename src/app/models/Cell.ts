export default class Cell {
  orientations: Array<Orientation>
  block_group: BlockGroup


  constructor(orientations: Array<Orientation>, block_group: BlockGroup) {
    this.orientations = orientations;
    this.block_group = block_group;
  }

}

export enum BlockGroup {
  VOID = "VOID",
  WALL = "WALL",
  DOT = "DOT",
  SUPERDOT = "SUPERDOT",
  OBJECT = "OBJECT",
}

export enum Orientation {
  LEFT = "LEFT",
  RIGHT = "RIGHT",
  BOTTOM = "BOTTOM",
  TOP = "TOP",
}