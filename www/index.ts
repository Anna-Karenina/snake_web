import init, { World, Direction } from "snake_engine";

init().then((wasm) => {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;
  const snakeSpawnIdx = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, snakeSpawnIdx);
  const worldWidth = world.width();
  const canvas = document.getElementById("snake-canvas") as HTMLCanvasElement;
  const ctx = canvas.getContext("2d");
  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  document.addEventListener("keydown", (event) => {
    switch (event.code) {
      case "ArrowUp":
        world.change_snake_dir(Direction.Up);
        break;

      case "ArrowRight":
        world.change_snake_dir(Direction.Right);

        break;

      case "ArrowDown":
        world.change_snake_dir(Direction.Down);

        break;

      case "ArrowLeft":
        world.change_snake_dir(Direction.Left);

        break;
    }
  });

  const drawWorld = () => {
    ctx.beginPath();

    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }

    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
    }

    ctx.stroke();
  };

  const drawSnake = () => {
    const snakeCellPtr = world.snake_cell();
    const snakeLen = world.snake_length();
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      snakeCellPtr,
      snakeLen
    );
    const headColor = "#7878db";
    const bodyColor = "#333333";

    snakeCells.forEach((cellIdx, idx) => {
      const col = cellIdx % worldWidth;
      const row = Math.floor(cellIdx / worldWidth);
      ctx.fillStyle = idx === 0 ? headColor : bodyColor;
      ctx.beginPath();
      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    });

    ctx.stroke();
  };

  const paint = () => {
    drawWorld();
    drawSnake();
  };

  const update = () => {
    const fps = 3;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      requestAnimationFrame(update);
    }, 1000 / fps);
  };

  paint();
  update();
});
