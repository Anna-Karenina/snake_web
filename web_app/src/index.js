import { createRoot } from "react-dom/client";
import { App, Dashboard } from "./App";
import React from "react";
import { Game } from "./presentation/game/Game";
import "the-new-css-reset/css/reset.css";
import {
  createBrowserRouter,
  createRoutesFromElements,
  Route,
  RouterProvider,
} from "react-router-dom";

const router = createBrowserRouter(
  createRoutesFromElements(
    <Route path="/" element={<App />}>
      <Route path="dashboard" element={<Dashboard />} />
      <Route path="game" element={<Game />} />
      {/* ... etc. */}
    </Route>
  )
);

createRoot(document.getElementById("app")).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
