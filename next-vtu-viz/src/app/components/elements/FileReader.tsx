"use client";

import { useState } from "react";
import { FileUpload } from "./FileUpload";
import {
  readFileAuto,
  parseVTUFile,
  FileContent,
} from "@/app/utils/fileReaders";

interface FileReaderProps {
  className?: string;
}

export function FileReader({ className = "" }: FileReaderProps) {
  const [fileContent, setFileContent] = useState<FileContent | null>(null);
  const [parsedContent, setParsedContent] = useState<unknown>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string>("");

  const handleFileSelect = async (files: File[]) => {
    if (files.length === 0) return;

    const file = files[0];
    setIsLoading(true);
    setError("");
    setParsedContent(null);

    try {
      // Read the file
      const content = await readFileAuto(file);
      setFileContent(content);

      // Try to parse specific file types
      const extension = file.name.split(".").pop()?.toLowerCase();

      if (extension === "vtu") {
        const vtuData = await parseVTUFile(file);
        setParsedContent(vtuData);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to read file");
    } finally {
      setIsLoading(false);
    }
  };

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  };

  const renderContent = () => {
    if (!fileContent || !parsedContent) return null;

    const extension = fileContent.name.split(".").pop()?.toLowerCase();

    switch (extension) {
      case "vtu":
        const vtuData = parsedContent as {
          isXML: boolean;
          hasPoints: boolean;
          hasCells: boolean;
          pointCount?: number;
          cellCount?: number;
          content: string;
        };
        return (
          <div className="space-y-4">
            <div className="box-border grid grid-cols-2 md:grid-cols-4 gap-4 p-4 bg-card border border-border rounded-md">
              <div className="text-center">
                <div className="text-2xl font-bold text-foreground">
                  {vtuData.isXML ? "XML" : "Binary"}
                </div>
                <div className="text-sm text-muted-foreground">Format</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-foreground">
                  {vtuData.pointCount || "Unknown"}
                </div>
                <div className="text-sm text-muted-foreground">Points</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-foreground">
                  {vtuData.cellCount || "Unknown"}
                </div>
                <div className="text-sm text-muted-foreground">Cells</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-foreground">
                  {vtuData.hasPoints && vtuData.hasCells ? "✅" : "❌"}
                </div>
                <div className="text-sm text-muted-foreground">Valid</div>
              </div>
            </div>

            <details className="box-border border border-border rounded-md">
              <summary className="px-4 py-2 bg-muted cursor-pointer hover:bg-accent transition-colors">
                View Raw Content (first 1000 characters)
              </summary>
              <pre className="p-4 text-xs overflow-auto max-h-64 text-muted-foreground">
                {vtuData.content.substring(0, 1000)}
                {vtuData.content.length > 1000 && "..."}
              </pre>
            </details>
          </div>
        );

      default:
        // For images, show the image
        if (fileContent.type.startsWith("image/")) {
          return (
            <img
              src={parsedContent as string}
              alt={fileContent.name}
              className="max-w-full max-h-96 rounded-md border border-border"
            />
          );
        }

        // For other text files
        return (
          <pre className="box-border bg-muted p-4 rounded-md text-sm overflow-auto max-h-96 whitespace-pre-wrap">
            {typeof parsedContent === "string"
              ? parsedContent.length > 2000
                ? parsedContent.substring(0, 2000) + "..."
                : parsedContent
              : String(parsedContent)}
          </pre>
        );
    }
  };

  return (
    <div className={`space-y-6 ${className}`}>
      <FileUpload
        onFileSelect={handleFileSelect}
        accept=".vtu,.vtk,.json,.csv,.txt,.xml,.jpg,.png,.gif"
        multiple={false}
        maxSize={50 * 1024 * 1024} // 50MB
      />

      {isLoading && (
        <div className="box-border flex items-center justify-center p-8 border border-border rounded-md">
          <div className="text-center">
            <div className="animate-spin w-8 h-8 border-2 border-primary border-t-transparent rounded-full mx-auto mb-2"></div>
            <p className="text-muted-foreground">Reading file...</p>
          </div>
        </div>
      )}

      {error && (
        <div className="box-border p-4 bg-destructive/10 border border-destructive/20 rounded-md">
          <p className="text-destructive">{error}</p>
        </div>
      )}

      {fileContent && !isLoading && !error && (
        <div className="space-y-4">
          <div className="box-border flex items-center justify-between p-4 bg-card border border-border rounded-md">
            <div>
              <h3 className="font-semibold text-foreground">
                {fileContent.name}
              </h3>
              <p className="text-sm text-muted-foreground">
                {formatFileSize(fileContent.size)} •{" "}
                {fileContent.type || "Unknown type"}
              </p>
            </div>
            <div className="text-sm text-muted-foreground">
              Modified:{" "}
              {new Date(fileContent.lastModified).toLocaleDateString()}
            </div>
          </div>

          <div className="box-border border border-border rounded-md overflow-hidden">
            <div className="px-4 py-2 bg-muted border-b border-border">
              <h4 className="font-medium text-foreground">File Content</h4>
            </div>
            <div className="p-4">{renderContent()}</div>
          </div>
        </div>
      )}
    </div>
  );
}
