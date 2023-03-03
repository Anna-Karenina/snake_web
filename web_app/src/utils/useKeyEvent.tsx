import { useEffect, useState } from "react";

export function useKeyEvents() {
  const [key, setKey] = useState("");

  useEffect(() => {
    window.addEventListener("keydown", (event) => {
      switch (event.code) {
        case "ArrowUp":
          setKey("ArrowUp");
          break;

        case "ArrowRight":
          setKey("ArrowRight");
          break;

        case "ArrowDown":
          setKey("ArrowDown");
          break;

        case "ArrowLeft":
          setKey("ArrowLeft");
          break;

        case "Enter":
          break;
      }
    });
  }, []);

  return key;
}
