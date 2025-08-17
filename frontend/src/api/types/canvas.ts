export interface CanvasBlockPlacement {
  id: string;
  blockId: string;
  x: number;
  y: number;
  width: number;
  height: number;
  collapsed: boolean;
}

export interface Canvas {
  id: string;
  name: string;
  blockPlacements: CanvasBlockPlacement[];
}
