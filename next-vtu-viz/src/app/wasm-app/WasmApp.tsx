"use client";

import { useEffect, useState } from "react";
import init from "./rust-vtu.js";

export const WasmApp = () => {
  const [initialised, setInitialised] = useState<boolean>(false);

  const handleInitialised = async () => {
    await init();
    setInitialised(true);
  };

  useEffect(() => {
    if (!initialised) {
      handleInitialised();
    }
  }, [initialised]);

  return <canvas id="wasm-app" width="400" height="300" />;
};
