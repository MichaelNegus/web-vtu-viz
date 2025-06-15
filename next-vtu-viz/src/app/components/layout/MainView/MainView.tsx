"use client";

import { FileUpload } from "../../elements";
import { useState } from "react";
// import { readFileAsText } from "@/app/utils/fileReaders";
import { WasmApp } from "@/app/wasm-app/WasmApp";
import { read_file } from "@/app/wasm-app/rust-vtu";

export const MainView: React.FC = () => {
  const [file, setFile] = useState<File | null>(null);

  const [isReading, setIsReading] = useState(false);

  const handleFileSelect = async (files: File[]) => {
    // Read the file content directly
    if (files.length > 0) {
      setIsReading(true);
      try {
        const file = files[0];
        setFile(file);
        // read_file(file.name, new Uint8Array(await file.arrayBuffer()));
      } catch (error) {
        console.error("Failed to read file:", error);
      } finally {
        setIsReading(false);
      }
    }
  };

  if (file) {
    return (
      <div className="flex-1 bg-background">
        <WasmApp />
      </div>
    );
  }

  return (
    <div className="flex-1 p-8 bg-background">
      <div className="max-w-4xl mx-auto">
        <div className="mb-8">
          <h1 className="text-4xl font-bold text-foreground mb-4">
            Welcome to VTU Viz
          </h1>
          <p className="text-lg text-muted-foreground mb-6">
            Upload a VTU file to visualize it. The file content will be read
            directly in your browser.
          </p>
        </div>

        <div className="grid grid-cols-1 gap-8">
          {/* File Upload */}
          <div className="space-y-4">
            <h2 className="text-2xl font-semibold text-foreground">
              Upload VTU File
            </h2>
            <FileUpload
              onFileSelect={handleFileSelect}
              accept=".vtu"
              multiple={false}
              maxSize={5 * 1024 * 1024} // 5MB
            />
          </div>

          {/* File Reading Status */}
          {isReading && (
            <div className="box-border flex items-center justify-center p-8 border border-border rounded-md">
              <div className="text-center">
                <div className="animate-spin w-8 h-8 border-2 border-primary border-t-transparent rounded-full mx-auto mb-2"></div>
                <p className="text-muted-foreground">Reading file content...</p>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
