import React, { useEffect, useRef, useState } from "react";
import init, { GameStatus, InitOutput, World } from "snake_engine";
import { random } from "../../utils/random";
import { useKeyEvents } from "../../utils/useKeyEvent";
import { useWasm } from "../../utils/useWasm";

export function Canvas() {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;
  const snakeSpawnIdx = random(WORLD_WIDTH * WORLD_WIDTH);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [wasm, wasmReady] = useWasm();
  const [world, setWorld] = useState(null as unknown as World);
  const [worldWidth, setWorldWidth] = useState(0);
  const [buttonActionText, setButtonActionText] = useState("Play");
  const key = useKeyEvents();

  useEffect(() => {
    if (!wasmReady) return;
    const world = World.new(WORLD_WIDTH, snakeSpawnIdx);
    const worldWidth = world.width();
    setWorldWidth(worldWidth);
    setWorld(world);
  }, [wasmReady]);

  const drawWorld = (ctx) => {
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

  const drawReward = (ctx) => {
    const idx = world.reward_cell();
    const col = idx! % worldWidth;
    const row = Math.floor(idx! / worldWidth);

    ctx.beginPath();
    ctx.fillStyle = "#FF0000";
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

    ctx.stroke();
  };

  const drawSnake = (ctx) => {
    const snakeCellPtr = world.snake_cell();
    const snakeLen = world.snake_length();

    const snakeCells = new Uint32Array(
      //@ts-ignore
      wasm.memory.buffer,
      snakeCellPtr,
      snakeLen
    );
    const headColor = "#7878db";
    const bodyColor = "#333333";

    snakeCells
      .filter((cellidx, idx) => !(idx > 0 && cellidx === snakeCells[0]))
      .forEach((cellIdx, idx) => {
        const col = cellIdx % worldWidth;
        const row = Math.floor(cellIdx / worldWidth);
        ctx.fillStyle = idx === 0 ? headColor : bodyColor;
        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
      });

    ctx.stroke();
  };

  const paint = (context, canvas) => {
    drawWorld(context);
    drawSnake(context);
    drawReward(context);
  };

  useEffect(() => {
    if (!wasmReady) return;
    const canvas = canvasRef.current;
    const context = canvas!.getContext("2d");
    canvas!.height = worldWidth * CELL_SIZE;
    canvas!.width = worldWidth * CELL_SIZE;

    paint(context, canvas);
  }, [world]);

  useEffect(() => {
    console.log(key);
  }, [key]);

  function startGame(): void {
    const status = world.game_status();
    if (status === undefined) {
      setButtonActionText("Payling...");
      world.start_game();
      // play();
    }
  }

  return (
    <>
      <div style={{ display: "flex", color: "white" }}>
        {wasmReady ? "wasmReady" : "!wasmReady"}
      </div>
      <button style={{ color: "white" }} onClick={() => startGame()}>
        {buttonActionText}
      </button>
      <canvas ref={canvasRef} />
    </>
  );
}
