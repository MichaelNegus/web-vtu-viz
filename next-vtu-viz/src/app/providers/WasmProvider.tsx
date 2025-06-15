"use client";

import { createContext, useContext, useEffect, useState } from "react";
import init from "@/app/wasm-app/rust-vtu.js";

export const WasmContext = createContext<{
  initialised: boolean;
  file: File | null;
  setFile: (file: File) => void;
}>({
  initialised: false,
  file: null,
  setFile: () => {},
});

export const WasmProvider = ({ children }: { children: React.ReactNode }) => {
  const [initialised, setInitialised] = useState<boolean>(false);
  const [file, setFile] = useState<File | null>(null);

  useEffect(() => {
    init();
    setInitialised(true);
  }, []);

  return (
    <WasmContext.Provider value={{ initialised, file, setFile }}>
      {children}
    </WasmContext.Provider>
  );
};

export const useWasm = () => {
  return useContext(WasmContext);
};
