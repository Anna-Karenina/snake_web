import React from "react";
import { Link, Outlet } from "react-router-dom";
import * as classes from "./App.module.css";

export function App() {
  return (
    <main className={classes.main}>
      <nav>
        <Link to="dashboard">dashboard</Link>
        <Link to="game">Game</Link>
      </nav>
      <section>
        <Outlet />
      </section>
    </main>
  );
}

export function Dashboard() {
  return <h2> Dashboard </h2>;
}
