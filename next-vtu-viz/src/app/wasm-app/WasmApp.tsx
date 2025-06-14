'use client';

import {useEffect} from "react";
import init, {import_file_path} from "./rust-vtu.js";

export type WasmAppProps = {
  inputFiles: File[]
}

export const WasmApp = (props: WasmAppProps) => {

  useEffect(() => {
    (async () => {
      await init()
      import_file_path(props.inputFiles[0].webkitRelativePath)
    })()
  }, []);

  return <canvas id="wasm-app" width="400" height="300"/>
}
