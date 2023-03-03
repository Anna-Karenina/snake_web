import { useEffect, useState } from "react";
import init, { InitOutput } from "snake_engine";

export function useWasm() {
  const [wasm, setWasm] = useState(null as unknown as InitOutput);
  const [wasmReady, setWasmReady] = useState(false);

  useEffect(() => {
    if (wasmReady) return;

    const initWasm = async () => {
      const w = await init();

      setWasm(w);
      setWasmReady(true);
    };

    initWasm();
  }, []);

  return [wasm, wasmReady];
}
