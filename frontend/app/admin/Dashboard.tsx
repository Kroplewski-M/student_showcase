"use client";

import GlassCard from "../components/GlassCard";
import { PieChart, Pie, Tooltip, Legend, ResponsiveContainer } from "recharts";

const courseData = [
  { name: "Computer Science", value: 42, fill: "#6366f1" },
  { name: "Mathematics", value: 28, fill: "#8b5cf6" },
  { name: "Physics", value: 19, fill: "#a78bfa" },
  { name: "Engineering", value: 35, fill: "#c4b5fd" },
  { name: "Biology", value: 14, fill: "#ddd6fe" },
];

const gradeData = [
  { name: "A", value: 38, fill: "#6366f1" },
  { name: "B", value: 45, fill: "#8b5cf6" },
  { name: "C", value: 27, fill: "#a78bfa" },
  { name: "D", value: 12, fill: "#c4b5fd" },
  { name: "F", value: 6, fill: "#ddd6fe" },
];

export default function Dashboard() {
  return (
    <GlassCard className="p-8">
      <h1 className="text-2xl font-bold text-light mb-1">Dashboard</h1>
      <p className="text-sm text-secondary/50 mb-8">
        View current data about students
      </p>

      <div className="flex flex-wrap gap-8 justify-center">
        <div className="flex flex-col items-center">
          <h2 className="text-sm font-semibold text-secondary/70 mb-3">
            Students by Course
          </h2>
          <ResponsiveContainer width={300} height={300}>
            <PieChart>
              <Pie
                data={courseData}
                dataKey="value"
                cx="50%"
                cy="50%"
                outerRadius={100}
                label
              />
              <Tooltip />
              <Legend />
            </PieChart>
          </ResponsiveContainer>
        </div>

        <div className="flex flex-col items-center">
          <h2 className="text-sm font-semibold text-secondary/70 mb-3">
            Grade Distribution
          </h2>
          <ResponsiveContainer width={300} height={300}>
            <PieChart>
              <Pie
                data={gradeData}
                dataKey="value"
                cx="50%"
                cy="50%"
                outerRadius={100}
                label
              />
              <Tooltip />
              <Legend />
            </PieChart>
          </ResponsiveContainer>
        </div>
      </div>
    </GlassCard>
  );
}
