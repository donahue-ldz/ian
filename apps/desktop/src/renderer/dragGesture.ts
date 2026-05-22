type Point = {
  x: number;
  y: number;
};

const DRAG_THRESHOLD_PX = 8;

export function getDragOffset(start: Point, current: Point): Point {
  return {
    x: current.x - start.x,
    y: current.y - start.y,
  };
}

export function shouldStartDrag(start: Point, current: Point): boolean {
  const offset = getDragOffset(start, current);
  return Math.hypot(offset.x, offset.y) >= DRAG_THRESHOLD_PX;
}
