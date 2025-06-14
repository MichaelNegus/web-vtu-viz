"use client";

import { useEffect } from "react";
import init, { read_file } from "./rust-vtu.js";

export type WasmAppProps = {
  name: string;
  inputData: ArrayBuffer;
};

export const WasmApp = (props: WasmAppProps) => {
  useEffect(() => {
    (async () => {
      await init();
      read_file(props.name, new Uint8Array(props.inputData));
    })();
  }, []);

  return <canvas id="wasm-app" width="400" height="300" />;
};
