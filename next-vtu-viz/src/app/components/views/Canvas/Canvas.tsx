"use client";

import { useWasm } from "@/app/providers/WasmProvider";
import { X } from "lucide-react";

export const Canvas = () => {
  const { closeFile, isOpen } = useWasm();

  return (
    <div className={`relative ${isOpen ? "visible" : "invisible"}`}>
      <div className="absolute top-4 right-4">
        <button
          className="p-1 rounded-md bg-background hover:bg-muted"
          onClick={closeFile}
        >
          <X size={16} />
        </button>
      </div>
      <canvas id="wasm-app" width="400" height="300" />
    </div>
  );
};
