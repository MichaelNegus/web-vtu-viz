"use client";

import { useEffect } from "react";
import init from "./rust-vtu.js";

export type WasmAppProps = {
  inputFiles: File[];
};

export const WasmApp: React.FC<WasmAppProps> = (props) => {
  useEffect(() => {
    (async () => {
      await init();
      // import_file_path(props.inputFiles[0].webkitRelativePath);
    })();
  }, [props.inputFiles]);

  return <canvas id="wasm-app" width="400" height="300" />;
};
