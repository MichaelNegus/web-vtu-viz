"use client";

import { createContext, useContext, useEffect, useState } from "react";
import init from "@/app/wasm-app/rust-vtu.js";

export const WasmContext = createContext<{
  initialised: boolean;
  fileParsed: boolean;
  parseFile: (file: File) => void;
}>({
  initialised: false,
  fileParsed: false,
  parseFile: () => {},
});

export const WasmProvider = ({ children }: { children: React.ReactNode }) => {
  const [initialised, setInitialised] = useState<boolean>(false);
  const [fileParsed, setFileParsed] = useState<boolean>(false);

  const parseFile = (file: File) => {
    console.log(file);
    // TODO: Parse the file
    setFileParsed(true);
  };

  useEffect(() => {
    init();
    setInitialised(true);
  }, []);

  return (
    <WasmContext.Provider value={{ initialised, fileParsed, parseFile }}>
      {children}
    </WasmContext.Provider>
  );
};

export const useWasm = () => {
  return useContext(WasmContext);
};
