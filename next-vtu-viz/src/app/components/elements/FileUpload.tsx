"use client";

import { useCallback, useState } from "react";

interface FileUploadProps {
  onFileSelect?: (files: File[]) => void;
  accept?: string;
  multiple?: boolean;
  maxSize?: number; // in bytes
  className?: string;
  disabled?: boolean;
}

interface FileWithPreview extends File {
  preview?: string;
}

export function FileUpload({
  onFileSelect,
  accept = "*/*",
  multiple = false,
  maxSize = 10 * 1024 * 1024, // 10MB default
  className = "",
  disabled = false,
}: FileUploadProps) {
  const [isDragOver, setIsDragOver] = useState(false);
  const [uploadedFiles, setUploadedFiles] = useState<FileWithPreview[]>([]);
  const [error, setError] = useState<string>("");

  const validateFile = (file: File): string | null => {
    if (file.size > maxSize) {
      return `File ${file.name} is too large. Maximum size is ${Math.round(
        maxSize / 1024 / 1024
      )}MB`;
    }
    return null;
  };

  const handleFiles = useCallback(
    (files: FileList | null) => {
      if (!files || disabled) return;

      const fileArray = Array.from(files);
      const validFiles: FileWithPreview[] = [];
      let errorMessage = "";

      fileArray.forEach((file) => {
        const validation = validateFile(file);
        if (validation) {
          errorMessage = validation;
        } else {
          // Create preview for images
          const fileWithPreview: FileWithPreview = file;
          if (file.type.startsWith("image/")) {
            fileWithPreview.preview = URL.createObjectURL(file);
          }
          validFiles.push(fileWithPreview);
        }
      });

      if (errorMessage) {
        setError(errorMessage);
        return;
      }

      setError("");
      if (!multiple) {
        setUploadedFiles(validFiles.slice(0, 1));
        if (onFileSelect) {
          onFileSelect(validFiles.slice(0, 1));
        }
      } else {
        setUploadedFiles((prev) => [...prev, ...validFiles]);
        if (onFileSelect) {
          onFileSelect([...uploadedFiles, ...validFiles]);
        }
      }
    },
    [maxSize, multiple, onFileSelect, uploadedFiles, disabled]
  );

  const handleDragOver = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      if (!disabled) {
        setIsDragOver(true);
      }
    },
    [disabled]
  );

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
  }, []);

  const handleDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      setIsDragOver(false);
      if (!disabled) {
        handleFiles(e.dataTransfer.files);
      }
    },
    [handleFiles, disabled]
  );

  const handleFileInput = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      handleFiles(e.target.files);
    },
    [handleFiles]
  );

  const removeFile = (index: number) => {
    const newFiles = uploadedFiles.filter((_, i) => i !== index);
    setUploadedFiles(newFiles);
    if (onFileSelect) {
      onFileSelect(newFiles);
    }
  };

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  };

  return (
    <div className={`space-y-4 ${className}`}>
      {/* Upload Area */}
      <div
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        className={`
          box-border relative border-2 border-dashed rounded-lg p-8 text-center transition-colors
          ${
            isDragOver
              ? "border-primary bg-primary/5"
              : "border-border hover:border-primary/50"
          }
          ${
            disabled
              ? "opacity-50 cursor-not-allowed"
              : "cursor-pointer hover:bg-muted/50"
          }
        `}
      >
        <input
          type="file"
          accept={accept}
          multiple={multiple}
          onChange={handleFileInput}
          disabled={disabled}
          className="absolute inset-0 w-full h-full opacity-0 cursor-pointer disabled:cursor-not-allowed"
        />

        <div className="space-y-4">
          <div className="mx-auto w-12 h-12 flex items-center justify-center rounded-full bg-muted">
            <svg
              className="w-6 h-6 text-muted-foreground"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
              />
            </svg>
          </div>

          <div>
            <p className="text-foreground font-medium">
              {isDragOver
                ? "Drop files here"
                : "Click to upload or drag and drop"}
            </p>
            <p className="text-muted-foreground text-sm mt-1">
              {multiple ? "Multiple files allowed" : "Single file only"} • Max{" "}
              {Math.round(maxSize / 1024 / 1024)}MB
            </p>
          </div>
        </div>
      </div>

      {/* Error Message */}
      {error && (
        <div className="box-border p-3 rounded-md bg-destructive/10 border border-destructive/20">
          <p className="text-destructive text-sm">{error}</p>
        </div>
      )}

      {/* Uploaded Files */}
      {uploadedFiles.length > 0 && (
        <div className="space-y-2">
          <h4 className="text-sm font-medium text-foreground">
            Uploaded Files ({uploadedFiles.length})
          </h4>

          <div className="space-y-2">
            {uploadedFiles.map((file, index) => (
              <div
                key={`${file.name}-${index}`}
                className="box-border flex items-center justify-between p-3 rounded-md border border-border bg-card"
              >
                <div className="flex items-center space-x-3 flex-1 min-w-0">
                  {file.preview ? (
                    <img
                      src={file.preview}
                      alt={file.name}
                      className="w-10 h-10 object-cover rounded border border-border"
                    />
                  ) : (
                    <div className="w-10 h-10 flex items-center justify-center rounded bg-muted">
                      <svg
                        className="w-5 h-5 text-muted-foreground"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                        />
                      </svg>
                    </div>
                  )}

                  <div className="flex-1 min-w-0">
                    <p className="text-sm font-medium text-foreground truncate">
                      {file.name}
                    </p>
                    <p className="text-xs text-muted-foreground">
                      {formatFileSize(file.size)}
                    </p>
                  </div>
                </div>

                <button
                  onClick={() => removeFile(index)}
                  className="box-border ml-2 p-1 rounded-md hover:bg-destructive/10 text-muted-foreground hover:text-destructive transition-colors"
                  title="Remove file"
                >
                  <svg
                    className="w-4 h-4"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M6 18L18 6M6 6l12 12"
                    />
                  </svg>
                </button>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
