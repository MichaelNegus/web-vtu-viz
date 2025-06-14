"use client";

import { FileUpload } from "../../elements";

export const MainView: React.FC = () => {
  const handleFileSelect = (files: File[]) => {
    console.log("Selected files:", files);
    // Here you would typically upload the files to your server
  };

  return (
    <div className="flex-1 p-8 bg-background">
      <div className="max-w-4xl mx-auto">
        <div className="mb-8">
          <h1 className="text-4xl font-bold text-foreground mb-4">
            Welcome to VTU Viz
          </h1>
          <p className="text-lg text-muted-foreground mb-6">
            This will be a really cool app. I promise.
          </p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Single File Upload */}
          <div className="space-y-4">
            <h2 className="text-2xl font-semibold text-foreground">
              Single File Upload
            </h2>
            <FileUpload
              onFileSelect={handleFileSelect}
              accept="vtu/*"
              multiple={false}
              maxSize={5 * 1024 * 1024} // 5MB
            />
          </div>
        </div>
      </div>
    </div>
  );
};
