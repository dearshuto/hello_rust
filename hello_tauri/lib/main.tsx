import { useState } from "react";

type Args = {
  greetMessage: string;
  setter: (p: string) => void;
};

function MyApp(args: Args) {
  return (
    <div>
      <h1>{args.greetMessage}</h1>
      <form>
        <input
          placeholder="Enter..."
          onChange={(e) => args.setter(e.currentTarget.value)}
        />
      </form>
      <button>Button</button>
      <p>Please input</p>
    </div>
  );
}

export default MyApp;
