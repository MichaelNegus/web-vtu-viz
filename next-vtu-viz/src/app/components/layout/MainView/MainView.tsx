"use client";

import { FileUpload } from "../../elements";
import { useState } from "react";
import { readFileAsText, parseVTUFile } from "@/app/utils/fileReaders";

export const MainView: React.FC = () => {
  const [fileContent, setFileContent] = useState<string | null>(null);
  const [vtuInfo, setVtuInfo] = useState<{
    content: string;
    isXML: boolean;
    hasPoints: boolean;
    hasCells: boolean;
    pointCount?: number;
    cellCount?: number;
  } | null>(null);
  const [isReading, setIsReading] = useState(false);

  const handleFileSelect = async (files: File[]) => {
    // Read the file content directly
    if (files.length > 0) {
      setIsReading(true);
      try {
        const file = files[0];

        // Read as text
        const content = await readFileAsText(file);
        setFileContent(content.content as string);

        console.log("File content:", content);

        // If it's a VTU file, parse it
        if (file.name.toLowerCase().endsWith(".vtu")) {
          const vtuData = await parseVTUFile(file);
          setVtuInfo(vtuData);
        }
      } catch (error) {
        console.error("Failed to read file:", error);
      } finally {
        setIsReading(false);
      }
    }
  };

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

          {/* VTU File Info */}
          {vtuInfo && (
            <div className="space-y-4">
              <h3 className="text-xl font-semibold text-foreground">
                VTU File Analysis
              </h3>
              <div className="box-border grid grid-cols-2 md:grid-cols-4 gap-4 p-4 bg-card border border-border rounded-md">
                <div className="text-center">
                  <div className="text-2xl font-bold text-foreground">
                    {vtuInfo.isXML ? "XML" : "Binary"}
                  </div>
                  <div className="text-sm text-muted-foreground">Format</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold text-foreground">
                    {vtuInfo.pointCount || "Unknown"}
                  </div>
                  <div className="text-sm text-muted-foreground">Points</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold text-foreground">
                    {vtuInfo.cellCount || "Unknown"}
                  </div>
                  <div className="text-sm text-muted-foreground">Cells</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold text-foreground">
                    {vtuInfo.hasPoints && vtuInfo.hasCells ? "✅" : "❌"}
                  </div>
                  <div className="text-sm text-muted-foreground">Valid</div>
                </div>
              </div>
            </div>
          )}

          {/* File Content Preview */}
          {fileContent && (
            <div className="space-y-4">
              <h3 className="text-xl font-semibold text-foreground">
                File Content Preview
              </h3>
              <details className="box-border border border-border rounded-md">
                <summary className="px-4 py-2 bg-muted cursor-pointer hover:bg-accent transition-colors">
                  View Raw Content
                </summary>
                <pre className="p-4 text-xs overflow-auto max-h-64 text-muted-foreground bg-background">
                  {fileContent}
                </pre>
              </details>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
