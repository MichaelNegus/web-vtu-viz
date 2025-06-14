'use client';

import {useEffect} from "react";

export const Wasm = () => {

  const init = async () => {
    console.log(JSON.stringify("about to load wasm"))
    const wasmModule = await WebAssembly.instantiateStreaming(fetch('rust-vtu.wasm'), {
      env: {
        // Optional: import functions needed by your wasm
        abort: () => console.log("abort called"),
      },
    });
    console.log(JSON.stringify("wasm loaded"))
  }

  useEffect(() => {
    (async () => {
      await init()
    })()
  }, []);

  return <div>app</div>
}
