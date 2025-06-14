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
      console.log("Reading file", props.inputData);
      const byteArray = new Uint8Array(props.inputData);
      console.log("Byte array", byteArray);
      await init();
      read_file(props.name, byteArray);
    })();
  }, []);

  return <canvas id="wasm-app" width="400" height="300" />;
};
