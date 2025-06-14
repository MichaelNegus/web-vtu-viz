# Theme System Guide

Your Next.js application now has a complete light/dark mode theme system set up using `next-themes` and Tailwind CSS v4.

## Features

- ✅ Light mode
- ✅ Dark mode
- ✅ System mode (follows OS preference)
- ✅ Smooth transitions
- ✅ No flash on page load
- ✅ Theme persistence

## Components

### ThemeToggle

A dropdown select component that allows users to choose between Light, Dark, and System themes.

```tsx
import { ThemeToggle } from "./components/theme-toggle";

<ThemeToggle />;
```

### ThemeToggleButton

An icon button that cycles through Light → Dark → System themes.

```tsx
import { ThemeToggleButton } from "./components/theme-toggle";

<ThemeToggleButton />;
```

## Using Theme-Aware Colors

Use these Tailwind classes for automatic theme switching:

### Backgrounds

- `bg-background` - Main page background
- `bg-card` - Card/panel backgrounds
- `bg-popover` - Popup/modal backgrounds
- `bg-primary` - Primary accent background
- `bg-secondary` - Secondary background
- `bg-muted` - Subtle background
- `bg-accent` - Accent background

### Text Colors

- `text-foreground` - Primary text
- `text-muted-foreground` - Secondary/muted text
- `text-primary-foreground` - Text on primary backgrounds
- `text-secondary-foreground` - Text on secondary backgrounds
- `text-card-foreground` - Text on card backgrounds

### Borders

- `border-border` - Standard border color
- `border-input` - Input field borders

### Interactive States

- `bg-accent hover:bg-accent/80` - Hover states
- `focus:ring-ring` - Focus rings

## Example Usage

```tsx
function MyComponent() {
  return (
    <div className="bg-card border border-border rounded-lg p-6">
      <h2 className="text-foreground text-xl font-semibold mb-4">Card Title</h2>
      <p className="text-muted-foreground mb-4">
        This text adapts to the theme automatically.
      </p>
      <button className="bg-primary text-primary-foreground px-4 py-2 rounded hover:bg-primary/90 transition-colors">
        Primary Button
      </button>
    </div>
  );
}
```

## Customizing Theme Colors

To customize the colors, edit the CSS custom properties in `src/app/globals.css`:

```css
:root {
  /* Light mode colors */
  --background: 0 0% 100%;
  --foreground: 240 10% 3.9%;
  /* ... more colors */
}

.dark {
  /* Dark mode colors */
  --background: 240 10% 3.9%;
  --foreground: 0 0% 98%;
  /* ... more colors */
}
```

Colors are defined in HSL format without the `hsl()` wrapper for compatibility with Tailwind's opacity modifiers.

## Programmatic Theme Control

```tsx
import { useTheme } from "next-themes";

function ThemeAwareComponent() {
  const { theme, setTheme } = useTheme();

  return (
    <div>
      <p>Current theme: {theme}</p>
      <button onClick={() => setTheme("light")}>Light</button>
      <button onClick={() => setTheme("dark")}>Dark</button>
      <button onClick={() => setTheme("system")}>System</button>
    </div>
  );
}
```

## Best Practices

1. **Always use semantic color classes** instead of hardcoded colors
2. **Test both themes** during development
3. **Use the system theme as default** for better UX
4. **Add smooth transitions** with `transition-colors`
5. **Handle loading states** with the `mounted` pattern shown in the components

Your theme system is now ready to use! The toggle buttons are already integrated into your sidebar and main page.
