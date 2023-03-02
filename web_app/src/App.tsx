import React from "react";
import { Link, Outlet } from "react-router-dom";
import * as classes from "./App.module.css";

export function App() {
  return (
    <main className={classes.main}>
      <h1>Hello world!</h1>
      <Link to="dashboard">dashboard</Link>
      <Link to="game">Game</Link>
      <Outlet />
    </main>
  );
}

export function Dashboard() {
  return <h2> Dashboard </h2>;
}
