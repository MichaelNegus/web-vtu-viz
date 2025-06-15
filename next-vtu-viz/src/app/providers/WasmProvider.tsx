"use client";

import { createContext, useContext, useEffect, useState } from "react";
import init from "@/app/wasm-app/rust-vtu.js";

export const WasmContext = createContext<{
  initialised: boolean;
  fileParsed: boolean;
  parseFile: (file: File) => void;
  closeFile: () => void;
  openFile: (file: File) => void;
  isOpen: boolean;
}>({
  initialised: false,
  fileParsed: false,
  parseFile: () => {},
  closeFile: () => {},
  openFile: () => {},
  isOpen: false,
});

export const WasmProvider = ({ children }: { children: React.ReactNode }) => {
  const [initialised, setInitialised] = useState<boolean>(false);
  const [fileParsed, setFileParsed] = useState<boolean>(false);
  const [isOpen, setIsOpen] = useState<boolean>(false);

  const parseFile = (file: File) => {
    console.log(file);
    // TODO: Parse the file
    setFileParsed(true);
    setIsOpen(true);
  };

  const closeFile = () => {
    setIsOpen(false);
  };

  const openFile = (file: File) => {
    if (fileParsed) {
      setIsOpen(true);
    } else {
      parseFile(file);
    }
  };

  useEffect(() => {
    init();
    setInitialised(true);
  }, []);

  return (
    <WasmContext.Provider
      value={{
        initialised,
        fileParsed,
        parseFile,
        closeFile,
        openFile,
        isOpen,
      }}
    >
      {children}
    </WasmContext.Provider>
  );
};

export const useWasm = () => {
  return useContext(WasmContext);
};
