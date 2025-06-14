'use client';

import {useEffect} from "react";
import init from "./rust-vtu.js";

export const WasmApp = () => {

  useEffect(() => {
    (async () => {
      await init()
    })()
  }, []);

  return <canvas id="wasm-app" width="400" height="300"/>
}
