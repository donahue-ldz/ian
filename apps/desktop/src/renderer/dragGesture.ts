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

export function getPhysicalDragOffset(
  start: Point,
  current: Point,
  devicePixelRatio: number,
): Point {
  const offset = getDragOffset(start, current);
  const scale = Number.isFinite(devicePixelRatio) && devicePixelRatio > 0
    ? devicePixelRatio
    : 1;

  return {
    x: offset.x * scale,
    y: offset.y * scale,
  };
}

export function shouldStartDrag(start: Point, current: Point): boolean {
  const offset = getDragOffset(start, current);
  return Math.hypot(offset.x, offset.y) >= DRAG_THRESHOLD_PX;
}
