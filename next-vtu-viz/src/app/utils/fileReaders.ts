// Utility functions for reading file content directly in the browser

export interface FileContent {
  name: string;
  size: number;
  type: string;
  content: string | ArrayBuffer | null;
  lastModified: number;
}

/**
 * Read a file as text (good for .txt, .json, .csv, .xml, .vtu files)
 */
export const readFileAsText = (file: File): Promise<FileContent> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();

    reader.onload = (event) => {
      console.log("File content:", event.target?.result);
      resolve({
        name: file.name,
        size: file.size,
        type: file.type,
        content: event.target?.result as string,
        lastModified: file.lastModified,
      });
    };

    reader.onerror = () => reject(reader.error);
    reader.readAsText(file);
  });
};

/**
 * Read a file as ArrayBuffer (good for binary files like .vtk, .bin, etc.)
 */
export const readFileAsArrayBuffer = (file: File): Promise<FileContent> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();

    reader.onload = (event) => {
      resolve({
        name: file.name,
        size: file.size,
        type: file.type,
        content: event.target?.result as ArrayBuffer,
        lastModified: file.lastModified,
      });
    };

    reader.onerror = () => reject(reader.error);
    reader.readAsArrayBuffer(file);
  });
};

/**
 * Read a file as Data URL (good for images, creating previews)
 */
export const readFileAsDataURL = (file: File): Promise<FileContent> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();

    reader.onload = (event) => {
      resolve({
        name: file.name,
        size: file.size,
        type: file.type,
        content: event.target?.result as string,
        lastModified: file.lastModified,
      });
    };

    reader.onerror = () => reject(reader.error);
    reader.readAsDataURL(file);
  });
};

/**
 * Auto-detect file type and read accordingly
 */
export const readFileAuto = async (file: File): Promise<FileContent> => {
  const extension = file.name.split(".").pop()?.toLowerCase();

  if (extension === "vtu") {
    return readFileAsText(file);
  }

  switch (extension) {
    case "json":
    case "txt":
    case "csv":
    case "xml":
    case "vtu":
    case "svg":
    case "html":
    case "css":
    case "js":
    case "ts":
      return readFileAsText(file);

    case "jpg":
    case "jpeg":
    case "png":
    case "gif":
    case "webp":
      return readFileAsDataURL(file);

    case "vtk":
    case "bin":
    case "dat":
      return readFileAsArrayBuffer(file);

    default:
      // Default to text for unknown files
      return readFileAsText(file);
  }
};

/**
 * Read VTU file and extract basic info
 */
export const parseVTUFile = async (
  file: File
): Promise<{
  content: string;
  isXML: boolean;
  hasPoints: boolean;
  hasCells: boolean;
  pointCount?: number;
  cellCount?: number;
}> => {
  const fileContent = await readFileAsText(file);
  const content = fileContent.content as string;

  const isXML = content.includes("<?xml") || content.includes("<VTKFile");
  const hasPoints = content.includes("<Points>") || content.includes("POINTS");
  const hasCells = content.includes("<Cells>") || content.includes("CELLS");

  // Try to extract point and cell counts
  let pointCount: number | undefined;
  let cellCount: number | undefined;

  if (isXML) {
    const pointsMatch = content.match(/NumberOfPoints="(\d+)"/);
    const cellsMatch = content.match(/NumberOfCells="(\d+)"/);
    pointCount = pointsMatch ? parseInt(pointsMatch[1]) : undefined;
    cellCount = cellsMatch ? parseInt(cellsMatch[1]) : undefined;
  }

  return {
    content,
    isXML,
    hasPoints,
    hasCells,
    pointCount,
    cellCount,
  };
};
