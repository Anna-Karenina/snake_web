import init, { World, Direction } from "snake_engine";
import { random } from "./utils/random";

init().then((wasm) => {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;
  const snakeSpawnIdx = random(WORLD_WIDTH * WORLD_WIDTH);

  const gameControlBtn = document.getElementById("game-control-btn");
  const gameStatus = document.getElementById("game-status");
  const world = World.new(WORLD_WIDTH, snakeSpawnIdx);
  const worldWidth = world.width();
  const canvas = document.getElementById("snake-canvas") as HTMLCanvasElement;
  const ctx = canvas.getContext("2d");
  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  gameControlBtn.addEventListener("click", (_) => {
    const status = world.game_status();
    gameControlBtn.textContent = "Payling...";
    if (!status) {
      world.start_game();
      play();
    } else {
      console.log("must stops");
    }
  });

  document.addEventListener("keydown", (event) => {
    switch (event.code) {
      case "ArrowUp":
        world.change_snake_direction(Direction.Up);
        break;

      case "ArrowRight":
        world.change_snake_direction(Direction.Right);
        break;

      case "ArrowDown":
        world.change_snake_direction(Direction.Down);
        break;

      case "ArrowLeft":
        world.change_snake_direction(Direction.Left);
        break;

      case "Enter":
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

  const drawReward = () => {
    const idx = world.reward_cell();
    const col = idx % worldWidth;
    const row = Math.floor(idx / worldWidth);

    ctx.beginPath();
    ctx.fillStyle = "#FF0000";
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

    ctx.stroke();

    if (idx === 1000) {
      // alert("You win!");
    }
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

  const drawGameStatus = () => {
    gameStatus.textContent = world.game_status_text();
  };

  const paint = () => {
    drawWorld();
    drawSnake();
    drawReward();
    drawGameStatus();
  };

  const play = () => {
    const fps = 3;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      requestAnimationFrame(play);
    }, 1000 / fps);
  };

  paint();
});
