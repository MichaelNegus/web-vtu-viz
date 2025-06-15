"use client";

import { useWasm } from "@/app/providers/WasmProvider";

export const Canvas = () => {
  const { fileParsed } = useWasm();

  return (
    <canvas
      id="wasm-app"
      className={fileParsed ? "visible" : "invisible"}
      width="400"
      height="300"
    />
  );
};
