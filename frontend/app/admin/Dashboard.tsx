"use client";

import DrawPieChart from "../components/charts/PieChart";
import GlassCard from "../components/GlassCard";
import { chartData } from "../lib/models";

export interface DashboardData {
  studentsVerified: chartData[];
  studentInterests: chartData[];
  studentCourses: chartData[];
  projectStack: chartData[];
  studentsWithProject: chartData[];
}
interface DashboardProps {
  data: DashboardData;
}

export default function Dashboard({ data }: DashboardProps) {
  return (
    <GlassCard className="p-8">
      <h1 className="text-2xl font-bold text-light mb-1">Dashboard</h1>
      <p className="text-sm text-secondary/50 mb-8">
        View current data about students
      </p>

      <div className="flex flex-wrap gap-8 justify-center">
        <DrawPieChart
          data={data.studentsVerified}
          width={300}
          height={300}
          title="Students Verified"
        />
        <DrawPieChart
          data={data.studentInterests}
          width={300}
          height={300}
          title="Student Interests"
        />
        <DrawPieChart
          data={data.studentCourses}
          width={300}
          height={300}
          title="Student Courses"
        />
        <DrawPieChart
          data={data.projectStack}
          width={300}
          height={300}
          title="Project Stacks"
        />
        <DrawPieChart
          data={data.studentsWithProject}
          width={300}
          height={300}
          title="Students With Atleast One Project"
        />
      </div>
    </GlassCard>
  );
}
