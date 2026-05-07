"use client";

import GlassCard from "../components/GlassCard";
import { PieChart, Pie, Tooltip, Legend, ResponsiveContainer } from "recharts";

const withColors = (data: { name: string; value: number }[]) =>
  data.map((item, i) => ({
    ...item,
    fill: `hsl(${(i * 360) / data.length}, 65%, 55%)`,
  }));

const courseData = [
  { name: "Computer Science", value: 42 },
  { name: "Mathematics", value: 28 },
  { name: "Physics", value: 19 },
  { name: "Engineering", value: 35 },
  { name: "Biology", value: 14 },
];

const gradeData = [
  { name: "A", value: 38 },
  { name: "B", value: 45 },
  { name: "C", value: 27 },
  { name: "D", value: 12 },
  { name: "F", value: 6 },
];

export default function Dashboard() {
  return (
    <GlassCard className="p-8">
      <h1 className="text-2xl font-bold text-light mb-1">Dashboard</h1>
      <p className="text-sm text-secondary/50 mb-8">
        View current data about students
      </p>

      <div className="flex flex-wrap gap-8 justify-center">
        <div className="flex flex-col items-center bg-white/5 border border-white/10 rounded-2xl p-6 shadow-lg">
          <h2 className="text-sm font-semibold text-secondary/70 mb-3">
            Students by Course
          </h2>
          <ResponsiveContainer width={300} height={300}>
            <PieChart>
              <Pie
                data={withColors(courseData)}
                dataKey="value"
                cx="50%"
                cy="50%"
                outerRadius={100}
              />
              <Tooltip />
              <Legend
                iconSize={10}
                wrapperStyle={{
                  whiteSpace: "nowrap",
                  overflow: "scroll",
                }}
              />
            </PieChart>
          </ResponsiveContainer>
        </div>

        <div className="flex flex-col items-center bg-white/5 border border-white/10 rounded-2xl p-6 shadow-lg">
          <h2 className="text-sm font-semibold text-secondary/70 mb-3">
            Grade Distribution
          </h2>
          <ResponsiveContainer width={300} height={300}>
            <PieChart>
              <Pie
                data={withColors(gradeData)}
                dataKey="value"
                cx="50%"
                cy="50%"
                outerRadius={100}
              />
              <Tooltip />
              <Legend
                iconSize={10}
                wrapperStyle={{
                  whiteSpace: "nowrap",
                  overflow: "scroll",
                }}
              />
            </PieChart>
          </ResponsiveContainer>
        </div>
      </div>
    </GlassCard>
  );
}
