import { ThemeToggle } from "../../elements";

export const Header = () => {
  return (
    <div className="flex items-center justify-end border-b border-border p-4">
      <ThemeToggle />
    </div>
  );
};
