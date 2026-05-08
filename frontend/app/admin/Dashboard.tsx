"use client";

import DrawPieChart from "../components/charts/PieChart";
import GlassCard from "../components/GlassCard";

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
        <DrawPieChart
          data={courseData}
          width={300}
          height={300}
          title="Students by course"
        />
        <DrawPieChart
          data={gradeData}
          width={300}
          height={300}
          title="Grade Distribution"
        />
      </div>
    </GlassCard>
  );
}
