'use client';

import {useEffect} from "react";
import init from "./rust-vtu.js";

export const WasmApp = () => {

  useEffect(() => {
    (async () => {
      await init()
    })()
  }, []);

  return <div>WASM app here...</div>
}
