'use client';

import {useEffect} from "react";
import init, {read_file} from "./rust-vtu.js";

export type WasmAppProps = {
  inputFiles: File[]
}

export const WasmApp = (props: WasmAppProps) => {

  useEffect(() => {
    (async () => {
      await init()
      read_file(props.inputFiles[0].name, await props.inputFiles[0].bytes())
    })()
  }, []);

  return <canvas id="wasm-app" width="400" height="300"/>
}
