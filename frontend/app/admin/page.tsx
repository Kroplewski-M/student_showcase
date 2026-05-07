"use client";

import { useState } from "react";
import StudentLookup from "./StudentLookup";
import Dashboard from "./Dashboard";

enum AdminView {
  Dashboard,
  StudentLookup,
}

export default function Admin() {
  const [view, setView] = useState<AdminView>(AdminView.Dashboard);
  return (
    <main className="mx-auto max-w-2xl px-5  min-h-screen">
      <div className="flex items-center justify-center gap-2 mb-5 mt-32">
        <button
          onClick={() => setView(AdminView.Dashboard)}
          className="cursor-pointer gap-2 rounded-lg border border-secondary/20 bg-secondary/6 px-4 py-2 text-sm font-medium text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary"
        >
          Dashboard
        </button>
        <button
          onClick={() => setView(AdminView.StudentLookup)}
          className="cursor-pointer gap-2 rounded-lg border border-secondary/20 bg-secondary/6 px-4 py-2 text-sm font-medium text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary"
        >
          Student Lookup
        </button>
      </div>

      <div className="flex items-center justify-center">
        {view == AdminView.StudentLookup ? <StudentLookup /> : <Dashboard />}
      </div>
    </main>
  );
}
