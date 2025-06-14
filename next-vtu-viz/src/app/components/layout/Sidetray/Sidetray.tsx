export const Sidetray = () => {
  return (
    <div className="flex flex-col gap-4 w-60 bg-card border-r border-border h-full">
      <div className="flex items-center justify-center p-4 border-b border-border">
        <h2 className="text-lg font-semibold text-foreground h-10 flex items-center justify-center">
          VTU Viz
        </h2>
      </div>

      <div className="flex flex-col gap-2 p-4">
        <div className="p-3 rounded-md bg-muted">
          <h3 className="font-medium text-foreground mb-2">Theme Demo</h3>
          <p className="text-sm text-muted-foreground">
            This sidebar automatically adapts to your selected theme!
          </p>
        </div>

        <button className="w-full p-2 text-left rounded-md bg-primary text-primary-foreground hover:bg-primary/90 transition-colors">
          Primary Button
        </button>

        <button className="w-full p-2 text-left rounded-md bg-secondary text-secondary-foreground hover:bg-secondary/80 transition-colors">
          Secondary Button
        </button>
      </div>
    </div>
  );
};
