"use client";

import { useTheme } from "next-themes";
import { useEffect, useState } from "react";

export function ThemeToggle() {
  const [mounted, setMounted] = useState(false);
  const { theme, setTheme } = useTheme();

  const toggleTheme = () => {
    if (theme === "light") {
      setTheme("dark");
    } else if (theme === "dark") {
      setTheme("system");
    } else {
      setTheme("light");
    }
  };

  const getThemeIcon = () => {
    switch (theme) {
      case "light":
        return "☀️";
      case "dark":
        return "🌙";
      default:
        return "💻";
    }
  };

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) {
    return (
      <button className="flex items-center justify-center w-10 h-10 rounded-md border bg-background text-foreground border-border hover:bg-accent hover:text-accent-foreground focus:outline-none focus:ring-2 focus:ring-ring transition-colors">
        <span className="text-lg">{"💻"}</span>
      </button>
    );
  }

  return (
    <button
      onClick={toggleTheme}
      className="flex items-center justify-center w-10 h-10 rounded-md border bg-background text-foreground border-border hover:bg-accent hover:text-accent-foreground focus:outline-none focus:ring-2 focus:ring-ring transition-colors"
      title={`Current theme: ${theme}. Click to cycle through themes.`}
    >
      <span className="text-lg">{getThemeIcon()}</span>
    </button>
  );
}
