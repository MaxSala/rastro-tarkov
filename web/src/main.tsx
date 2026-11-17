import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./app/App";
import { connect } from "./lib/socket";
import { applyTheme } from "./lib/theme";
import { lang } from "./locales";
import "./index.css";
import "./map.css";
import "./splash.css";

document.documentElement.lang = lang;
applyTheme();
connect();

const root = document.getElementById("root");
if (root) {
  createRoot(root).render(
    <StrictMode>
      <App />
    </StrictMode>,
  );
}
