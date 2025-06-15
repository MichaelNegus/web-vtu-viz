"use client";

import { useWasm } from "@/app/providers/WasmProvider";

export const Canvas = () => {
  const { file } = useWasm();

  return (
    <canvas
      id="wasm-app"
      className={file ? "visible" : "invisible"}
      width="400"
      height="300"
    />
  );
};
